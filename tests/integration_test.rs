use cc2report::{conversation_analyzer, parser, smart_analyzer};
use chrono::NaiveDate;
use std::path::PathBuf;

#[test]
fn test_parse_jsonl_entries() {
    let test_data_dir = PathBuf::from("tests/data/test-project/test-session.jsonl");
    let file = std::fs::File::open(&test_data_dir).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut entry_count = 0;
    let mut user_count = 0;
    let mut assistant_count = 0;
    let mut summary_count = 0;

    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<parser::LogRecord>(&line) {
            Ok(parser::LogRecord::Entry(entry)) => {
                // The summary entry might be parsed as a LogEntry with type="summary"
                // due to untagged enum parsing order
                if entry.entry_type == "summary" {
                    summary_count += 1;
                } else {
                    entry_count += 1;
                    match entry.entry_type.as_str() {
                        "user" => user_count += 1,
                        "assistant" => assistant_count += 1,
                        _ => {
                            eprintln!("Unknown entry type: {}", entry.entry_type);
                        }
                    }
                }
            }
            Ok(parser::LogRecord::Summary(_)) => {
                summary_count += 1;
            }
            Err(e) => {
                panic!("Failed to parse line: {}\nError: {}", line, e);
            }
        }
    }

    assert_eq!(entry_count, 6, "Expected 6 log entries (excluding summary)");
    assert_eq!(user_count, 3, "Expected 3 user messages");
    assert_eq!(assistant_count, 3, "Expected 3 assistant messages");
    assert_eq!(summary_count, 1, "Expected 1 summary entry");
    assert_eq!(entry_count + summary_count, 7, "Expected 7 total records");
}

#[test]
fn test_conversation_flow_analysis() {
    let mut flow = conversation_analyzer::ConversationFlow::new();

    // Simulate conversation analysis
    flow.analyze_user_message(
        "Test message about implementing a feature",
        "2025-06-29T10:00:00.000Z",
    );

    // Create a proper Message struct for assistant response
    let message = parser::Message {
        role: Some("assistant".to_string()),
        content: Some(
            serde_json::json!([{"type": "text", "text": "I'll help you implement that feature."}]),
        ),
        model: None,
        id: None,
        message_type: None,
    };
    flow.analyze_assistant_response(&message);

    flow.analyze_user_message("Please add error handling", "2025-06-29T10:01:00.000Z");

    let message2 = parser::Message {
        role: Some("assistant".to_string()),
        content: Some(
            serde_json::json!([{"type": "text", "text": "Added error handling to the code."}]),
        ),
        model: None,
        id: None,
        message_type: None,
    };
    flow.analyze_assistant_response(&message2);

    flow.finalize();

    assert!(!flow.topics.is_empty(), "Should have at least one topic");
    assert_eq!(flow.topics[0].steps.len(), 2, "Should have 2 work steps");
    assert_eq!(
        flow.topics[0].user_intent.len() > 0,
        true,
        "Should have user intent"
    );
}

#[test]
fn test_date_filtering() {
    let test_date = NaiveDate::from_ymd_opt(2025, 6, 29).unwrap();

    // Test date range logic
    let filter = Some((Some(test_date), Some(test_date)));

    // This date should be included
    let timestamp = chrono::DateTime::parse_from_rfc3339("2025-06-29T10:00:00.000Z")
        .unwrap()
        .naive_utc()
        .date();
    assert!(is_in_date_range(timestamp, filter));

    // This date should be excluded
    let timestamp = chrono::DateTime::parse_from_rfc3339("2025-06-30T10:00:00.000Z")
        .unwrap()
        .naive_utc()
        .date();
    assert!(!is_in_date_range(timestamp, filter));
}

#[test]
fn test_smart_report_structure() {
    use smart_analyzer::*;

    let report = SmartReport {
        date: "2025-06-29".to_string(),
        summary: ExecutiveSummary {
            total_work_items: 10,
            completion_rate: 85.0,
            key_focus_areas: vec!["Development".to_string(), "Testing".to_string()],
            productivity_insights: "Good progress on feature implementation".to_string(),
        },
        projects: vec![SmartProjectSummary {
            title: "Test Project".to_string(),
            objective: "Implement test features".to_string(),
            work_breakdown: WorkBreakdown {
                primary_activities: vec![SmartActivity {
                    category: "Development".to_string(),
                    summary: "Implemented core features".to_string(),
                    item_count: 5,
                    complexity: ComplexityLevel::Moderate,
                }],
                effort_distribution: EffortMetrics {
                    development: 60.0,
                    debugging: 20.0,
                    configuration: 10.0,
                    other: 10.0,
                },
            },
            outcomes: ProjectOutcomes {
                delivered_value: vec!["Feature X implemented".to_string()],
                technical_improvements: vec!["Improved performance".to_string()],
                unresolved_issues: vec!["Need to add more tests".to_string()],
            },
        }],
        token_tracker: None,
    };

    // Test JSON serialization
    let json = serde_json::to_string_pretty(&report).unwrap();
    assert!(json.contains("Test Project"));
    assert!(json.contains("Development"));
    assert!(json.contains("85"));
}

// Helper function matching the one in work_report_v2.rs
fn is_in_date_range(
    date: chrono::NaiveDate,
    filter: Option<(Option<chrono::NaiveDate>, Option<chrono::NaiveDate>)>,
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

#[cfg(test)]
mod mock_ai_tests {

    #[test]
    fn test_ai_response_parsing() {
        // Test that we can parse AI responses correctly
        let mock_response = r#"{
            "project_title": "Test Project",
            "project_purpose": "Testing AI parsing",
            "main_activities": [
                {
                    "category": "Development",
                    "description": "Wrote test code",
                    "impact": "Essential for quality",
                    "technical_details": null
                }
            ],
            "achievements": ["Tests written"],
            "challenges": ["None"],
            "insights": "Testing is important"
        }"#;

        let parsed: Result<cc2report::ai_analyzer::AiAnalysisResponse, _> =
            serde_json::from_str(mock_response);

        assert!(parsed.is_ok());
        let response = parsed.unwrap();
        assert_eq!(response.project_title, "Test Project");
        assert_eq!(response.main_activities.len(), 1);
        assert_eq!(response.achievements.len(), 1);
    }
}
