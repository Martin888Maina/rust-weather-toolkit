use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use std::env;

use crate::weather;

#[derive(Deserialize)]
struct WeatherQuery {
    city: String,
}

#[derive(Serialize)]
struct WeatherApiResponse {
    city: String,
    country: String,
    temperature: f64,
    feels_like: f64,
    description: String,
    humidity: u32,
    pressure: u32,
    wind_speed: f64,
    visibility: u32,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn get_weather(
    query: web::Query<WeatherQuery>,
    api_key: web::Data<String>,
) -> HttpResponse {
    let city = query.city.trim();

    if city.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "City name cannot be empty".to_string(),
        });
    }

    match weather::fetch_weather(city, &api_key).await {
        Ok(data) => {
            let description = data
                .weather
                .first()
                .map(|w| w.description.clone())
                .unwrap_or_default();

            HttpResponse::Ok().json(WeatherApiResponse {
                city: data.name,
                country: data.sys.country,
                temperature: data.main.temp,
                feels_like: data.main.feels_like,
                description,
                humidity: data.main.humidity,
                pressure: data.main.pressure,
                wind_speed: data.wind.speed,
                visibility: data.visibility,
            })
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("404") {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: format!("City '{}' not found. Please check the spelling.", city),
                })
            } else if msg.contains("401") {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Invalid API key. Check your OPENWEATHER_API_KEY.".to_string(),
                })
            } else {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: format!("Failed to fetch weather data: {}", msg),
                })
            }
        }
    }
}

pub async fn run_server(api_key: String) {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3005);

    let bind_addr = format!("0.0.0.0:{}", port);
    println!("Starting weather server on http://{}", bind_addr);

    let key = web::Data::new(api_key);

    HttpServer::new(move || {
        App::new()
            .app_data(key.clone())
            .route("/api/weather", web::get().to(get_weather))
            .service(
                Files::new("/", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            )
    })
    .bind(&bind_addr)
    .unwrap_or_else(|e| panic!("Failed to bind to {}: {}", bind_addr, e))
    .run()
    .await
    .unwrap_or_else(|e| eprintln!("Server error: {}", e));
}
