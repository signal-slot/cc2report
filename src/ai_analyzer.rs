use crate::cache::ApiCache;
use crate::templates::{get_template_path, Templates};
use crate::token_tracker::{TokenTracker, TokenUsage};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AiAnalysisRequest {
    pub conversations: Vec<ConversationData>,
    pub analysis_type: AnalysisType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationData {
    pub project_name: String,
    pub user_messages: Vec<String>,
    pub assistant_actions: Vec<String>,
    pub timestamps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnalysisType {
    ProjectSummary,
    TaskCategorization,
    AchievementExtraction,
    ChallengeIdentification,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiAnalysisResponse {
    pub project_title: String,
    pub project_purpose: String,
    pub main_activities: Vec<AiActivity>,
    pub achievements: Vec<String>,
    pub challenges: Vec<String>,
    pub insights: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiActivity {
    pub category: String,
    pub description: String,
    pub impact: String,
    pub technical_details: Option<String>,
}

pub async fn analyze_with_ai(
    api_key: &str,
    conversations: Vec<ConversationData>,
    lang: &str,
    model: &str,
    quiet: bool,
    parallel: usize,
) -> Result<(HashMap<String, AiAnalysisResponse>, TokenTracker), Box<dyn std::error::Error>> {
    // Use parallel implementation if requested
    if parallel > 1 {
        return crate::ai_analyzer_parallel::analyze_with_ai_parallel(
            api_key,
            conversations,
            lang,
            model,
            quiet,
            parallel,
        )
        .await;
    }
    let client = reqwest::Client::new();
    let mut results = HashMap::new();
    let mut token_tracker = TokenTracker::new();

    // Initialize cache
    let cache = match ApiCache::new() {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!(
                "Cache initialization error: {}. Continuing without cache.",
                e
            );
            None
        }
    };

    // Load templates
    let templates = if let Some(template_path) = get_template_path() {
        match Templates::load_from_file(&template_path) {
            Ok(t) => Templates::merge_with_defaults(Some(t)),
            Err(e) => {
                eprintln!("Template loading error: {}. Using defaults.", e);
                Templates::default()
            }
        }
    } else {
        Templates::default()
    };

    // Create progress bar
    let pb = if quiet {
        ProgressBar::hidden()
    } else {
        let pb = ProgressBar::new(conversations.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message("Analyzing projects with AI...");
        pb
    };

    for conv in conversations {
        pb.set_message(format!("Analyzing: {}", conv.project_name));

        // Check cache first
        if let Some(ref cache) = cache {
            let cache_key_messages = vec![
                conv.user_messages.join("\n"),
                conv.assistant_actions.join("\n"),
            ];

            if let Some(cached_response) =
                cache.get(&conv.project_name, lang, model, &cache_key_messages)
            {
                results.insert(conv.project_name.clone(), cached_response);
                pb.inc(1);
                continue;
            }
        }

        let prompt = create_analysis_prompt(&conv, lang, &templates);

        let default_instruction = get_language_instruction(lang).to_string();
        let language_instruction = templates
            .prompts
            .language_instructions
            .get(lang)
            .unwrap_or(&default_instruction);

        let system_message = templates
            .prompts
            .system_message
            .replace("{language_instruction}", language_instruction);

        let model_name = match model {
            "gpt-4o" => "chatgpt-4o-latest",
            "gpt-4o-mini" => "gpt-4o-mini",
            "gpt-4-turbo" => "gpt-4-turbo-preview",
            "gpt-3.5-turbo" => "gpt-3.5-turbo",
            _ => model, // fallback to provided name
        };

        let request_body = serde_json::json!({
            "model": model_name,
            "messages": [
                {
                    "role": "system",
                    "content": system_message
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.3
        });

        match client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(api_response) => {
                            if let Some(content) =
                                api_response["choices"][0]["message"]["content"].as_str()
                            {
                                // Remove markdown code blocks if present
                                let cleaned_content = if content.starts_with("```json")
                                    && content.ends_with("```")
                                {
                                    content
                                        .trim_start_matches("```json")
                                        .trim_end_matches("```")
                                        .trim()
                                } else if content.starts_with("```") && content.ends_with("```") {
                                    content
                                        .trim_start_matches("```")
                                        .trim_end_matches("```")
                                        .trim()
                                } else {
                                    content
                                };

                                match serde_json::from_str::<AiAnalysisResponse>(cleaned_content) {
                                    Ok(analysis) => {
                                        // Track token usage
                                        if let Some(usage) = api_response["usage"].as_object() {
                                            let token_usage = TokenUsage {
                                                prompt_tokens: usage["prompt_tokens"]
                                                    .as_u64()
                                                    .unwrap_or(0)
                                                    as u32,
                                                completion_tokens: usage["completion_tokens"]
                                                    .as_u64()
                                                    .unwrap_or(0)
                                                    as u32,
                                                total_tokens: usage["total_tokens"]
                                                    .as_u64()
                                                    .unwrap_or(0)
                                                    as u32,
                                            };
                                            token_tracker
                                                .add_usage(&conv.project_name, token_usage);
                                        }

                                        // Store in cache
                                        if let Some(ref cache) = cache {
                                            let cache_key_messages = vec![
                                                conv.user_messages.join("\n"),
                                                conv.assistant_actions.join("\n"),
                                            ];
                                            if let Err(e) = cache.set(
                                                &conv.project_name,
                                                lang,
                                                model,
                                                &cache_key_messages,
                                                &analysis,
                                            ) {
                                                eprintln!("Cache save error: {}", e);
                                            }
                                        }

                                        results.insert(conv.project_name.clone(), analysis);
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "JSON parsing error for project '{}': {}",
                                            conv.project_name, e
                                        );
                                        eprintln!("Invalid JSON: {}", cleaned_content);
                                    }
                                }
                            } else {
                                eprintln!(
                                    "No content in API response for project '{}'",
                                    conv.project_name
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "Response parsing error for project '{}': {}",
                                conv.project_name, e
                            );
                        }
                    }
                } else {
                    let status = response.status();
                    match response.text().await {
                        Ok(error_text) => {
                            eprintln!(
                                "OpenAI API error for project '{}': {} - {}",
                                conv.project_name, status, error_text
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to get error text for project '{}': {}",
                                conv.project_name, e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("API call error for project '{}': {}", conv.project_name, e);
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("AI analysis complete!");

    // Calculate cost estimate
    token_tracker.calculate_cost(model);

    Ok((results, token_tracker))
}

fn get_language_instruction(lang: &str) -> &'static str {
    match lang {
        "ja" => "Use Japanese for all text fields.",
        "zh" => "Use Simplified Chinese (简体中文) for all text fields.",
        "ko" => "Use Korean (한국어) for all text fields.",
        "es" => "Use Spanish for all text fields.",
        "fr" => "Use French for all text fields.",
        "de" => "Use German for all text fields.",
        "pt" => "Use Portuguese for all text fields.",
        "ru" => "Use Russian for all text fields.",
        "it" => "Use Italian for all text fields.",
        "nl" => "Use Dutch for all text fields.",
        "pl" => "Use Polish for all text fields.",
        "tr" => "Use Turkish for all text fields.",
        "ar" => "Use Arabic for all text fields.",
        "hi" => "Use Hindi for all text fields.",
        "th" => "Use Thai for all text fields.",
        "vi" => "Use Vietnamese for all text fields.",
        "id" => "Use Indonesian for all text fields.",
        "ms" => "Use Malay for all text fields.",
        _ => "Use English for all text fields.",
    }
}

fn get_prompt_templates(lang: &str) -> (&'static str, &'static str) {
    match lang {
        "ja" => (
            "以下のソフトウェア開発の会話を分析し、JSON形式で応答してください。すべてのテキストフィールドは日本語で記述してください。重要：同じカテゴリの活動は1つのエントリにまとめてください。",
            r#"{{
    "project_title": "意味のあるプロジェクトタイトル（日本語で）",
    "project_purpose": "このプロジェクトの主な目的（日本語で）",
    "main_activities": [
        {{
            "category": "開発/テスト/設定/ドキュメント/バグ修正など（日本語で、各カテゴリは1回のみ）",
            "description": "実際に行った作業内容をまとめて記述（日本語で）",
            "impact": "この作業がなぜ重要だったか（日本語で）",
            "technical_details": "技術的な詳細（オプション、日本語で）"
        }}
    ],
    "achievements": ["具体的な成果のリスト（日本語で）"],
    "challenges": ["遭遇した問題や障害のリスト（日本語で）"],
    "insights": "重要な洞察やパターン（日本語で）"
}}"#
        ),
        "zh" => (
            "分析以下软件开发对话，并以JSON格式提供响应。所有文本字段使用简体中文。",
            r#"{{
    "project_title": "有意义的项目标题（中文）",
    "project_purpose": "该项目的主要目的（中文）",
    "main_activities": [
        {{
            "category": "开发/测试/配置/文档/修复错误等（中文）",
            "description": "实际完成的工作（中文）",
            "impact": "为什么这项工作很重要（中文）",
            "technical_details": "技术细节（可选，中文）"
        }}
    ],
    "achievements": ["具体成就列表（中文）"],
    "challenges": ["遇到的问题或障碍列表（中文）"],
    "insights": "重要见解或模式（中文）"
}}"#
        ),
        "ko" => (
            "다음 소프트웨어 개발 대화를 분석하고 JSON 형식으로 응답을 제공하십시오. 모든 텍스트 필드는 한국어로 작성하십시오.",
            r#"{{
    "project_title": "의미 있는 프로젝트 제목 (한국어)",
    "project_purpose": "이 프로젝트의 주요 목적 (한국어)",
    "main_activities": [
        {{
            "category": "개발/테스트/구성/문서/버그 수정 등 (한국어)",
            "description": "실제로 수행한 작업 (한국어)",
            "impact": "이 작업이 중요한 이유 (한국어)",
            "technical_details": "기술적 세부사항 (선택사항, 한국어)"
        }}
    ],
    "achievements": ["구체적인 성과 목록 (한국어)"],
    "challenges": ["직면한 문제나 장애물 목록 (한국어)"],
    "insights": "주요 통찰력이나 패턴 (한국어)"
}}"#
        ),
        "es" => (
            "Analiza la siguiente conversación de desarrollo de software y proporciona una respuesta JSON. Todos los campos de texto en español.",
            r#"{{
    "project_title": "Un título de proyecto significativo (en español)",
    "project_purpose": "El propósito principal de este proyecto (en español)",
    "main_activities": [
        {{
            "category": "Desarrollo/Pruebas/Configuración/Documentación/Corrección de errores/etc (en español)",
            "description": "Lo que se hizo realmente (en español)",
            "impact": "Por qué esto fue importante (en español)",
            "technical_details": "Contexto técnico opcional (en español)"
        }}
    ],
    "achievements": ["Lista de logros concretos (en español)"],
    "challenges": ["Lista de problemas o bloqueos encontrados (en español)"],
    "insights": "Perspectivas clave o patrones notados (en español)"
}}"#
        ),
        "fr" => (
            "Analysez la conversation de développement logiciel suivante et fournissez une réponse JSON. Tous les champs de texte en français.",
            r#"{{
    "project_title": "Un titre de projet significatif (en français)",
    "project_purpose": "L'objectif principal de ce projet (en français)",
    "main_activities": [
        {{
            "category": "Développement/Tests/Configuration/Documentation/Corrections de bugs/etc (en français)",
            "description": "Ce qui a été réellement fait (en français)",
            "impact": "Pourquoi c'était important (en français)",
            "technical_details": "Contexte technique optionnel (en français)"
        }}
    ],
    "achievements": ["Liste des réalisations concrètes (en français)"],
    "challenges": ["Liste des problèmes ou blocages rencontrés (en français)"],
    "insights": "Aperçus clés ou modèles remarqués (en français)"
}}"#
        ),
        _ => (
            "Analyze this software development conversation and provide a JSON response with the following structure:",
            r#"{{
    "project_title": "A meaningful project title (not a file path)",
    "project_purpose": "The main purpose of this project",
    "main_activities": [
        {{
            "category": "Development/Testing/Configuration/Documentation/Bug Fixes/etc",
            "description": "What was actually done (human-readable)",
            "impact": "Why this was important",
            "technical_details": "Optional technical context"
        }}
    ],
    "achievements": ["List of concrete accomplishments"],
    "challenges": ["List of issues or blockers encountered"],
    "insights": "Key insights or patterns noticed"
}}"#
        )
    }
}

