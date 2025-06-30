use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CostEstimate {
    pub model: String,
    pub input_cost_per_1k: f64,
    pub output_cost_per_1k: f64,
    pub total_cost: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenTracker {
    pub project_usage: HashMap<String, TokenUsage>,
    pub total_usage: TokenUsage,
    pub cost_estimate: Option<CostEstimate>,
}

impl TokenTracker {
    pub fn new() -> Self {
        Self {
            project_usage: HashMap::new(),
            total_usage: TokenUsage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
            cost_estimate: None,
        }
    }

    pub fn add_usage(&mut self, project_name: &str, usage: TokenUsage) {
        // Update project-specific usage
        let project_usage = self
            .project_usage
            .entry(project_name.to_string())
            .or_insert(TokenUsage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            });

        project_usage.prompt_tokens += usage.prompt_tokens;
        project_usage.completion_tokens += usage.completion_tokens;
        project_usage.total_tokens += usage.total_tokens;

        // Update total usage
        self.total_usage.prompt_tokens += usage.prompt_tokens;
        self.total_usage.completion_tokens += usage.completion_tokens;
        self.total_usage.total_tokens += usage.total_tokens;
    }

    pub fn calculate_cost(&mut self, model: &str) {
        let (input_cost, output_cost) = get_model_pricing(model);

        let total_input_cost = (self.total_usage.prompt_tokens as f64 / 1000.0) * input_cost;
        let total_output_cost = (self.total_usage.completion_tokens as f64 / 1000.0) * output_cost;
        let total_cost = total_input_cost + total_output_cost;

        self.cost_estimate = Some(CostEstimate {
            model: model.to_string(),
            input_cost_per_1k: input_cost,
            output_cost_per_1k: output_cost,
            total_cost,
        });
    }

    pub fn get_summary_string(&self, lang: &str) -> String {
        let mut summary = String::new();

        match lang {
            "ja" => {
                summary.push_str("\n## トークン使用状況\n\n");
                summary.push_str(&format!("### 総使用量\n"));
                summary.push_str(&format!(
                    "- 入力トークン: {}\n",
                    self.total_usage.prompt_tokens
                ));
                summary.push_str(&format!(
                    "- 出力トークン: {}\n",
                    self.total_usage.completion_tokens
                ));
                summary.push_str(&format!(
                    "- 合計トークン: {}\n\n",
                    self.total_usage.total_tokens
                ));

                if let Some(ref cost) = self.cost_estimate {
                    summary.push_str(&format!("### コスト見積もり ({})\n", cost.model));
                    summary.push_str(&format!(
                        "- 入力コスト: ${:.4} (${:.2}/1K tokens)\n",
                        (self.total_usage.prompt_tokens as f64 / 1000.0) * cost.input_cost_per_1k,
                        cost.input_cost_per_1k
                    ));
                    summary.push_str(&format!(
                        "- 出力コスト: ${:.4} (${:.2}/1K tokens)\n",
                        (self.total_usage.completion_tokens as f64 / 1000.0)
                            * cost.output_cost_per_1k,
                        cost.output_cost_per_1k
                    ));
                    summary.push_str(&format!("- **合計コスト: ${:.4}**\n\n", cost.total_cost));
                }

                if !self.project_usage.is_empty() {
                    summary.push_str("### プロジェクト別使用量\n");
                    let mut projects: Vec<_> = self.project_usage.iter().collect();
                    projects.sort_by(|a, b| b.1.total_tokens.cmp(&a.1.total_tokens));

                    for (project, usage) in projects.iter().take(10) {
                        summary
                            .push_str(&format!("- {}: {} tokens\n", project, usage.total_tokens));
                    }
                }
            }
            "zh" => {
                summary.push_str("\n## 令牌使用情况\n\n");
                summary.push_str(&format!("### 总使用量\n"));
                summary.push_str(&format!("- 输入令牌: {}\n", self.total_usage.prompt_tokens));
                summary.push_str(&format!(
                    "- 输出令牌: {}\n",
                    self.total_usage.completion_tokens
                ));
                summary.push_str(&format!("- 总令牌: {}\n\n", self.total_usage.total_tokens));

                if let Some(ref cost) = self.cost_estimate {
                    summary.push_str(&format!("### 成本估算 ({})\n", cost.model));
                    summary.push_str(&format!(
                        "- 输入成本: ${:.4}\n",
                        (self.total_usage.prompt_tokens as f64 / 1000.0) * cost.input_cost_per_1k
                    ));
                    summary.push_str(&format!(
                        "- 输出成本: ${:.4}\n",
                        (self.total_usage.completion_tokens as f64 / 1000.0)
                            * cost.output_cost_per_1k
                    ));
                    summary.push_str(&format!("- **总成本: ${:.4}**\n", cost.total_cost));
                }
            }
            _ => {
                summary.push_str("\n## Token Usage Summary\n\n");
                summary.push_str(&format!("### Total Usage\n"));
                summary.push_str(&format!(
                    "- Input tokens: {}\n",
                    self.total_usage.prompt_tokens
                ));
                summary.push_str(&format!(
                    "- Output tokens: {}\n",
                    self.total_usage.completion_tokens
                ));
                summary.push_str(&format!(
                    "- Total tokens: {}\n\n",
                    self.total_usage.total_tokens
                ));

                if let Some(ref cost) = self.cost_estimate {
                    summary.push_str(&format!("### Cost Estimate ({})\n", cost.model));
                    summary.push_str(&format!(
                        "- Input cost: ${:.4} (${:.2}/1K tokens)\n",
                        (self.total_usage.prompt_tokens as f64 / 1000.0) * cost.input_cost_per_1k,
                        cost.input_cost_per_1k
                    ));
                    summary.push_str(&format!(
                        "- Output cost: ${:.4} (${:.2}/1K tokens)\n",
                        (self.total_usage.completion_tokens as f64 / 1000.0)
                            * cost.output_cost_per_1k,
                        cost.output_cost_per_1k
                    ));
                    summary.push_str(&format!("- **Total cost: ${:.4}**\n\n", cost.total_cost));
                }

                if !self.project_usage.is_empty() {
                    summary.push_str("### Usage by Project\n");
                    let mut projects: Vec<_> = self.project_usage.iter().collect();
                    projects.sort_by(|a, b| b.1.total_tokens.cmp(&a.1.total_tokens));

                    for (project, usage) in projects.iter().take(10) {
                        summary
                            .push_str(&format!("- {}: {} tokens\n", project, usage.total_tokens));
                    }
                }
            }
        }

        summary
    }
}

fn get_model_pricing(model: &str) -> (f64, f64) {
    // Pricing as of 2024 (in USD per 1K tokens)
    // (input_cost, output_cost)
    match model {
        "gpt-4o" | "chatgpt-4o-latest" => (0.0025, 0.01), // $2.50/$10.00 per 1M
        "gpt-4o-mini" => (0.00015, 0.0006),               // $0.15/$0.60 per 1M
        "gpt-4-turbo" | "gpt-4-turbo-preview" => (0.01, 0.03), // $10/$30 per 1M
        "gpt-3.5-turbo" => (0.0005, 0.0015),              // $0.50/$1.50 per 1M
        _ => (0.0025, 0.01),                              // Default to gpt-4o pricing
    }
}
