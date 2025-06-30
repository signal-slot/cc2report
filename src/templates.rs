use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Templates {
    pub prompts: PromptTemplates,
    pub report: ReportTemplates,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptTemplates {
    pub system_message: String,
    pub user_prompt: String,
    pub language_instructions: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportTemplates {
    pub header: String,
    pub summary_section: String,
    pub project_section: String,
    pub activity_item: String,
}

impl Default for Templates {
    fn default() -> Self {
        Self {
            prompts: PromptTemplates {
                system_message: "You are an expert at analyzing software development conversations and creating meaningful work reports. \
                                Analyze the conversation and provide a structured JSON summary focusing on what was actually accomplished, \
                                not just what tools were used. Respond with valid JSON only, without markdown code blocks or any other formatting. \
                                {language_instruction}".to_string(),
                user_prompt: "{instructions}\n\n{structure}\n\nProject: {project_name}\nUser Messages:\n{user_messages}\n\n\
                             Assistant Actions:\n{assistant_actions}\n\nFocus on:\n1. What the user was trying to accomplish (intent)\n\
                             2. What was actually achieved (results)\n3. Any problems encountered\n4. The business/technical value delivered\n\n\
                             Important rules:\n- Combine activities of the same category into a single entry\n\
                             - Each category should appear only once\n- If multiple activities belong to the same category, summarize them together".to_string(),
                language_instructions: HashMap::new(),
            },
            report: ReportTemplates {
                header: "# {title} - {date}\n\n".to_string(),
                summary_section: "## {section_title}\n\n- **{total_items_label}**: {total_items}\n\
                                 - **{completion_label}**: {completion_rate}%\n- **{focus_areas_label}**:\n{focus_areas}\n\n\
                                 **{analysis_label}**: {analysis}\n\n".to_string(),
                project_section: "### {index}. {title}\n\n**{purpose_label}**: {purpose}\n\n".to_string(),
                activity_item: "- {category} - {description}\n".to_string(),
            },
        }
    }
}

impl Templates {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let templates: Templates = toml::from_str(&content)?;
        Ok(templates)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn merge_with_defaults(custom: Option<Templates>) -> Self {
        match custom {
            Some(mut custom_templates) => {
                let defaults = Self::default();

                // Fill in missing language instructions
                for (lang, instruction) in defaults.prompts.language_instructions {
                    custom_templates
                        .prompts
                        .language_instructions
                        .entry(lang)
                        .or_insert(instruction);
                }

                custom_templates
            }
            None => Self::default(),
        }
    }
}

pub fn get_template_path() -> Option<PathBuf> {
    // Check local config first
    if Path::new("cc2report.toml").exists() {
        return Some(PathBuf::from("cc2report.toml"));
    }

    // Check user config directory
    if let Ok(home) = std::env::var("HOME") {
        let config_path = Path::new(&home)
            .join(".config")
            .join("cc2report")
            .join("templates.toml");
        if config_path.exists() {
            return Some(config_path);
        }
    }

    None
}

pub fn create_default_template_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let defaults = Templates::default();

    // Add all language instructions
    let mut templates = defaults;
    templates.prompts.language_instructions.insert(
        "ja".to_string(),
        "Use Japanese for all text fields.".to_string(),
    );
    templates.prompts.language_instructions.insert(
        "en".to_string(),
        "Use English for all text fields.".to_string(),
    );
    templates.prompts.language_instructions.insert(
        "zh".to_string(),
        "Use Simplified Chinese (简体中文) for all text fields.".to_string(),
    );
    templates.prompts.language_instructions.insert(
        "ko".to_string(),
        "Use Korean (한국어) for all text fields.".to_string(),
    );
    templates.prompts.language_instructions.insert(
        "es".to_string(),
        "Use Spanish for all text fields.".to_string(),
    );
    templates.prompts.language_instructions.insert(
        "fr".to_string(),
        "Use French for all text fields.".to_string(),
    );

    templates.save_to_file(path)?;
    Ok(())
}

use std::path::PathBuf;
