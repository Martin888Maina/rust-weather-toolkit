mod weather;
mod cli;

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "web")]
mod web;

use cli::CliArgs;
use clap::Parser;
use dotenv::dotenv;
use std::io::{self, Write};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = CliArgs::parse();

    let api_key = match env::var("OPENWEATHER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: OPENWEATHER_API_KEY not found");
            eprintln!("   Please create a .env file with your API key");
            std::process::exit(1);
        }
    };

    #[cfg(feature = "web")]
    if args.web {
        web::run_server(api_key).await;
        return;
    }

    #[cfg(feature = "gui")]
    if args.gui {
        run_gui(api_key);
        return;
    }

    #[cfg(not(feature = "gui"))]
    if args.gui {
        eprintln!("GUI mode is not available in this build.");
        std::process::exit(1);
    }

    if let Some(city) = args.city {
        fetch_and_display(&city, &api_key).await;
        return;
    }

    interactive_mode(&api_key).await;
}

#[cfg(feature = "gui")]
fn run_gui(api_key: String) {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Weather App",
        options,
        Box::new(|_cc| Box::new(gui::WeatherApp::new(api_key))),
    );
}

async fn fetch_and_display(city: &str, api_key: &str) {
    println!("Fetching weather for {}...\n", city);
    match weather::fetch_weather(city, api_key).await {
        Ok(data) => println!("{}", weather::format_weather(&data)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn interactive_mode(api_key: &str) {
    println!("Weather CLI Application");
    println!("-------------------------------\n");

    loop {
        print!("Enter city name (or 'quit' to exit): ");
        io::stdout().flush().unwrap();

        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read line");
        let city = city.trim();

        if city.eq_ignore_ascii_case("quit") || city.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        if city.is_empty() {
            println!("Please enter a valid city name\n");
            continue;
        }

        fetch_and_display(city, api_key).await;
        println!();
    }
}
