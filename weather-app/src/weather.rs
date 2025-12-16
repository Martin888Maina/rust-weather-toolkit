use serde::{Deserialize, Serialize};
use std::error::Error;

// Define the structure for weather data from API
#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherResponse {
    pub main: MainWeather,
    pub weather: Vec<Weather>,
    pub name: String,
    pub sys: Sys,
    pub wind: Wind,
    pub visibility: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u32,
    pub pressure: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    pub description: String,
    pub main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sys {
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wind {
    pub speed: f64,
}

// Main function to fetch weather data
pub async fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    let weather_data: WeatherResponse = response.json().await?;
    Ok(weather_data)
}

// Helper function to format weather data for display
pub fn format_weather(weather: &WeatherResponse) -> String {
    format!(
        "Weather in {}, {}:
━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌡️  Temperature: {:.1}°C
🤔 Feels like: {:.1}°C
☁️  Condition: {}
💧 Humidity: {}%
🎚️  Pressure: {} hPa
💨 Wind Speed: {:.1} m/s
👁️  Visibility: {} meters
━━━━━━━━━━━━━━━━━━━━━━━━━━━",
        weather.name,
        weather.sys.country,
        weather.main.temp,
        weather.main.feels_like,
        weather.weather[0].description,
        weather.main.humidity,
        weather.main.pressure,
        weather.wind.speed,
        weather.visibility
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_response_structure() {
        let json_data = r#"
        {
            "main": {
                "temp": 22.5,
                "feels_like": 21.8,
                "humidity": 65,
                "pressure": 1013
            },
            "weather": [
                {
                    "main": "Clouds",
                    "description": "broken clouds"
                }
            ],
            "wind": {
                "speed": 3.5
            },
            "visibility": 10000,
            "name": "Nairobi",
            "sys": {
                "country": "KE"
            }
        }
        "#;

        let weather: Result<WeatherResponse, _> = serde_json::from_str(json_data);
        assert!(weather.is_ok());

        let weather = weather.unwrap();
        assert_eq!(weather.name, "Nairobi");
        assert_eq!(weather.main.temp, 22.5);
    }

    #[test]
    fn test_format_weather_output() {
        let weather = WeatherResponse {
            name: "TestCity".to_string(),
            main: MainWeather {
                temp: 25.0,
                feels_like: 24.5,
                humidity: 70,
                pressure: 1015,
            },
            weather: vec![Weather {
                main: "Clear".to_string(),
                description: "clear sky".to_string(),
            }],
            sys: Sys {
                country: "TC".to_string(),
            },
            wind: Wind { speed: 5.0 },
            visibility: 10000,
        };

        let output = format_weather(&weather);
        assert!(output.contains("TestCity"));
        assert!(output.contains("25.0"));
        assert!(output.contains("clear sky"));
    }
}
