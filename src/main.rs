mod model;
mod cli_config;
mod weather;
mod config;
use clap::Parser;
use config::config_manager::ConfigManager;
use weather::weather_controller::WeatherController;

#[tokio::main]
async fn main() {
    let args = cli_config::CliConfig::parse();

    let config_manager = ConfigManager::new("config.json");
    
    let weather_controller = WeatherController::init(args);
    weather_controller.get_weather().await;
}
