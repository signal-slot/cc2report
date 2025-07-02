use crate::conversation_analyzer::{ConversationFlow, Topic};
use crate::intelligent_summary::create_intelligent_summary;
use crate::parser::{LogEntry, LogRecord};
use chrono::{DateTime, NaiveDate, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Default)]
struct SessionMetrics {
    total_cost: f64,
    total_messages: usize,
    total_sessions: std::collections::HashSet<String>,
}

impl SessionMetrics {
    fn new() -> Self {
        Self::default()
    }
}

pub async fn analyze_conversations_with_ai(
    log_dir: &Path,
    date_filter: Option<(Option<NaiveDate>, Option<NaiveDate>)>,
    api_key: &str,
    lang: &str,
    model: &str,
    quiet: bool,
    parallel: usize,
) -> Result<crate::smart_analyzer::SmartReport, Box<dyn std::error::Error>> {
    let mut projects: HashMap<String, Vec<Topic>> = HashMap::new();
    let mut session_metrics = SessionMetrics::new();

    // Count directories for progress bar
    let dir_count = std::fs::read_dir(log_dir)?.count();
    let pb = if quiet {
        ProgressBar::hidden()
    } else {
        let pb = ProgressBar::new(dir_count as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message("Scanning project directories...");
        pb
    };

    // 各プロジェクトディレクトリを処理
    for entry in std::fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let project_name = path.file_name().unwrap().to_string_lossy().to_string();
            let readable_name = project_name
                .trim_start_matches('-')
                .replace('-', "/")
                .replace("/home/", "~/");

            pb.set_message(format!("Processing: {readable_name}"));

            let mut flow = ConversationFlow::new();

            // プロジェクト内のすべてのJSONLファイルを処理
            for jsonl_entry in std::fs::read_dir(&path)? {
                let jsonl_entry = jsonl_entry?;
                let jsonl_path = jsonl_entry.path();

                if jsonl_path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                    process_conversation_file(
                        &jsonl_path,
                        &mut flow,
                        date_filter,
                        &mut session_metrics,
                    )?;
                }
            }

            flow.finalize();

            if !flow.topics.is_empty() {
                projects.insert(readable_name, flow.topics);
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Directory scan complete!");

    // AI を使用してインテリジェントサマリーを生成
    let date_str = format_date_range(date_filter);
    let intelligent_report = create_intelligent_summary(
        projects,
        date_str,
        true,
        Some(api_key),
        lang,
        model,
        quiet,
        parallel,
    )
    .await?;

    // SmartReport 形式に変換
    Ok(convert_intelligent_to_smart(
        intelligent_report,
        session_metrics,
    ))
}

fn convert_intelligent_to_smart(
    intelligent: crate::intelligent_summary::IntelligentReport,
    metrics: SessionMetrics,
) -> crate::smart_analyzer::SmartReport {
    use crate::smart_analyzer::*;

    let project_summaries: Vec<SmartProjectSummary> = intelligent
        .projects
        .into_iter()
        .map(|proj| {
            let activities = proj
                .work_summary
                .activities
                .into_iter()
                .map(|act| SmartActivity {
                    category: act.category,
                    summary: act.description,
                    item_count: 1,
                    complexity: match act.impact_level {
                        crate::intelligent_summary::ImpactLevel::High => ComplexityLevel::Complex,
                        crate::intelligent_summary::ImpactLevel::Medium => {
                            ComplexityLevel::Moderate
                        }
                        crate::intelligent_summary::ImpactLevel::Low => ComplexityLevel::Simple,
                    },
                })
                .collect();

            let outcomes = ProjectOutcomes {
                delivered_value: proj
                    .key_achievements
                    .into_iter()
                    .map(|a| a.description)
                    .collect(),
                technical_improvements: vec![],
                unresolved_issues: proj.blockers.into_iter().map(|b| b.issue).collect(),
            };

            SmartProjectSummary {
                title: proj.title,
                objective: proj.purpose,
                work_breakdown: WorkBreakdown {
                    primary_activities: activities,
                    effort_distribution: EffortMetrics {
                        development: proj
                            .work_summary
                            .time_distribution
                            .get("Development")
                            .copied()
                            .unwrap_or(0.0),
                        debugging: proj
                            .work_summary
                            .time_distribution
                            .get("Debugging")
                            .copied()
                            .unwrap_or(0.0),
                        configuration: proj
                            .work_summary
                            .time_distribution
                            .get("Configuration")
                            .copied()
                            .unwrap_or(0.0),
                        other: proj
                            .work_summary
                            .time_distribution
                            .get("Other")
                            .copied()
                            .unwrap_or(0.0),
                    },
                },
                outcomes,
            }
        })
        .collect();

    SmartReport {
        date: intelligent.date,
        summary: ExecutiveSummary {
            total_work_items: metrics.total_messages,
            completion_rate: 85.0, // Default estimate
            key_focus_areas: extract_key_focus_areas(&project_summaries),
            productivity_insights: if metrics.total_cost > 0.0 {
                format!(
                    "{} Total cost: ${:.2}, Sessions: {}",
                    intelligent.overall_insights,
                    metrics.total_cost,
                    metrics.total_sessions.len()
                )
            } else {
                format!(
                    "{} Sessions: {}, Messages: {}",
                    intelligent.overall_insights,
                    metrics.total_sessions.len(),
                    metrics.total_messages
                )
            },
        },
        projects: project_summaries,
        token_tracker: intelligent.token_tracker,
    }
}

fn extract_key_focus_areas(projects: &[crate::smart_analyzer::SmartProjectSummary]) -> Vec<String> {
    use std::collections::HashMap;
    let mut category_counts = HashMap::new();

    for project in projects {
        for activity in &project.work_breakdown.primary_activities {
            *category_counts
                .entry(activity.category.clone())
                .or_insert(0) += 1;
        }
    }

    let mut categories: Vec<_> = category_counts.into_iter().collect();
    categories.sort_by(|a, b| b.1.cmp(&a.1));
    categories.into_iter().take(3).map(|(cat, _)| cat).collect()
}

fn process_conversation_file(
    path: &Path,
    flow: &mut ConversationFlow,
    date_filter: Option<(Option<NaiveDate>, Option<NaiveDate>)>,
    metrics: &mut SessionMetrics,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<LogRecord>(&line) {
            Ok(LogRecord::Entry(entry)) => {
                if let Some(timestamp_str) = entry.timestamp.clone() {
                    if let Some(timestamp) = parse_timestamp(&timestamp_str) {
                        if !is_in_date_range(timestamp.date_naive(), date_filter) {
                            continue;
                        }

                        // Track metrics first
                        if let Some(session_id) = &entry.session_id {
                            metrics.total_sessions.insert(session_id.clone());
                        }
                        if let Some(cost) = entry.cost_usd {
                            metrics.total_cost += cost;
                        }
                        if entry.entry_type == "user" || entry.entry_type == "assistant" {
                            metrics.total_messages += 1;
                        }

                        process_conversation_entry(entry, flow, &timestamp_str);
                    }
                }
            }
            Ok(LogRecord::Summary(_)) => {
                // サマリーエントリはスキップ
            }
            Err(_) => {
                // パースできないエントリはスキップ
            }
        }
    }

    Ok(())
}

