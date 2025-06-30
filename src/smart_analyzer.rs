use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SmartReport {
    pub date: String,
    pub summary: ExecutiveSummary,
    pub projects: Vec<SmartProjectSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_tracker: Option<crate::token_tracker::TokenTracker>,
}

#[derive(Debug, Serialize)]
pub struct ExecutiveSummary {
    pub total_work_items: usize,
    pub completion_rate: f32,
    pub key_focus_areas: Vec<String>,
    pub productivity_insights: String,
}

#[derive(Debug, Serialize)]
pub struct SmartProjectSummary {
    pub title: String,
    pub objective: String,
    pub work_breakdown: WorkBreakdown,
    pub outcomes: ProjectOutcomes,
}

#[derive(Debug, Serialize)]
pub struct WorkBreakdown {
    pub primary_activities: Vec<SmartActivity>,
    pub effort_distribution: EffortMetrics,
}

#[derive(Debug, Serialize)]
pub struct SmartActivity {
    pub category: String,
    pub summary: String,
    pub item_count: usize,
    pub complexity: ComplexityLevel,
}

#[derive(Debug, Serialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
}

#[derive(Debug, Serialize)]
pub struct EffortMetrics {
    pub development: f32,
    pub debugging: f32,
    pub configuration: f32,
    pub other: f32,
}

#[derive(Debug, Serialize)]
pub struct ProjectOutcomes {
    pub delivered_value: Vec<String>,
    pub technical_improvements: Vec<String>,
    pub unresolved_issues: Vec<String>,
}

pub fn generate_smart_report(
    report: &SmartReport,
    output_path: Option<&std::path::Path>,
    lang: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = String::new();

    match lang {
        "ja" => generate_japanese_smart_report(&mut output, report),
        "zh" => generate_chinese_smart_report(&mut output, report),
        "ko" => generate_korean_smart_report(&mut output, report),
        "es" => generate_spanish_smart_report(&mut output, report),
        "fr" => generate_french_smart_report(&mut output, report),
        _ => generate_english_smart_report(&mut output, report),
    }

    if let Some(path) = output_path {
        std::fs::write(path, output)?;
        println!("{}", get_output_message(lang, path));
    } else {
        print!("{}", output);
    }

    Ok(())
}

fn get_output_message(_lang: &str, path: &std::path::Path) -> String {
    // Always use English for system messages
    format!("AI work report written to: {}", path.display())
}

fn generate_japanese_smart_report(output: &mut String, report: &SmartReport) {
    output.push_str(&format!("# 作業報告書 - {}\n\n", report.date));

    for project in &report.projects {
        output.push_str(&format!("## {}\n\n", project.title));
        output.push_str(&format!("**目的**: {}\n\n", project.objective));

        // Work breakdown
        if !project.work_breakdown.primary_activities.is_empty() {
            output.push_str("**作業内容**:\n");
            for activity in &project.work_breakdown.primary_activities {
                output.push_str(&format!("- {} - {}\n", activity.category, activity.summary));
            }
            output.push_str("\n");
        }

        // Outcomes
        if !project.outcomes.delivered_value.is_empty() {
            output.push_str("**成果**:\n");
            for value in &project.outcomes.delivered_value {
                output.push_str(&format!("- {}\n", value));
            }
            output.push_str("\n");
        }

        if !project.outcomes.technical_improvements.is_empty() {
            output.push_str("**技術的改善**:\n");
            for improvement in &project.outcomes.technical_improvements {
                output.push_str(&format!("- {}\n", improvement));
            }
            output.push_str("\n");
        }

        if !project.outcomes.unresolved_issues.is_empty() {
            output.push_str("**未解決の課題**:\n");
            for issue in &project.outcomes.unresolved_issues {
                output.push_str(&format!("- {}\n", issue));
            }
            output.push_str("\n");
        }
    }

    // Add token usage summary if available
    if let Some(ref tracker) = report.token_tracker {
        output.push_str(&tracker.get_summary_string("ja"));
    }
}

