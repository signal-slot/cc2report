use crate::parser::Message;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ConversationFlow {
    pub current_topic: Option<Topic>,
    pub topics: Vec<Topic>,
    pub context_stack: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Topic {
    pub title: String,
    pub user_intent: String,
    pub steps: Vec<WorkStep>,
    pub outcome: TopicOutcome,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkStep {
    pub description: String,
    pub details: Vec<String>,
    pub result: StepResult,
}

#[derive(Debug, Clone, Serialize)]
pub enum StepResult {
    Success(String),
    Failed(String),
    InProgress,
}

#[derive(Debug, Clone, Serialize)]
pub enum TopicOutcome {
    Completed(String),
    PartiallyCompleted(String),
    Failed(String),
    InProgress,
}

impl ConversationFlow {
    pub fn new() -> Self {
        Self {
            current_topic: None,
            topics: Vec::new(),
            context_stack: Vec::new(),
        }
    }

    pub fn analyze_user_message(&mut self, content: &str, timestamp: &str) {
        // ユーザーの意図を理解する
        if let Some(intent) = extract_user_intent(content) {
            // 現在のトピックを完了させる
            if let Some(mut topic) = self.current_topic.take() {
                topic.completed_at = Some(timestamp.to_string());
                topic.outcome = determine_topic_outcome(&topic);
                self.topics.push(topic);
            }

            // 新しいトピックを開始
            self.current_topic = Some(Topic {
                title: generate_topic_title(&intent),
                user_intent: intent,
                steps: Vec::new(),
                outcome: TopicOutcome::InProgress,
                started_at: timestamp.to_string(),
                completed_at: None,
                timestamp: timestamp.to_string(),
            });
        }
    }

    pub fn analyze_assistant_response(&mut self, message: &Message) {
        if let Some(topic) = &mut self.current_topic {
            // アシスタントの応答から実際の作業内容を抽出
            if let Some(work_steps) = extract_work_steps(message) {
                topic.steps.extend(work_steps);
            }
        }
    }

    pub fn finalize(&mut self) {
        if let Some(mut topic) = self.current_topic.take() {
            topic.outcome = determine_topic_outcome(&topic);
            self.topics.push(topic);
        }
    }
}

fn extract_user_intent(content: &str) -> Option<String> {
    // 日本語と英語の両方に対応
    let _content_lower = content.to_lowercase();

    // 疑問文や依頼文のパターンを検出
    if content.contains("？")
        || content.contains("?")
        || content.contains("してください")
        || content.contains("して欲しい")
        || content.contains("したい")
        || content.contains("please")
        || content.contains("help")
        || content.contains("can you")
        || content.contains("create")
        || content.contains("implement")
        || content.contains("fix")
        || content.contains("作って")
        || content.contains("直して")
        || content.contains("実装")
    {
        // 文章を要約して意図を抽出
        Some(summarize_intent(content))
    } else {
        None
    }
}

fn summarize_intent(content: &str) -> String {
    // 長い文章を要約
    let sentences: Vec<&str> = content.split(&['.', '。', '!', '！'][..]).collect();
    if let Some(first) = sentences.first() {
        let trimmed = first.trim();
        if trimmed.chars().count() > 50 {
            let chars: Vec<char> = trimmed.chars().take(50).collect();
            format!("{}...", chars.into_iter().collect::<String>())
        } else {
            trimmed.to_string()
        }
    } else {
        content.chars().take(50).collect()
    }
}

fn generate_topic_title(intent: &str) -> String {
    // 意図から簡潔なタイトルを生成
    let intent_lower = intent.to_lowercase();

    // 日本語の場合
    if intent.contains("作")
        || intent.contains("実装")
        || intent.contains("修正")
        || intent.contains("追加")
        || intent.contains("改善")
        || intent.contains("解決")
    {
        // 主要な動詞と目的語を抽出
        if let Some(first_sentence) = intent.split(&['。', '、', '\n'][..]).next() {
            let trimmed = first_sentence.trim();
            if trimmed.chars().count() <= 30 {
                return trimmed.to_string();
            } else {
                let chars: Vec<char> = trimmed.chars().take(30).collect();
                return format!("{}...", chars.into_iter().collect::<String>());
            }
        }
    }

    // 英語の場合 - 主要な動詞を探す
    let action_words = [
        "create",
        "implement",
        "fix",
        "add",
        "update",
        "build",
        "develop",
        "analyze",
        "generate",
        "test",
        "debug",
        "refactor",
        "optimize",
        "setup",
        "configure",
        "install",
        "deploy",
        "write",
        "design",
    ];

    for action in &action_words {
        if intent_lower.contains(action) {
            // 動詞の前後の文脈を含める
            if let Some(pos) = intent_lower.find(action) {
                let start = pos.saturating_sub(10);
                let end = (pos + action.len() + 30).min(intent.len());
                let context = &intent[start..end];

                // 文の境界で切る
                if let Some(sentence_end) = context.find(&['.', '!', '?'][..]) {
                    return context[..sentence_end].trim().to_string();
                }
                return context.trim().to_string();
            }
        }
    }

    // デフォルト：最初の30文字
    if intent.chars().count() > 30 {
        let chars: Vec<char> = intent.chars().take(30).collect();
        format!("{}...", chars.into_iter().collect::<String>())
    } else {
        intent.to_string()
    }
}

fn extract_work_steps(message: &Message) -> Option<Vec<WorkStep>> {
    let mut steps = Vec::new();

    if let Some(content) = &message.content {
        if let Some(array) = content.as_array() {
            let mut current_step: Option<WorkStep> = None;

            for item in array {
                if let Some(obj) = item.as_object() {
                    match obj.get("type").and_then(|v| v.as_str()) {
                        Some("text") => {
                            if let Some(text) = obj.get("text").and_then(|v| v.as_str()) {
                                // テキストから作業内容を抽出
                                if let Some(step_desc) = extract_meaningful_action(text) {
                                    if let Some(step) = current_step.take() {
                                        steps.push(step);
                                    }
                                    current_step = Some(WorkStep {
                                        description: step_desc,
                                        details: Vec::new(),
                                        result: StepResult::InProgress,
                                    });
                                }
                            }
                        }
                        Some("tool_use") => {
                            if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                                if let Some(input) = obj.get("input") {
                                    // ツール使用を人間が理解できる形に変換
                                    let human_readable =
                                        convert_tool_use_to_human_readable(name, input);

                                    if let Some(step) = &mut current_step {
                                        step.details.push(human_readable);
                                    } else {
                                        current_step = Some(WorkStep {
                                            description: "作業を実行中".to_string(),
                                            details: vec![human_readable],
                                            result: StepResult::InProgress,
                                        });
                                    }
                                }
                            }
                        }
                        Some("tool_result") => {
                            if let Some(step) = &mut current_step {
                                // 結果を解析
                                if let Some(output) = obj.get("output") {
                                    step.result = analyze_tool_result(output);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            if let Some(step) = current_step {
                steps.push(step);
            }
        }
    }

    if steps.is_empty() {
        None
    } else {
        Some(steps)
    }
}

fn extract_meaningful_action(text: &str) -> Option<String> {
    // "I'll help you..." のような定型文を除外
    let ignore_patterns = [
        "i'll",
        "i will",
        "let me",
        "i'm going to",
        "i need to",
        "let's",
        "お手伝い",
        "させていただき",
        "しましょう",
        "します",
        "ですね",
    ];

    let text_lower = text.to_lowercase();

    // 短すぎるテキストは除外
    if text.len() < 10 {
        return None;
    }

    // 定型文で始まる場合はスキップ
    if ignore_patterns
        .iter()
        .any(|&pattern| text_lower.starts_with(pattern))
    {
        // ただし、文の後半に重要な内容がある場合は抽出
        let sentences: Vec<&str> = text.split(&['.', '。', ':', '：'][..]).collect();
        if sentences.len() > 1 {
            if let Some(second) = sentences.get(1) {
                let trimmed = second.trim();
                if trimmed.len() > 10 {
                    return Some(trimmed.to_string());
                }
            }
        }
    }

    // 意味のある作業を表す文を探す
    let meaningful_patterns = [
        // 英語のパターン
        "create",
        "implement",
        "fix",
        "add",
        "update",
        "modify",
        "build",
        "analyze",
        "generate",
        "test",
        "debug",
        "refactor",
        "optimize",
        "write",
        "design",
        "plan",
        "search",
        "find",
        "check",
        "verify",
        "install",
        "configure",
        "setup",
        "deploy",
        "run",
        "execute",
        "change",
        "improve",
        "enhance",
        "resolve",
        "solve",
        "handle",
        // 日本語のパターン
        "作成",
        "実装",
        "修正",
        "追加",
        "更新",
        "変更",
        "ビルド",
        "分析",
        "生成",
        "テスト",
        "デバッグ",
        "リファクタリング",
        "最適化",
        "記述",
        "設計",
        "計画",
        "検索",
        "確認",
        "検証",
        "インストール",
        "設定",
        "セットアップ",
        "デプロイ",
        "実行",
        "改善",
        "解決",
        "対応",
        "処理",
    ];

    // 最初の意味のある文を探す
    let sentences: Vec<&str> = text.split(&['.', '。', '\n'][..]).collect();
    for sentence in sentences {
        let trimmed = sentence.trim();
        let sentence_lower = trimmed.to_lowercase();

        if meaningful_patterns
            .iter()
            .any(|&pattern| sentence_lower.contains(pattern))
        {
            // 文が長すぎる場合は要約
            if trimmed.chars().count() > 80 {
                let chars: Vec<char> = trimmed.chars().take(80).collect();
                return Some(format!("{}...", chars.into_iter().collect::<String>()));
            }
            return Some(trimmed.to_string());
        }
    }

    // パターンに一致しない場合でも、コロンの後の内容は重要な可能性が高い
    if text.contains(':') || text.contains('：') {
        let parts: Vec<&str> = text.split(&[':', '：'][..]).collect();
        if parts.len() > 1 {
            let after_colon = parts[1].trim();
            if after_colon.len() > 10 && after_colon.len() < 200 {
                return Some(after_colon.to_string());
            }
        }
    }

    None
}

fn convert_tool_use_to_human_readable(tool_name: &str, input: &serde_json::Value) -> String {
    match tool_name {
        "Read" => {
            if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                let filename = path.split('/').last().unwrap_or(path);
                format!("{} を読み込み", filename)
            } else {
                "ファイルを読み込み".to_string()
            }
        }
        "Write" => {
            if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                let filename = path.split('/').last().unwrap_or(path);
                format!("{} を新規作成", filename)
            } else {
                "新規ファイルを作成".to_string()
            }
        }
        "Edit" | "MultiEdit" => {
            if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                let filename = path.split('/').last().unwrap_or(path);
                format!("{} を編集", filename)
            } else {
                "ファイルを編集".to_string()
            }
        }
        "Bash" => {
            if let Some(cmd) = input.get("command").and_then(|v| v.as_str()) {
                let cmd_parts: Vec<&str> = cmd.split_whitespace().collect();
                match cmd_parts.first() {
                    Some(&"cargo") if cmd_parts.get(1) == Some(&"build") => {
                        "Rustプロジェクトをビルド".to_string()
                    }
                    Some(&"cargo") if cmd_parts.get(1) == Some(&"test") => {
                        "Rustのテストを実行".to_string()
                    }
                    Some(&"cargo") if cmd_parts.get(1) == Some(&"run") => {
                        "プログラムを実行".to_string()
                    }
                    Some(&"git") if cmd_parts.get(1) == Some(&"commit") => {
                        "変更をGitにコミット".to_string()
                    }
                    Some(&"git") if cmd_parts.get(1) == Some(&"status") => {
                        "Gitステータスを確認".to_string()
                    }
                    Some(&"npm") if cmd_parts.get(1) == Some(&"install") => {
                        "npm依存関係をインストール".to_string()
                    }
                    Some(&"echo") => "メッセージを出力".to_string(),
                    Some(&"mkdir") => "ディレクトリを作成".to_string(),
                    _ => {
                        if cmd.len() > 30 {
                            format!("{} コマンドを実行", cmd_parts.first().unwrap_or(&""))
                        } else {
                            format!("「{}」を実行", cmd)
                        }
                    }
                }
            } else {
                "コマンドを実行".to_string()
            }
        }
        "Grep" => {
            if let Some(pattern) = input.get("pattern").and_then(|v| v.as_str()) {
                let pattern_display = if pattern.chars().count() > 20 {
                    let truncated: String = pattern.chars().take(20).collect();
                    format!("{}...", truncated)
                } else {
                    pattern.to_string()
                };
                format!("「{}」を検索", pattern_display)
            } else {
                "コード内を検索".to_string()
            }
        }
        "TodoWrite" => "TODOリストを更新".to_string(),
        "TodoRead" => "TODOリストを確認".to_string(),
        _ => format!("{} ツールを使用", tool_name),
    }
}

fn analyze_tool_result(output: &serde_json::Value) -> StepResult {
    if let Some(output_str) = output.as_str() {
        if output_str.contains("error")
            || output_str.contains("Error")
            || output_str.contains("failed")
            || output_str.contains("Failed")
        {
            StepResult::Failed(extract_error_summary(output_str))
        } else {
            StepResult::Success("完了".to_string())
        }
    } else if let Some(obj) = output.as_object() {
        if obj.contains_key("error") {
            StepResult::Failed("エラーが発生".to_string())
        } else {
            StepResult::Success("完了".to_string())
        }
    } else {
        StepResult::Success("完了".to_string())
    }
}

fn extract_error_summary(error_text: &str) -> String {
    let lines: Vec<&str> = error_text.lines().collect();
    if let Some(first_error_line) = lines
        .iter()
        .find(|line| line.contains("error") || line.contains("Error"))
    {
        first_error_line.trim().to_string()
    } else {
        "エラーが発生しました".to_string()
    }
}

fn determine_topic_outcome(topic: &Topic) -> TopicOutcome {
    let total_steps = topic.steps.len();
    if total_steps == 0 {
        return TopicOutcome::InProgress;
    }

    let (success_count, failed_count) =
        topic
            .steps
            .iter()
            .fold((0, 0), |(succ, fail), step| match &step.result {
                StepResult::Success(_) => (succ + 1, fail),
                StepResult::Failed(_) => (succ, fail + 1),
                StepResult::InProgress => (succ, fail),
            });

    if failed_count > 0 && success_count == 0 {
        TopicOutcome::Failed("すべての作業が失敗しました".to_string())
    } else if failed_count > 0 {
        TopicOutcome::PartiallyCompleted(format!("{}/{}の作業が完了", success_count, total_steps))
    } else if success_count == total_steps {
        TopicOutcome::Completed("すべての作業が正常に完了しました".to_string())
    } else {
        TopicOutcome::InProgress
    }
}