fn process_conversation_entry(entry: LogEntry, flow: &mut ConversationFlow, timestamp: &str) {
    match entry.entry_type.as_str() {
        "user" => {
            if let Some(message) = &entry.message {
                if let Some(content) = extract_message_content(message) {
                    flow.analyze_user_message(&content, timestamp);
                }
            }
        }
        "assistant" => {
            if let Some(message) = &entry.message {
                flow.analyze_assistant_response(message);
            }
        }
        _ => {}
    }
}

fn extract_message_content(message: &crate::parser::Message) -> Option<String> {
    if let Some(content) = &message.content {
        if let Some(text) = content.as_str() {
            return Some(text.to_string());
        } else if let Some(array) = content.as_array() {
            for item in array {
                if let Some(obj) = item.as_object() {
                    if obj.get("type").and_then(|v| v.as_str()) == Some("text") {
                        if let Some(text) = obj.get("content").and_then(|v| v.as_str()) {
                            return Some(text.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn parse_timestamp(timestamp: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn is_in_date_range(
    date: NaiveDate,
    filter: Option<(Option<NaiveDate>, Option<NaiveDate>)>,
) -> bool {
    if let Some((from_date, to_date)) = filter {
        match (from_date, to_date) {
            (Some(from), Some(to)) => date >= from && date <= to,
            (Some(from), None) => date >= from,
            (None, Some(to)) => date <= to,
            (None, None) => true,
        }
    } else {
        true
    }
}

fn format_date_range(date_filter: Option<(Option<NaiveDate>, Option<NaiveDate>)>) -> String {
    match date_filter {
        Some((Some(from), Some(to))) if from == to => from.format("%Y-%m-%d").to_string(),
        Some((Some(from), Some(to))) => {
            format!("{} から {}", from.format("%Y-%m-%d"), to.format("%Y-%m-%d"))
        }
        Some((Some(from), None)) => format!("{} 以降", from.format("%Y-%m-%d")),
        Some((None, Some(to))) => format!("{} まで", to.format("%Y-%m-%d")),
        _ => "全期間".to_string(),
    }
}
