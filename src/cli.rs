use clap::{Arg, Command};

/// Build the command line interface
pub fn build_cli() -> Command {
    Command::new("cc2report")
        .version("1.0.0")
        .author("Claude Code Log Analyzer")
        .about("Generate intelligent work reports from Claude Code project logs\n\nBy default, generates a report for today's conversations.")
        .arg(
            Arg::new("date")
                .short('d')
                .long("date")
                .value_name("DATE")
                .help("Filter logs by date (YYYY-MM-DD)")
                .conflicts_with("from")
                .conflicts_with("to")
                .conflicts_with("weekly")
                .conflicts_with("monthly")
                .conflicts_with("all")
                .required(false),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Generate report for all conversations (no date filter)")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("date")
                .conflicts_with("from")
                .conflicts_with("to")
                .conflicts_with("weekly")
                .conflicts_with("monthly")
                .required(false),
        )
        .arg(
            Arg::new("weekly")
                .short('w')
                .long("weekly")
                .help("Generate report for the current week")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("date")
                .conflicts_with("from")
                .conflicts_with("to")
                .conflicts_with("monthly")
                .conflicts_with("all")
                .required(false),
        )
        .arg(
            Arg::new("monthly")
                .short('m')
                .long("monthly")
                .help("Generate report for the current month")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("date")
                .conflicts_with("from")
                .conflicts_with("to")
                .conflicts_with("weekly")
                .conflicts_with("all")
                .required(false),
        )
        .arg(
            Arg::new("from")
                .short('f')
                .long("from")
                .value_name("DATE")
                .help("Start date for range (YYYY-MM-DD)")
                .requires("to")
                .required(false),
        )
        .arg(
            Arg::new("to")
                .short('t')
                .long("to")
                .value_name("DATE")
                .help("End date for range (YYYY-MM-DD)")
                .requires("from")
                .required(false),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file path (default: stdout)")
                .required(false),
        )
        .arg(
            Arg::new("log-dir")
                .short('l')
                .long("log-dir")
                .value_name("DIR")
                .help("Claude projects directory (default: ~/.claude/projects)")
                .required(false),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .value_name("FORMAT")
                .help("Output format: markdown or json (default: markdown)")
                .value_parser(["markdown", "json"])
                .default_value("markdown")
                .required(false),
        )
        .arg(
            Arg::new("api-key")
                .long("api-key")
                .value_name("KEY")
                .help("OpenAI API key (or set OPENAI_API_KEY env var)")
                .required(false),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .value_name("LANG")
                .help("Language for work reports (default: auto-detected from LANG)")
                .value_parser([
                    "en", "ja", "zh", "ko", "es", "fr", "de", "pt", "ru", "it", "nl", "pl", "tr",
                    "ar", "hi", "th", "vi", "id", "ms",
                ])
                .required(false),
        )
        .arg(
            Arg::new("model")
                .long("model")
                .value_name("MODEL")
                .help("OpenAI model to use (default: gpt-4o for best quality)")
                .value_parser(["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"])
                .default_value("gpt-4o")
                .required(false),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress progress indicators")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("generate-template")
                .long("generate-template")
                .help("Generate a template configuration file")
                .value_name("FILE")
                .required(false),
        )
        .arg(
            Arg::new("clear-cache")
                .long("clear-cache")
                .help("Clear the API response cache")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("cache-info")
                .long("cache-info")
                .help("Show cache information")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("show-token-usage")
                .long("show-token-usage")
                .help("Show token usage tracking and cost estimation")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("parallel")
                .short('p')
                .long("parallel")
                .value_name("N")
                .help("Number of parallel API requests (default: 1, max: 10)")
                .value_parser(clap::value_parser!(usize))
                .default_value("1")
                .required(false),
        )
}
