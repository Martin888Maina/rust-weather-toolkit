use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Weather CLI")]
#[command(about = "Fetch weather information for any city", long_about = None)]
pub struct CliArgs {
    /// City name to fetch weather for
    #[arg(short, long)]
    pub city: Option<String>,

    /// Launch GUI interface
    #[arg(short, long, default_value = "false")]
    pub gui: bool,
}
