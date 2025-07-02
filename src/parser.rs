#![allow(clippy::collapsible_if)]

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum LogRecord {
    Entry(LogEntry),
    Summary(SummaryEntry),
}

#[derive(Debug, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    #[serde(rename = "isSidechain")]
    pub is_sidechain: Option<bool>,
    #[serde(rename = "userType")]
    pub user_type: Option<String>,
    pub cwd: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub message: Option<Message>,
    #[serde(rename = "costUSD")]
    pub cost_usd: Option<f64>,
    #[serde(rename = "durationMs")]
    pub duration_ms: Option<u64>,
    pub uuid: Option<String>,
    pub timestamp: Option<String>,
    #[serde(rename = "toolUseResult")]
    pub tool_use_result: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct SummaryEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub summary: String,
    #[serde(rename = "leafUuid")]
    pub leaf_uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<serde_json::Value>,
    pub model: Option<String>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProjectSummary {
    pub project_path: String,
    pub sessions: usize,
    pub messages: usize,
    pub cost_usd: f64,
    pub duration_ms: u64,
    pub models: HashMap<String, usize>,
}

#[derive(Debug, Serialize)]
pub struct DailyReport {
    pub date: String,
    pub projects: Vec<ProjectSummary>,
    pub total_sessions: usize,
    pub total_messages: usize,
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub models_used: HashMap<String, usize>,
    pub tools_used: HashMap<String, usize>,
}

fn parse_timestamp(timestamp: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

pub fn analyze_logs(
    log_dir: &Path,
    date_filter: Option<(Option<NaiveDate>, Option<NaiveDate>)>,
) -> Result<DailyReport, Box<dyn std::error::Error>> {
    let mut projects: HashMap<String, ProjectSummary> = HashMap::new();
    let mut project_sessions: HashMap<String, HashSet<String>> = HashMap::new();
    let mut total_cost = 0.0;
    let mut total_duration = 0;
    let mut total_messages = 0;
    let mut all_sessions: HashSet<String> = HashSet::new();
    let mut global_models: HashMap<String, usize> = HashMap::new();
    let mut tools_used: HashMap<String, usize> = HashMap::new();

    // Traverse all project directories
    for entry in std::fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let project_name = path.file_name().unwrap().to_string_lossy().to_string();

            // Process all JSONL files in the project directory
            for jsonl_entry in std::fs::read_dir(&path)? {
                let jsonl_entry = jsonl_entry?;
                let jsonl_path = jsonl_entry.path();

                if jsonl_path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                    let file = File::open(&jsonl_path)?;
                    let reader = BufReader::new(file);

                    for line in reader.lines() {
                        let line = line?;
                        if line.trim().is_empty() {
                            continue;
                        }

                        match serde_json::from_str::<LogRecord>(&line) {
                            Ok(LogRecord::Entry(entry)) => {
                                // Parse timestamp and filter by date
                                if let Some(timestamp_str) = &entry.timestamp {
                                    if let Some(timestamp) = parse_timestamp(timestamp_str) {
                                        let entry_date = timestamp.date_naive();

                                        if let Some((from_date, to_date)) = date_filter {
                                            let in_range = match (from_date, to_date) {
                                                (Some(from), Some(to)) => {
                                                    entry_date >= from && entry_date <= to
                                                }
                                                (Some(from), None) => entry_date >= from,
                                                (None, Some(to)) => entry_date <= to,
                                                (None, None) => true,
                                            };
                                            if !in_range {
                                                continue;
                                            }
                                        }

                                        // Track sessions
                                        if let Some(session_id) = &entry.session_id {
                                            all_sessions.insert(session_id.clone());
                                            project_sessions
                                                .entry(project_name.clone())
                                                .or_default()
                                                .insert(session_id.clone());
                                        }

                                        // Initialize project summary if needed
                                        let project_summary = projects
                                            .entry(project_name.clone())
                                            .or_insert_with(|| ProjectSummary {
                                                project_path: project_name.clone(),
                                                sessions: 0,
                                                messages: 0,
                                                cost_usd: 0.0,
                                                duration_ms: 0,
                                                models: HashMap::new(),
                                            });

                                        // Count messages
                                        if entry.entry_type == "user"
                                            || entry.entry_type == "assistant"
                                        {
                                            total_messages += 1;
                                            project_summary.messages += 1;
                                        }

                                        // Track costs
                                        if let Some(cost) = entry.cost_usd {
                                            total_cost += cost;
                                            project_summary.cost_usd += cost;
                                        }

                                        // Track duration
                                        if let Some(duration) = entry.duration_ms {
                                            total_duration += duration;
                                            project_summary.duration_ms += duration;
                                        }

                                        // Track models
                                        if let Some(message) = &entry.message {
                                            if let Some(model) = &message.model {
                                                *global_models.entry(model.clone()).or_default() +=
                                                    1;
                                                *project_summary
                                                    .models
                                                    .entry(model.clone())
                                                    .or_default() += 1;
                                            }
                                        }

                                        // Track tool usage
                                        if entry.entry_type == "assistant" {
                                            if let Some(message) = &entry.message {
                                                if let Some(content) = &message.content {
                                                    if let Some(array) = content.as_array() {
                                                        for item in array {
                                                            if let Some(obj) = item.as_object() {
                                                                if obj
                                                                    .get("type")
                                                                    .and_then(|v| v.as_str())
                                                                    == Some("tool_use")
                                                                {
                                                                    if let Some(tool_name) = obj
                                                                        .get("name")
                                                                        .and_then(|v| v.as_str())
                                                                    {
                                                                        *tools_used
                                                                            .entry(
                                                                                tool_name
                                                                                    .to_string(),
                                                                            )
                                                                            .or_default() += 1;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(LogRecord::Summary(_)) => {
                                // Skip summary entries for now
                            }
                            Err(e) => {
                                eprintln!("Error parsing JSON: {e} in line: {line}");
                            }
                        }
                    }
                }
            }
        }
    }

    // Update session counts for each project
    for (project_name, summary) in projects.iter_mut() {
        if let Some(sessions) = project_sessions.get(project_name) {
            summary.sessions = sessions.len();
        }
    }

    let date_str = match date_filter {
        Some((Some(from), Some(to))) if from == to => from.format("%Y-%m-%d").to_string(),
        Some((Some(from), Some(to))) => {
            format!("{} to {}", from.format("%Y-%m-%d"), to.format("%Y-%m-%d"))
        }
        Some((Some(from), None)) => format!("From {}", from.format("%Y-%m-%d")),
        Some((None, Some(to))) => format!("Until {}", to.format("%Y-%m-%d")),
        _ => "All dates".to_string(),
    };

    Ok(DailyReport {
        date: date_str,
        projects: projects.into_values().collect(),
        total_sessions: all_sessions.len(),
        total_messages,
        total_cost_usd: total_cost,
        total_duration_ms: total_duration,
        models_used: global_models,
        tools_used,
    })
}