fn create_analysis_prompt(conv: &ConversationData, lang: &str, templates: &Templates) -> String {
    let (instructions, structure) = get_prompt_templates(lang);

    // Use template if available, otherwise use default format
    templates
        .prompts
        .user_prompt
        .replace("{instructions}", instructions)
        .replace("{structure}", structure)
        .replace("{project_name}", &conv.project_name)
        .replace("{user_messages}", &conv.user_messages.join("\n---\n"))
        .replace("{assistant_actions}", &conv.assistant_actions.join("\n"))
}

// Alternative approach using local LLM or embedding-based clustering
pub mod semantic_analyzer {
    use super::*;

    pub async fn cluster_similar_tasks(
        _tasks: Vec<String>,
    ) -> Result<Vec<TaskCluster>, Box<dyn std::error::Error>> {
        // This could use sentence embeddings to group similar tasks
        // For now, returning a placeholder
        Ok(vec![])
    }

    #[derive(Debug, Serialize)]
    pub struct TaskCluster {
        pub theme: String,
        pub tasks: Vec<String>,
        pub summary: String,
    }
}

// Smart categorization using NLP
pub fn smart_categorize_activity(user_intent: &str, actions: &[String]) -> ActivityCategory {
    // This would use more sophisticated NLP techniques
    // For now, a simplified version
    ActivityCategory {
        primary: infer_primary_category(user_intent, actions),
        confidence: 0.8,
        reasoning: "AI-based inference".to_string(),
    }
}

#[derive(Debug, Serialize)]
pub struct ActivityCategory {
    pub primary: String,
    pub confidence: f32,
    pub reasoning: String,
}

fn infer_primary_category(_intent: &str, actions: &[String]) -> String {
    // This would use ML models in production
    // Placeholder logic for now
    if actions
        .iter()
        .any(|a| a.contains("test") || a.contains("verify"))
    {
        "Testing & Validation".to_string()
    } else if actions
        .iter()
        .any(|a| a.contains("implement") || a.contains("create"))
    {
        "Feature Development".to_string()
    } else if actions
        .iter()
        .any(|a| a.contains("fix") || a.contains("resolve"))
    {
        "Bug Fixing & Troubleshooting".to_string()
    } else {
        "General Development".to_string()
    }
}
