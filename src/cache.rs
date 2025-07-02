use crate::ai_analyzer::AiAnalysisResponse;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
struct CacheEntry {
    response: AiAnalysisResponse,
    timestamp: SystemTime,
}

pub struct ApiCache {
    cache_dir: PathBuf,
    ttl: Duration,
}

impl ApiCache {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let cache_dir = Self::get_cache_dir()?;
        fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            cache_dir,
            ttl: Duration::from_secs(86400), // 24 hours default TTL
        })
    }

    pub fn with_ttl(ttl_hours: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cache = Self::new()?;
        cache.ttl = Duration::from_secs(ttl_hours * 3600);
        Ok(cache)
    }

    fn get_cache_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Ok(home) = std::env::var("HOME") {
            Ok(PathBuf::from(home).join(".cache").join("cc2report"))
        } else {
            Ok(PathBuf::from(".cc2report-cache"))
        }
    }

    fn get_cache_key(project_name: &str, lang: &str, model: &str, messages: &[String]) -> String {
        let mut hasher = DefaultHasher::new();
        project_name.hash(&mut hasher);
        lang.hash(&mut hasher);
        model.hash(&mut hasher);
        for msg in messages {
            msg.hash(&mut hasher);
        }
        format!("{:x}", hasher.finish())
    }

    pub fn get(
        &self,
        project_name: &str,
        lang: &str,
        model: &str,
        messages: &[String],
    ) -> Option<AiAnalysisResponse> {
        let key = Self::get_cache_key(project_name, lang, model, messages);
        let cache_file = self.cache_dir.join(format!("{key}.json"));

        if !cache_file.exists() {
            return None;
        }

        match fs::read_to_string(&cache_file) {
            Ok(content) => {
                match serde_json::from_str::<CacheEntry>(&content) {
                    Ok(entry) => {
                        // Check if cache is still valid
                        if let Ok(elapsed) = entry.timestamp.elapsed() {
                            if elapsed < self.ttl {
                                eprintln!(
                                    "Cache hit: {} (TTL: {:?})",
                                    project_name,
                                    self.ttl - elapsed
                                );
                                return Some(entry.response);
                            } else {
                                eprintln!("Cache expired: {project_name}");
                                // Remove expired cache
                                let _ = fs::remove_file(&cache_file);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Cache read error: {e}");
                    }
                }
            }
            Err(e) => {
                eprintln!("Cache file read error: {e}");
            }
        }

        None
    }

    pub fn set(
        &self,
        project_name: &str,
        lang: &str,
        model: &str,
        messages: &[String],
        response: &AiAnalysisResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = Self::get_cache_key(project_name, lang, model, messages);
        let cache_file = self.cache_dir.join(format!("{key}.json"));

        let entry = CacheEntry {
            response: response.clone(),
            timestamp: SystemTime::now(),
        };

        let content = serde_json::to_string_pretty(&entry)?;
        fs::write(&cache_file, content)?;

        eprintln!("Saved to cache: {project_name}");
        Ok(())
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                fs::remove_file(entry.path())?;
            }
        }
        eprintln!("Cache cleared");
        Ok(())
    }

    pub fn size(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut total_size = 0;
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                total_size += entry.metadata()?.len();
            }
        }
        Ok(total_size)
    }
}
