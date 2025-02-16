use clap::{Parser};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "weather", long_about = "Weather is a cli tool that will give you the current temperature for the city you specify")]
pub struct CliConfig {
    #[arg(required=false)]
    pub city: Option<String>,
    #[clap(short, long, required=false)]
    default: Option<String>,
}