use crate::ai_analyzer::{analyze_with_ai, AiAnalysisResponse, ConversationData};
use crate::conversation_analyzer::Topic;
use crate::token_tracker::TokenTracker;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct IntelligentReport {
    pub date: String,
    pub projects: Vec<IntelligentProjectSummary>,
    pub overall_insights: String,
    pub token_tracker: Option<TokenTracker>,
}

#[derive(Debug, Serialize)]
pub struct IntelligentProjectSummary {
    pub title: String,
    pub purpose: String,
    pub work_summary: WorkSummary,
    pub key_achievements: Vec<Achievement>,
    pub blockers: Vec<Blocker>,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkSummary {
    pub primary_focus: String,
    pub activities: Vec<Activity>,
    pub time_distribution: HashMap<String, f32>,
}

#[derive(Debug, Serialize)]
pub struct Activity {
    pub description: String,
    pub category: String,
    pub impact_level: ImpactLevel,
}

#[derive(Debug, Serialize)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize)]
pub struct Achievement {
    pub description: String,
    pub impact: String,
}

#[derive(Debug, Serialize)]
pub struct Blocker {
    pub issue: String,
    pub severity: Severity,
    pub resolution_status: ResolutionStatus,
}

#[derive(Debug, Serialize)]
pub enum Severity {
    Critical,
    Major,
    Minor,
}

#[derive(Debug, Serialize)]
pub enum ResolutionStatus {
    Resolved,
    InProgress,
    Blocked,
}

#[allow(clippy::too_many_arguments)]
pub async fn create_intelligent_summary(
    projects: HashMap<String, Vec<Topic>>,
    date_str: String,
    use_ai: bool,
    api_key: Option<&str>,
    lang: &str,
    model: &str,
    quiet: bool,
    parallel: usize,
) -> Result<IntelligentReport, Box<dyn std::error::Error>> {
    let mut project_summaries = Vec::new();
    let mut token_tracker_option = None;

    if use_ai && api_key.is_some() {
        // Convert topics to conversation data for AI analysis
        let conversations: Vec<ConversationData> = projects
            .iter()
            .map(|(name, topics)| ConversationData {
                project_name: name.clone(),
                user_messages: extract_user_messages(topics),
                assistant_actions: extract_assistant_actions(topics),
                timestamps: extract_timestamps(topics),
            })
            .collect();

        let (ai_results, token_tracker) = analyze_with_ai(
            api_key.unwrap(),
            conversations,
            lang,
            model,
            quiet,
            parallel,
        )
        .await?;
        token_tracker_option = Some(token_tracker);

        // Process successful AI results
        let mut processed_projects = std::collections::HashSet::new();
        for (project_name, ai_response) in ai_results {
            project_summaries.push(convert_ai_response_to_summary(ai_response));
            processed_projects.insert(project_name);
        }

        // Fallback for projects that failed AI analysis
        for (project_name, topics) in &projects {
            if !processed_projects.contains(project_name) {
                eprintln!(
                    "Using fallback analysis for project '{project_name}' after AI analysis failed"
                );
                let summary = analyze_project_intelligently(project_name, topics);
                project_summaries.push(summary);
            }
        }
    } else {
        // Fallback to rule-based analysis with smarter heuristics
        for (project_name, topics) in &projects {
            let summary = analyze_project_intelligently(project_name, topics);
            project_summaries.push(summary);
        }
    }

    let failed_count = if use_ai && api_key.is_some() {
        projects.len().saturating_sub(project_summaries.len())
    } else {
        0
    };
    let mut overall_insights = generate_overall_insights(&project_summaries);

    if failed_count > 0 {
        overall_insights.push_str(&format!(
            " Note: {failed_count} project(s) failed to analyze."
        ));
    }

    Ok(IntelligentReport {
        date: date_str,
        projects: project_summaries,
        overall_insights,
        token_tracker: token_tracker_option,
    })
}

fn analyze_project_intelligently(
    project_name: &str,
    topics: &[Topic],
) -> IntelligentProjectSummary {
    // Use statistical analysis instead of keyword matching
    let activity_patterns = detect_activity_patterns(topics);
    let work_focus = determine_primary_focus(&activity_patterns);

    IntelligentProjectSummary {
        title: generate_intelligent_title(project_name, topics),
        purpose: infer_project_purpose(topics),
        work_summary: WorkSummary {
            primary_focus: work_focus,
            activities: group_activities_intelligently(topics),
            time_distribution: calculate_time_distribution(topics),
        },
        key_achievements: extract_meaningful_achievements(topics),
        blockers: identify_blockers(topics),
        next_steps: suggest_next_steps(topics),
    }
}

fn detect_activity_patterns(topics: &[Topic]) -> HashMap<String, f32> {
    let mut patterns = HashMap::new();

    // Analyze action patterns rather than keywords
    for topic in topics {
        let pattern_type = classify_by_behavior(&topic.user_intent, &topic.steps);
        *patterns.entry(pattern_type).or_insert(0.0) += 1.0;
    }

    // Normalize
    let total: f32 = patterns.values().sum();
    for value in patterns.values_mut() {
        *value /= total;
    }

    patterns
}

fn classify_by_behavior(_intent: &str, steps: &[crate::conversation_analyzer::WorkStep]) -> String {
    // Look at the sequence of actions to understand what was done
    let action_sequence: Vec<&str> = steps.iter().map(|s| s.description.as_str()).collect();

    // Pattern matching on action sequences
    if action_sequence.contains(&"Create file") && action_sequence.contains(&"Write code") {
        "Feature Implementation".to_string()
    } else if action_sequence.contains(&"Read file") && action_sequence.contains(&"Fix error") {
        "Debugging & Fixes".to_string()
    } else if action_sequence.contains(&"Run tests") || action_sequence.contains(&"Verify") {
        "Quality Assurance".to_string()
    } else if action_sequence.contains(&"Update config")
        || action_sequence.contains(&"Change settings")
    {
        "Configuration Management".to_string()
    } else {
        "General Development".to_string()
    }
}

fn generate_intelligent_title(project_path: &str, topics: &[Topic]) -> String {
    // Extract meaningful project name from path and context
    let path_parts: Vec<&str> = project_path.split('/').collect();
    let last_meaningful_part = path_parts
        .iter()
        .rev()
        .find(|&&part| !part.is_empty() && part != "home" && part != "projects")
        .unwrap_or(&"Project");

    // Look for patterns in topics to enhance the title
    let main_theme = detect_main_theme(topics);

    format!(
        "{} - {}",
        format_project_name(last_meaningful_part),
        main_theme
    )
}

fn detect_main_theme(topics: &[Topic]) -> String {
    // Analyze all topics to find the overarching theme
    if topics.is_empty() {
        return "Development Work".to_string();
    }

    // Count different types of work
    let mut theme_counts: HashMap<&str, usize> = HashMap::new();

    for topic in topics {
        if topic.user_intent.contains("report") || topic.user_intent.contains("analyze") {
            *theme_counts.entry("Analytics & Reporting").or_insert(0) += 1;
        } else if topic.user_intent.contains("display") || topic.user_intent.contains("UI") {
            *theme_counts.entry("UI Development").or_insert(0) += 1;
        } else if topic.user_intent.contains("fix") || topic.user_intent.contains("error") {
            *theme_counts.entry("Bug Fixes & Improvements").or_insert(0) += 1;
        } else {
            *theme_counts.entry("Feature Development").or_insert(0) += 1;
        }
    }

    theme_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(theme, _)| theme.to_string())
        .unwrap_or_else(|| "Development Work".to_string())
}

