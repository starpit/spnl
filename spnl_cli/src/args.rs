use crate::demos::Demo;
use clap::Parser;

#[derive(Parser, Debug, serde::Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to process
    #[arg(required_unless_present("demo"))]
    pub file: Option<String>,

    /// Demo to run
    #[arg(value_enum, short, long)]
    pub demo: Option<Demo>,

    /// Model
    #[arg(short, long, default_value = "ollama/granite3.2:2b")]
    pub model: String,

    /// Temperature
    #[arg(short, long, default_value_t = 0.5)]
    pub temperature: f32,

    /// Max Completion/Generated Tokens
    #[arg(short = 'l', long, default_value_t = 100)]
    pub max_tokens: i32,

    /// Number of candidate emails to consider
    #[arg(short, long, default_value_t = 5)]
    pub n: u32,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