fn generate_english_smart_report(output: &mut String, report: &SmartReport) {
    output.push_str(&format!("# Work Report - {}\n\n", report.date));

    for project in &report.projects {
        output.push_str(&format!("## {}\n\n", project.title));
        output.push_str(&format!("**Objective**: {}\n\n", project.objective));

        // Work breakdown
        if !project.work_breakdown.primary_activities.is_empty() {
            output.push_str("**Activities**:\n");
            for activity in &project.work_breakdown.primary_activities {
                output.push_str(&format!("- {} - {}\n", activity.category, activity.summary));
            }
            output.push_str("\n");
        }

        // Outcomes
        if !project.outcomes.delivered_value.is_empty() {
            output.push_str("**Delivered Value**:\n");
            for value in &project.outcomes.delivered_value {
                output.push_str(&format!("- {}\n", value));
            }
            output.push_str("\n");
        }

        if !project.outcomes.technical_improvements.is_empty() {
            output.push_str("**Technical Improvements**:\n");
            for improvement in &project.outcomes.technical_improvements {
                output.push_str(&format!("- {}\n", improvement));
            }
            output.push_str("\n");
        }

        if !project.outcomes.unresolved_issues.is_empty() {
            output.push_str("**Unresolved Issues**:\n");
            for issue in &project.outcomes.unresolved_issues {
                output.push_str(&format!("- {}\n", issue));
            }
            output.push_str("\n");
        }
    }

    // Add token usage summary if available
    if let Some(ref tracker) = report.token_tracker {
        output.push_str(&tracker.get_summary_string("en"));
    }
}

fn generate_chinese_smart_report(output: &mut String, report: &SmartReport) {
    output.push_str(&format!("# 工作报告 - {}\n\n", report.date));

    for project in &report.projects {
        output.push_str(&format!("## {}\n\n", project.title));
        output.push_str(&format!("**目标**: {}\n\n", project.objective));

        if !project.work_breakdown.primary_activities.is_empty() {
            output.push_str("**工作内容**:\n");
            for activity in &project.work_breakdown.primary_activities {
                output.push_str(&format!("- {} - {}\n", activity.category, activity.summary));
            }
            output.push_str("\n");
        }

        if !project.outcomes.delivered_value.is_empty() {
            output.push_str("**交付价值**:\n");
            for value in &project.outcomes.delivered_value {
                output.push_str(&format!("- {}\n", value));
            }
            output.push_str("\n");
        }

        if !project.outcomes.technical_improvements.is_empty() {
            output.push_str("**技术改进**:\n");
            for improvement in &project.outcomes.technical_improvements {
                output.push_str(&format!("- {}\n", improvement));
            }
            output.push_str("\n");
        }

        if !project.outcomes.unresolved_issues.is_empty() {
            output.push_str("**未解决的问题**:\n");
            for issue in &project.outcomes.unresolved_issues {
                output.push_str(&format!("- {}\n", issue));
            }
            output.push_str("\n");
        }
    }

    // Add token usage summary if available
    if let Some(ref tracker) = report.token_tracker {
        output.push_str(&tracker.get_summary_string("zh"));
    }
}

