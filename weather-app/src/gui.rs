use eframe::egui;
use crate::weather::{self, WeatherResponse};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct WeatherApp {
    city_input: String,
    weather_data: Option<Result<WeatherResponse, String>>,
    api_key: String,
    is_loading: bool,
    tx: Sender<Result<WeatherResponse, String>>,
    rx: Receiver<Result<WeatherResponse, String>>,
}

impl WeatherApp {
    pub fn new(api_key: String) -> Self {
        let (tx, rx) = channel();
        Self {
            city_input: String::new(),
            weather_data: None,
            api_key,
            is_loading: false,
            tx,
            rx,
        }
    }

    fn fetch_weather(&mut self) {
        let city = self.city_input.clone();
        let api_key = self.api_key.clone();
        let tx = self.tx.clone();

        self.is_loading = true;

        // Spawn async task
        tokio::spawn(async move {
            let result = weather::fetch_weather(&city, &api_key)
                .await
                .map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for weather data from async task
        if let Ok(data) = self.rx.try_recv() {
            self.weather_data = Some(data);
            self.is_loading = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌤️ Weather Application");
            ui.add_space(20.0);

            // City input
            ui.horizontal(|ui| {
                ui.label("City:");
                let response = ui.text_edit_singleline(&mut self.city_input);

                // Fetch on Enter key
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.fetch_weather();
                }
            });

            ui.add_space(10.0);

            // Fetch button
            if ui.button("🔍 Get Weather").clicked() {
                self.fetch_weather();
            }

            ui.add_space(20.0);

            // Loading indicator
            if self.is_loading {
                ui.spinner();
                ui.label("Fetching weather data...");
            }

            // Display weather data
            if let Some(ref result) = self.weather_data {
                match result {
                    Ok(weather) => {
                        ui.group(|ui| {
                            ui.heading(format!("📍 {}, {}", weather.name, weather.sys.country));
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.label("🌡️ Temperature:");
                                ui.label(format!("{:.1}°C", weather.main.temp));
                            });

                            ui.horizontal(|ui| {
                                ui.label("🤔 Feels like:");
                                ui.label(format!("{:.1}°C", weather.main.feels_like));
                            });

                            ui.horizontal(|ui| {
                                ui.label("☁️ Condition:");
                                ui.label(&weather.weather[0].description);
                            });

                            ui.horizontal(|ui| {
                                ui.label("💧 Humidity:");
                                ui.label(format!("{}%", weather.main.humidity));
                            });

                            ui.horizontal(|ui| {
                                ui.label("🎚️ Pressure:");
                                ui.label(format!("{} hPa", weather.main.pressure));
                            });

                            ui.horizontal(|ui| {
                                ui.label("💨 Wind:");
                                ui.label(format!("{:.1} m/s", weather.wind.speed));
                            });

                            ui.horizontal(|ui| {
                                ui.label("👁️ Visibility:");
                                ui.label(format!("{} meters", weather.visibility));
                            });
                        });
                    }
                    Err(error) => {
                        ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", error));
                    }
                }
            }
        });

        // Request repaint for loading animation
        if self.is_loading {
            ctx.request_repaint();
        }
    }
}
