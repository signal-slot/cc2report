use cc2report::{
    cache, cli,
    config::{Config, OutputFormat},
    error::{ApiError, AppError, Result},
    logger, smart_analyzer, templates,
    work_report_v2::analyze_conversations_with_ai,
};
use chrono::{Datelike, NaiveDate};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

async fn run() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    // Initialize logger
    let quiet = matches.get_flag("quiet");
    logger::init(logger::LogLevel::Info, quiet);

    // Handle special commands first
    if let Some(template_path) = matches.get_one::<String>("generate-template") {
        return handle_template_generation(template_path);
    }

    if matches.get_flag("clear-cache") {
        return handle_cache_clear();
    }

    if matches.get_flag("cache-info") {
        return handle_cache_info();
    }

    // Build configuration
    let config = Config::from_args(&matches)?;
    config.validate()?;

    // Parse date filters
    let date_filter = parse_date_filter(&matches)?;

    // Run analysis
    run_analysis(config, date_filter, &matches).await
}

fn handle_template_generation(template_path: &str) -> Result<()> {
    let path = PathBuf::from(template_path);
    templates::create_default_template_file(&path)
        .map_err(|e| AppError::Config(format!("Template generation failed: {}", e)))?;

    println!("Template file generated: {}", path.display());
    println!("Edit this file to customize AI prompts and report formats.");
    Ok(())
}

fn handle_cache_clear() -> Result<()> {
    let cache = cache::ApiCache::new()
        .map_err(|e| AppError::Cache(format!("Cache initialization failed: {}", e)))?;

    cache
        .clear()
        .map_err(|e| AppError::Cache(format!("Cache clear failed: {}", e)))?;

    println!("API response cache cleared.");
    Ok(())
}

fn handle_cache_info() -> Result<()> {
    let cache = cache::ApiCache::new()
        .map_err(|e| AppError::Cache(format!("Cache initialization failed: {}", e)))?;

    let size = cache
        .size()
        .map_err(|e| AppError::Cache(format!("Failed to get cache size: {}", e)))?;

    let size_mb = size as f64 / 1_048_576.0;
    println!("Cache size: {:.2} MB", size_mb);
    Ok(())
}

fn parse_date_filter(
    matches: &clap::ArgMatches,
) -> Result<Option<(Option<NaiveDate>, Option<NaiveDate>)>> {
    if let Some(date_str) = matches.get_one::<String>("date") {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| AppError::Config(format!("Invalid date format: {}", e)))?;
        Ok(Some((Some(date), Some(date))))
    } else if let (Some(from_str), Some(to_str)) = (
        matches.get_one::<String>("from"),
        matches.get_one::<String>("to"),
    ) {
        let from_date = NaiveDate::parse_from_str(from_str, "%Y-%m-%d")
            .map_err(|e| AppError::Config(format!("Invalid from date: {}", e)))?;
        let to_date = NaiveDate::parse_from_str(to_str, "%Y-%m-%d")
            .map_err(|e| AppError::Config(format!("Invalid to date: {}", e)))?;
        Ok(Some((Some(from_date), Some(to_date))))
    } else if matches.get_flag("weekly") {
        let today = chrono::Local::now().naive_local().date();
        let weekday = today.weekday();
        let days_since_monday = weekday.num_days_from_monday();
        let monday = today - chrono::Duration::days(days_since_monday as i64);
        let sunday = monday + chrono::Duration::days(6);
        Ok(Some((Some(monday), Some(sunday))))
    } else if matches.get_flag("monthly") {
        let today = chrono::Local::now().naive_local().date();
        let first_day = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
            .ok_or_else(|| AppError::Config("Invalid date calculation".to_string()))?;
        let last_day = if today.month() == 12 {
            NaiveDate::from_ymd_opt(today.year() + 1, 1, 1)
                .ok_or_else(|| AppError::Config("Invalid date calculation".to_string()))?
                - chrono::Duration::days(1)
        } else {
            NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1)
                .ok_or_else(|| AppError::Config("Invalid date calculation".to_string()))?
                - chrono::Duration::days(1)
        };
        Ok(Some((Some(first_day), Some(last_day))))
    } else if matches.get_flag("all") {
        // When --all is specified, return None to indicate no date filter
        Ok(None)
    } else {
        // Default behavior: when no date options are specified, use today
        let today = chrono::Local::now().naive_local().date();
        Ok(Some((Some(today), Some(today))))
    }
}

async fn run_analysis(
    config: Config,
    date_filter: Option<(Option<NaiveDate>, Option<NaiveDate>)>,
    matches: &clap::ArgMatches,
) -> Result<()> {
    let api_key = config.api.openai_api_key.ok_or(ApiError::MissingApiKey)?;

    logger::info("Starting analysis...");

    // Run AI analysis
    let mut report = analyze_conversations_with_ai(
        &config.processing.log_dir,
        date_filter,
        &api_key,
        &config.output.language,
        &config.api.model,
        config.output.quiet,
        config.processing.parallel_requests,
    )
    .await
    .map_err(|e| AppError::Processing(format!("Analysis failed: {}", e)))?;

    // Handle token tracking display
    if !config.output.show_token_usage {
        report.token_tracker = None;
    }

    // Generate output
    let output_path = matches.get_one::<String>("output").map(PathBuf::from);

    match config.output.format {
        OutputFormat::Json => {
            let json_output =
                serde_json::to_string_pretty(&report).map_err(|e| AppError::Json(e))?;

            if let Some(path) = output_path.as_deref() {
                std::fs::write(path, json_output).map_err(|e| AppError::Io(e))?;
                logger::info(&format!("Report written to: {}", path.display()));
            } else {
                println!("{}", json_output);
            }
        }
        OutputFormat::Markdown => {
            smart_analyzer::generate_smart_report(
                &report,
                output_path.as_deref(),
                &config.output.language,
            )
            .map_err(|e| AppError::Processing(format!("Report generation failed: {}", e)))?;
        }
    }

    Ok(())
}

// Remove this impl as it's not needed
