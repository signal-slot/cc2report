use crate::ai_analyzer::*;
use crate::cache::ApiCache;
use crate::templates::{get_template_path, Templates};
use crate::token_tracker::{TokenTracker, TokenUsage};
use futures::stream::{self, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn analyze_with_ai_parallel(
    api_key: &str,
    conversations: Vec<ConversationData>,
    lang: &str,
    model: &str,
    quiet: bool,
    parallel: usize,
) -> Result<(HashMap<String, AiAnalysisResponse>, TokenTracker), Box<dyn std::error::Error>> {
    let client = Arc::new(reqwest::Client::new());
    let results = Arc::new(Mutex::new(HashMap::new()));
    let token_tracker = Arc::new(Mutex::new(TokenTracker::new()));

    // Initialize cache
    let cache = match ApiCache::new() {
        Ok(c) => Some(Arc::new(c)),
        Err(e) => {
            eprintln!("Cache initialization error: {e}. Continuing without cache.");
            None
        }
    };

    // Load templates
    let templates = Arc::new(if let Some(template_path) = get_template_path() {
        match Templates::load_from_file(&template_path) {
            Ok(t) => Templates::merge_with_defaults(Some(t)),
            Err(e) => {
                eprintln!("Template loading error: {e}. Using defaults.");
                Templates::default()
            }
        }
    } else {
        Templates::default()
    });

    // Create multi-progress for parallel progress bars
    let multi_progress = if quiet {
        None
    } else {
        Some(Arc::new(MultiProgress::new()))
    };

    // Create main progress bar
    let main_pb = if let Some(ref mp) = multi_progress {
        let pb = mp.add(ProgressBar::new(conversations.len() as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} Analyzing {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        pb.set_message("projects with AI...");
        Some(pb)
    } else {
        None
    };

    // Process conversations in parallel
    let parallel = parallel.max(1);
    let chunks: Vec<Vec<ConversationData>> = conversations
        .chunks(conversations.len().max(1) / parallel + 1)
        .map(|chunk| chunk.to_vec())
        .collect();

    let api_key = Arc::new(api_key.to_string());
    let lang = Arc::new(lang.to_string());
    let model = Arc::new(model.to_string());

    let futures = chunks.into_iter().map(|chunk| {
        let client = Arc::clone(&client);
        let results = Arc::clone(&results);
        let token_tracker = Arc::clone(&token_tracker);
        let cache = cache.clone();
        let templates = Arc::clone(&templates);
        let api_key = Arc::clone(&api_key);
        let lang = Arc::clone(&lang);
        let model = Arc::clone(&model);
        let main_pb = main_pb.clone();

        async move {
            for conv in chunk {
                // Check cache first
                let mut use_cache = false;
                if let Some(ref cache) = cache {
                    let cache_key_messages = vec![
                        conv.user_messages.join("\n"),
                        conv.assistant_actions.join("\n"),
                    ];

                    if let Some(cached_response) =
                        cache.get(&conv.project_name, &lang, &model, &cache_key_messages)
                    {
                        results
                            .lock()
                            .unwrap()
                            .insert(conv.project_name.clone(), cached_response);
                        if let Some(ref pb) = main_pb {
                            pb.inc(1);
                        }
                        use_cache = true;
                    }
                }

                if use_cache {
                    continue;
                }

                // Process API call
                let prompt = create_analysis_prompt(&conv, &lang, &templates);

                let default_instruction = get_language_instruction(&lang).to_string();
                let language_instruction = templates
                    .prompts
                    .language_instructions
                    .get(lang.as_str())
                    .unwrap_or(&default_instruction);

                let system_message = templates
                    .prompts
                    .system_message
                    .replace("{language_instruction}", language_instruction);

                let model_name = match model.as_str() {
                    "gpt-4o" => "chatgpt-4o-latest",
                    "gpt-4o-mini" => "gpt-4o-mini",
                    "gpt-4-turbo" => "gpt-4-turbo-preview",
                    "gpt-3.5-turbo" => "gpt-3.5-turbo",
                    _ => model.as_str(),
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
                    .header("Authorization", format!("Bearer {api_key}"))
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
                                        } else if content.starts_with("```")
                                            && content.ends_with("```")
                                        {
                                            content
                                                .trim_start_matches("```")
                                                .trim_end_matches("```")
                                                .trim()
                                        } else {
                                            content
                                        };

                                        match serde_json::from_str::<AiAnalysisResponse>(
                                            cleaned_content,
                                        ) {
                                            Ok(analysis) => {
                                                // Track token usage
                                                if let Some(usage) =
                                                    api_response["usage"].as_object()
                                                {
                                                    let token_usage = TokenUsage {
                                                        prompt_tokens: usage["prompt_tokens"]
                                                            .as_u64()
                                                            .unwrap_or(0)
                                                            as u32,
                                                        completion_tokens: usage
                                                            ["completion_tokens"]
                                                            .as_u64()
                                                            .unwrap_or(0)
                                                            as u32,
                                                        total_tokens: usage["total_tokens"]
                                                            .as_u64()
                                                            .unwrap_or(0)
                                                            as u32,
                                                    };
                                                    token_tracker
                                                        .lock()
                                                        .unwrap()
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
                                                        &lang,
                                                        &model,
                                                        &cache_key_messages,
                                                        &analysis,
                                                    ) {
                                                        eprintln!("Cache save error: {e}");
                                                    }
                                                }

                                                results
                                                    .lock()
                                                    .unwrap()
                                                    .insert(conv.project_name.clone(), analysis);
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "JSON parsing error for project '{}': {}",
                                                    conv.project_name, e
                                                );
                                                eprintln!("Invalid JSON: {cleaned_content}");
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

                if let Some(ref pb) = main_pb {
                    pb.inc(1);
                }
            }
        }
    });

    // Execute all futures concurrently
    let _: Vec<_> = stream::iter(futures)
        .buffer_unordered(parallel)
        .collect()
        .await;

    if let Some(ref pb) = main_pb {
        pb.finish_with_message("AI analysis complete!");
    }

    // Extract results
    let final_results = match Arc::try_unwrap(results) {
        Ok(mutex) => mutex.into_inner().unwrap(),
        Err(arc) => arc.lock().unwrap().clone(),
    };

    let mut final_tracker = match Arc::try_unwrap(token_tracker) {
        Ok(mutex) => mutex.into_inner().unwrap(),
        Err(arc) => arc.lock().unwrap().clone(),
    };

    // Calculate cost estimate
    final_tracker.calculate_cost(model.as_str());

    Ok((final_results, final_tracker))
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
