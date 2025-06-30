use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub cache: CacheConfig,
    pub output: OutputConfig,
    pub processing: ProcessingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub openai_api_key: Option<String>,
    pub model: String,
    pub temperature: f32,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub cache_dir: PathBuf,
    pub ttl_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub language: String,
    pub show_token_usage: bool,
    pub quiet: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Markdown,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    pub parallel_requests: usize,
    pub log_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                openai_api_key: None,
                model: "gpt-4o".to_string(),
                temperature: 0.3,
                max_retries: 3,
                retry_delay_ms: 1000,
            },
            cache: CacheConfig {
                enabled: true,
                cache_dir: default_cache_dir(),
                ttl_hours: 24,
            },
            output: OutputConfig {
                format: OutputFormat::Markdown,
                language: detect_system_language(),
                show_token_usage: false,
                quiet: false,
            },
            processing: ProcessingConfig {
                parallel_requests: 1,
                log_dir: default_log_dir(),
            },
        }
    }
}

impl Config {
    /// Create config from command line arguments
    pub fn from_args(matches: &clap::ArgMatches) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();

        // API configuration
        if let Some(key) = matches.get_one::<String>("api-key") {
            config.api.openai_api_key = Some(key.clone());
        } else if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            config.api.openai_api_key = Some(key);
        }

        if let Some(model) = matches.get_one::<String>("model") {
            config.api.model = model.clone();
        }

        // Cache configuration is always enabled by default
        // Users can clear cache with --clear-cache if needed

        // Output configuration
        if let Some(format) = matches.get_one::<String>("format") {
            config.output.format = match format.as_str() {
                "json" => OutputFormat::Json,
                _ => OutputFormat::Markdown,
            };
        }

        if let Some(lang) = matches.get_one::<String>("lang") {
            config.output.language = lang.clone();
        }

        config.output.show_token_usage = matches.get_flag("show-token-usage");
        config.output.quiet = matches.get_flag("quiet");

        // Processing configuration
        if let Some(parallel) = matches.get_one::<usize>("parallel") {
            config.processing.parallel_requests = (*parallel).min(10);
        }

        if let Some(log_dir) = matches.get_one::<String>("log-dir") {
            config.processing.log_dir = PathBuf::from(log_dir);
        }

        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.api.openai_api_key.is_none() {
            return Err("OpenAI API key is required. Set OPENAI_API_KEY environment variable or use --api-key option.".into());
        }

        if !self.processing.log_dir.exists() {
            return Err(format!(
                "Log directory does not exist: {}",
                self.processing.log_dir.display()
            )
            .into());
        }

        Ok(())
    }
}

fn default_cache_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".cache").join("cc2report")
    } else {
        PathBuf::from(".cc2report-cache")
    }
}

fn default_log_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".claude").join("projects")
    } else {
        PathBuf::from(".")
    }
}

fn detect_system_language() -> String {
    if let Ok(lang) = std::env::var("LANG") {
        let lang_code = lang.split('.').next().unwrap_or(&lang);
        let primary_lang = lang_code.split('_').next().unwrap_or(lang_code);

        match primary_lang {
            "ja" => "ja",
            "zh" => "zh",
            "ko" => "ko",
            "es" => "es",
            "fr" => "fr",
            "de" => "de",
            "pt" => "pt",
            "ru" => "ru",
            "it" => "it",
            "nl" => "nl",
            "pl" => "pl",
            "tr" => "tr",
            "ar" => "ar",
            "hi" => "hi",
            "th" => "th",
            "vi" => "vi",
            "id" => "id",
            "ms" => "ms",
            _ => "en",
        }
        .to_string()
    } else {
        "en".to_string()
    }
}