fn format_project_name(name: &str) -> String {
    // Convert technical names to readable format
    name.replace(['-', '_'], " ")
        .split_whitespace()
        .map(|word| {
            if word.len() <= 3 && word.chars().all(|c| c.is_alphabetic()) {
                word.to_uppercase()
            } else {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn group_activities_intelligently(topics: &[Topic]) -> Vec<Activity> {
    let mut activities = Vec::new();
    let mut activity_groups: HashMap<String, Vec<&Topic>> = HashMap::new();

    // Group by semantic similarity, not keywords
    for topic in topics {
        let group = determine_semantic_group(topic);
        activity_groups.entry(group).or_default().push(topic);
    }

    // Convert groups to activities
    for (group_name, group_topics) in activity_groups {
        if !group_topics.is_empty() {
            activities.push(Activity {
                description: create_activity_description(&group_name, group_topics.len()),
                category: group_name.clone(),
                impact_level: assess_impact_level(&group_topics),
            });
        }
    }

    // Sort by impact level
    activities.sort_by(|a, b| match (&a.impact_level, &b.impact_level) {
        (ImpactLevel::High, ImpactLevel::High) => std::cmp::Ordering::Equal,
        (ImpactLevel::High, _) => std::cmp::Ordering::Less,
        (_, ImpactLevel::High) => std::cmp::Ordering::Greater,
        (ImpactLevel::Medium, ImpactLevel::Medium) => std::cmp::Ordering::Equal,
        (ImpactLevel::Medium, _) => std::cmp::Ordering::Less,
        (_, ImpactLevel::Medium) => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    });

    activities
}

fn determine_semantic_group(topic: &Topic) -> String {
    // Group by the actual work done, not keywords
    let intent_lower = topic.user_intent.to_lowercase();
    let has_code_changes = topic.steps.iter().any(|s| {
        s.description.contains("Write")
            || s.description.contains("Edit")
            || s.description.contains("Create")
    });
    let has_analysis = topic.steps.iter().any(|s| {
        s.description.contains("Read")
            || s.description.contains("Analyze")
            || s.description.contains("Search")
    });

    if has_code_changes && intent_lower.contains("implement") {
        "New Feature Implementation".to_string()
    } else if has_code_changes && (intent_lower.contains("fix") || intent_lower.contains("error")) {
        "Bug Fixes and Error Resolution".to_string()
    } else if has_analysis && !has_code_changes {
        "Code Analysis and Research".to_string()
    } else if intent_lower.contains("test") {
        "Testing and Validation".to_string()
    } else if intent_lower.contains("document") {
        "Documentation Updates".to_string()
    } else {
        "Development Tasks".to_string()
    }
}

fn assess_impact_level(topics: &[&Topic]) -> ImpactLevel {
    // Assess based on outcomes and complexity
    let successful_count = topics
        .iter()
        .filter(|t| {
            matches!(
                t.outcome,
                crate::conversation_analyzer::TopicOutcome::Completed(_)
            )
        })
        .count();

    let avg_steps = topics.iter().map(|t| t.steps.len()).sum::<usize>() / topics.len().max(1);

    if successful_count == topics.len() && avg_steps > 5 {
        ImpactLevel::High
    } else if successful_count > topics.len() / 2 {
        ImpactLevel::Medium
    } else {
        ImpactLevel::Low
    }
}

fn create_activity_description(category: &str, count: usize) -> String {
    if count == 1 {
        category.to_string()
    } else {
        format!("{category} ({count} tasks)")
    }
}

fn calculate_time_distribution(topics: &[Topic]) -> HashMap<String, f32> {
    let mut distribution = HashMap::new();
    let total_steps: usize = topics.iter().map(|t| t.steps.len()).sum();

    for topic in topics {
        let category = determine_semantic_group(topic);
        let steps = topic.steps.len() as f32;
        *distribution.entry(category).or_insert(0.0) += steps;
    }

    // Convert to percentages
    for value in distribution.values_mut() {
        *value = (*value / total_steps as f32) * 100.0;
    }

    distribution
}

fn extract_meaningful_achievements(topics: &[Topic]) -> Vec<Achievement> {
    let mut achievements = Vec::new();

    for topic in topics {
        if let crate::conversation_analyzer::TopicOutcome::Completed(_) = &topic.outcome {
            if let Some(achievement) = create_achievement_from_topic(topic) {
                achievements.push(achievement);
            }
        }
    }

    // Deduplicate and prioritize
    deduplicate_achievements(&mut achievements);
    achievements.truncate(5); // Keep top 5

    achievements
}

fn create_achievement_from_topic(topic: &Topic) -> Option<Achievement> {
    let intent = &topic.user_intent;
    let has_significant_changes = topic.steps.len() > 3;

    if has_significant_changes {
        Some(Achievement {
            description: summarize_achievement(intent, &topic.steps),
            impact: assess_achievement_impact(topic),
        })
    } else {
        None
    }
}

fn summarize_achievement(intent: &str, steps: &[crate::conversation_analyzer::WorkStep]) -> String {
    // Create a meaningful summary based on intent and actions
    let key_actions: Vec<&str> = steps
        .iter()
        .filter(|s| {
            matches!(
                s.result,
                crate::conversation_analyzer::StepResult::Success(_)
            )
        })
        .map(|s| s.description.as_str())
        .collect();

    if key_actions.is_empty() {
        simplify_intent(intent)
    } else {
        format!(
            "{} through {}",
            simplify_intent(intent),
            summarize_actions(&key_actions)
        )
    }
}

fn simplify_intent(intent: &str) -> String {
    // Extract the core action from the intent
    intent
        .split_whitespace()
        .take(10)
        .collect::<Vec<&str>>()
        .join(" ")
}

fn summarize_actions(actions: &[&str]) -> String {
    let unique_actions: std::collections::HashSet<_> = actions.iter().cloned().collect();
    if unique_actions.len() == 1 {
        actions
            .first()
            .map(|action| action.to_lowercase())
            .unwrap_or_else(|| "unknown action".to_string())
    } else {
        format!("{} actions", unique_actions.len())
    }
}

fn assess_achievement_impact(topic: &Topic) -> String {
    match topic.steps.len() {
        0..=2 => "Minor improvement".to_string(),
        3..=5 => "Moderate enhancement".to_string(),
        _ => "Significant advancement".to_string(),
    }
}

fn deduplicate_achievements(achievements: &mut Vec<Achievement>) {
    // Remove similar achievements
    achievements.sort_by(|a, b| b.impact.cmp(&a.impact));
    achievements.dedup_by(|a, b| similarity_score(&a.description, &b.description) > 0.7);
}

fn similarity_score(a: &str, b: &str) -> f32 {
    // Simple word overlap similarity
    let words_a: std::collections::HashSet<_> = a.split_whitespace().collect();
    let words_b: std::collections::HashSet<_> = b.split_whitespace().collect();
    let intersection = words_a.intersection(&words_b).count();
    let union = words_a.union(&words_b).count();

    if union == 0 {
        0.0
    } else {
        intersection as f32 / union as f32
    }
}

fn identify_blockers(topics: &[Topic]) -> Vec<Blocker> {
    let mut blockers = Vec::new();

    for topic in topics {
        match &topic.outcome {
            crate::conversation_analyzer::TopicOutcome::Failed(msg) => {
                blockers.push(Blocker {
                    issue: create_blocker_description(topic, msg),
                    severity: assess_severity(topic),
                    resolution_status: ResolutionStatus::Blocked,
                });
            }
            crate::conversation_analyzer::TopicOutcome::PartiallyCompleted(msg) => {
                blockers.push(Blocker {
                    issue: create_blocker_description(topic, msg),
                    severity: Severity::Major,
                    resolution_status: ResolutionStatus::InProgress,
                });
            }
            _ => {}
        }
    }

    blockers
}

fn create_blocker_description(topic: &Topic, error_msg: &str) -> String {
    format!(
        "While {}: {}",
        simplify_intent(&topic.user_intent).to_lowercase(),
        simplify_error_message(error_msg)
    )
}

fn simplify_error_message(msg: &str) -> String {
    msg.split('\n').next().unwrap_or(msg).trim().to_string()
}

fn assess_severity(topic: &Topic) -> Severity {
    let failed_steps = topic
        .steps
        .iter()
        .filter(|s| {
            matches!(
                s.result,
                crate::conversation_analyzer::StepResult::Failed(_)
            )
        })
        .count();

    match failed_steps {
        0..=1 => Severity::Minor,
        2..=3 => Severity::Major,
        _ => Severity::Critical,
    }
}

fn suggest_next_steps(topics: &[Topic]) -> Vec<String> {
    let mut suggestions = Vec::new();

    // Based on failures and partial completions
    for topic in topics {
        if matches!(
            topic.outcome,
            crate::conversation_analyzer::TopicOutcome::Failed(_)
                | crate::conversation_analyzer::TopicOutcome::PartiallyCompleted(_)
        ) {
            if let Some(suggestion) = generate_suggestion_for_topic(topic) {
                suggestions.push(suggestion);
            }
        }
    }

    suggestions.truncate(3); // Keep top 3 suggestions
    suggestions
}

fn generate_suggestion_for_topic(topic: &Topic) -> Option<String> {
    match &topic.outcome {
        crate::conversation_analyzer::TopicOutcome::Failed(_) => Some(format!(
            "Retry {}",
            simplify_intent(&topic.user_intent).to_lowercase()
        )),
        crate::conversation_analyzer::TopicOutcome::PartiallyCompleted(_) => Some(format!(
            "Complete remaining tasks for {}",
            simplify_intent(&topic.user_intent).to_lowercase()
        )),
        _ => None,
    }
}

fn infer_project_purpose(topics: &[Topic]) -> String {
    if topics.is_empty() {
        return "Project development and maintenance".to_string();
    }

    // Look at the first few topics to understand the overall goal
    let initial_intents: Vec<&str> = topics
        .iter()
        .take(3)
        .map(|t| t.user_intent.as_str())
        .collect();

    // Synthesize a purpose from the intents
    synthesize_purpose(&initial_intents)
}

fn synthesize_purpose(intents: &[&str]) -> String {
    // This would be more sophisticated in production
    if intents
        .iter()
        .any(|i| i.contains("report") || i.contains("analyze"))
    {
        "Building analytics and reporting capabilities".to_string()
    } else if intents
        .iter()
        .any(|i| i.contains("UI") || i.contains("display"))
    {
        "Developing user interface components".to_string()
    } else {
        "Software development and improvement".to_string()
    }
}

fn determine_primary_focus(patterns: &HashMap<String, f32>) -> String {
    patterns
        .iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(k, _)| k.clone())
        .unwrap_or_else(|| "General Development".to_string())
}

fn generate_overall_insights(summaries: &[IntelligentProjectSummary]) -> String {
    // Analyze across all projects for patterns
    let total_achievements: usize = summaries.iter().map(|s| s.key_achievements.len()).sum();

    let total_blockers: usize = summaries.iter().map(|s| s.blockers.len()).sum();

    format!(
        "Completed {} projects with {} key achievements and {} blockers to address. {}",
        summaries.len(),
        total_achievements,
        total_blockers,
        if total_blockers > total_achievements {
            "Focus needed on resolving technical debt."
        } else {
            "Good progress with manageable technical challenges."
        }
    )
}

fn extract_user_messages(topics: &[Topic]) -> Vec<String> {
    topics.iter().map(|t| t.user_intent.clone()).collect()
}

fn extract_assistant_actions(topics: &[Topic]) -> Vec<String> {
    topics
        .iter()
        .flat_map(|t| t.steps.iter().map(|s| s.description.clone()))
        .collect()
}

fn extract_timestamps(topics: &[Topic]) -> Vec<String> {
    topics.iter().map(|t| t.timestamp.clone()).collect()
}

fn convert_ai_response_to_summary(response: AiAnalysisResponse) -> IntelligentProjectSummary {
    IntelligentProjectSummary {
        title: response.project_title,
        purpose: response.project_purpose,
        work_summary: WorkSummary {
            primary_focus: response
                .main_activities
                .first()
                .map(|a| a.category.clone())
                .unwrap_or_else(|| "General Development".to_string()),
            activities: response
                .main_activities
                .into_iter()
                .map(|a| Activity {
                    description: a.description,
                    category: a.category,
                    impact_level: match a.impact.to_lowercase().as_str() {
                        s if s.contains("critical") || s.contains("major") => ImpactLevel::High,
                        s if s.contains("moderate") => ImpactLevel::Medium,
                        _ => ImpactLevel::Low,
                    },
                })
                .collect(),
            time_distribution: HashMap::new(),
        },
        key_achievements: response
            .achievements
            .into_iter()
            .map(|a| Achievement {
                description: a,
                impact: "Completed successfully".to_string(),
            })
            .collect(),
        blockers: response
            .challenges
            .into_iter()
            .map(|c| Blocker {
                issue: c,
                severity: Severity::Major,
                resolution_status: ResolutionStatus::InProgress,
            })
            .collect(),
        next_steps: vec![],
    }
}
