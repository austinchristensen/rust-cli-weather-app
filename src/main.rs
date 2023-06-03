mod model;
mod cli_config;
mod weather;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli_config::CliConfig::parse();
    
    let weather_controller = weather::weather_controller::WeatherController::init(args);
    weather_controller.get_weather().await;
}
