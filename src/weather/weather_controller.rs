use crate::{cli_config::CliConfig, model::weather_data::WeatherData};

pub struct WeatherController {
    cli_config: CliConfig
}

impl WeatherController {
    pub fn init(wc: CliConfig) -> WeatherController {
        WeatherController {cli_config: wc}
    }

    pub async fn get_weather(self) -> Result<(), Box<dyn std::error::Error>> {
        let cities: Vec<String> = vec![self.cli_config.city];
    
        for city in cities {
            let url = format!("https://api.weatherapi.com/v1/current.json?key=f618ac31e4a741d7811182635230206&q={}&aqi=no", city);
            let response = reqwest::get(&url).await?.text().await?;
    
            let weather: WeatherData = serde_json::from_str(&response)?;
    
            println!(
                "Weather in {}: Temperature: {}°C",
                city, weather.current.temp_f
            );
        }
    
        Ok(())
    }
}