fn generate_korean_smart_report(output: &mut String, report: &SmartReport) {
    output.push_str(&format!("# 작업 보고서 - {}\n\n", report.date));

    for project in &report.projects {
        output.push_str(&format!("## {}\n\n", project.title));
        output.push_str(&format!("**목표**: {}\n\n", project.objective));

        if !project.work_breakdown.primary_activities.is_empty() {
            output.push_str("**작업 내용**:\n");
            for activity in &project.work_breakdown.primary_activities {
                output.push_str(&format!("- {} - {}\n", activity.category, activity.summary));
            }
            output.push_str("\n");
        }

        if !project.outcomes.delivered_value.is_empty() {
            output.push_str("**제공된 가치**:\n");
            for value in &project.outcomes.delivered_value {
                output.push_str(&format!("- {}\n", value));
            }
            output.push_str("\n");
        }

        if !project.outcomes.technical_improvements.is_empty() {
            output.push_str("**기술적 개선사항**:\n");
            for improvement in &project.outcomes.technical_improvements {
                output.push_str(&format!("- {}\n", improvement));
            }
            output.push_str("\n");
        }

        if !project.outcomes.unresolved_issues.is_empty() {
            output.push_str("**미해결 문제**:\n");
            for issue in &project.outcomes.unresolved_issues {
                output.push_str(&format!("- {}\n", issue));
            }
            output.push_str("\n");
        }
    }

    // Add token usage summary if available
    if let Some(ref tracker) = report.token_tracker {
        output.push_str(&tracker.get_summary_string("ko"));
    }
}

fn generate_spanish_smart_report(output: &mut String, report: &SmartReport) {
    output.push_str(&format!("# Informe de Trabajo - {}\n\n", report.date));

    for project in &report.projects {
        output.push_str(&format!("## {}\n\n", project.title));
        output.push_str(&format!("**Objetivo**: {}\n\n", project.objective));

        if !project.work_breakdown.primary_activities.is_empty() {
            output.push_str("**Actividades**:\n");
            for activity in &project.work_breakdown.primary_activities {
                output.push_str(&format!("- {} - {}\n", activity.category, activity.summary));
            }
            output.push_str("\n");
        }

        if !project.outcomes.delivered_value.is_empty() {
            output.push_str("**Valor entregado**:\n");
            for value in &project.outcomes.delivered_value {
                output.push_str(&format!("- {}\n", value));
            }
            output.push_str("\n");
        }

        if !project.outcomes.technical_improvements.is_empty() {
            output.push_str("**Mejoras técnicas**:\n");
            for improvement in &project.outcomes.technical_improvements {
                output.push_str(&format!("- {}\n", improvement));
            }
            output.push_str("\n");
        }

        if !project.outcomes.unresolved_issues.is_empty() {
            output.push_str("**Problemas no resueltos**:\n");
            for issue in &project.outcomes.unresolved_issues {
                output.push_str(&format!("- {}\n", issue));
            }
            output.push_str("\n");
        }
    }

    // Add token usage summary if available
    if let Some(ref tracker) = report.token_tracker {
        output.push_str(&tracker.get_summary_string("es"));
    }
}

fn generate_french_smart_report(output: &mut String, report: &SmartReport) {
    output.push_str(&format!("# Rapport de Travail - {}\n\n", report.date));

    for project in &report.projects {
        output.push_str(&format!("## {}\n\n", project.title));
        output.push_str(&format!("**Objectif**: {}\n\n", project.objective));

        if !project.work_breakdown.primary_activities.is_empty() {
            output.push_str("**Activités**:\n");
            for activity in &project.work_breakdown.primary_activities {
                output.push_str(&format!("- {} - {}\n", activity.category, activity.summary));
            }
            output.push_str("\n");
        }

        if !project.outcomes.delivered_value.is_empty() {
            output.push_str("**Valeur livrée**:\n");
            for value in &project.outcomes.delivered_value {
                output.push_str(&format!("- {}\n", value));
            }
            output.push_str("\n");
        }

        if !project.outcomes.technical_improvements.is_empty() {
            output.push_str("**Améliorations techniques**:\n");
            for improvement in &project.outcomes.technical_improvements {
                output.push_str(&format!("- {}\n", improvement));
            }
            output.push_str("\n");
        }

        if !project.outcomes.unresolved_issues.is_empty() {
            output.push_str("**Problèmes non résolus**:\n");
            for issue in &project.outcomes.unresolved_issues {
                output.push_str(&format!("- {}\n", issue));
            }
            output.push_str("\n");
        }
    }

    // Add token usage summary if available
    if let Some(ref tracker) = report.token_tracker {
        output.push_str(&tracker.get_summary_string("fr"));
    }
}
