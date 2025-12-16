# 🌤️ Rust Weather Toolkit

A comprehensive beginner's toolkit for learning Rust by building a real-world weather application with both terminal and GUI interfaces.

## 📚 Project Overview

This project demonstrates how to use Rust to build a functional weather application that fetches real-time weather data from the OpenWeatherMap API. It features both a command-line interface (CLI) and a graphical user interface (GUI), showcasing Rust's versatility and the power of code reusability.

**Created as part of:** Moringa School AI-Powered Learning Capstone Project

## ✨ Features

- 🖥️ **Terminal Interface**: Interactive CLI for quick weather lookups
- 🎨 **GUI Interface**: Visual application with modern design using egui
- 🔄 **Shared Logic**: DRY principle with reusable core weather module
- 🌍 **Real-time Data**: Live weather from OpenWeatherMap API
- ⚡ **Async Operations**: Fast, non-blocking API calls with tokio
- 📝 **Comprehensive Docs**: Full setup and troubleshooting guides
- ✅ **Unit Tests**: Test coverage for core functionality

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ and Cargo (install from [rustup.rs](https://rustup.rs))
- OpenWeatherMap API key (get free at [openweathermap.org](https://openweathermap.org/api))

### Installation

```bash
# Clone the repository
git clone https://github.com/Martin888Maina/rust-weather-toolkit.git
cd rust-weather-toolkit/weather-app

# Set up API key
cp .env.example .env
# Edit .env and add your OpenWeatherMap API key

# Build the project
cargo build

# Run terminal mode
cargo run

# Run GUI mode
cargo run -- --gui

# Quick city lookup
cargo run -- --city "London"
```

## 📖 Usage

### Terminal Mode (Interactive)

```bash
cargo run
```

This launches an interactive terminal interface where you can:
- Enter city names to get weather information
- Type 'quit' or 'exit' to close the application

**Example:**
```
🌤️  Weather CLI Application
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Enter city name (or 'quit' to exit): Nairobi
🔍 Fetching weather for Nairobi...

Weather in Nairobi, KE:
━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌡️  Temperature: 22.5°C
🤔 Feels like: 22.1°C
☁️  Condition: broken clouds
💧 Humidity: 65%
🎚️  Pressure: 1013 hPa
💨 Wind Speed: 3.5 m/s
👁️  Visibility: 10000 meters
━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Terminal Mode (Direct Query)

```bash
cargo run -- --city "London"
```

Get weather information for a specific city and exit immediately.

### GUI Mode

```bash
cargo run -- --gui
```

Launches a graphical window where you can:
- Enter city names in the text field
- Click "Get Weather" button or press Enter
- View formatted weather information

## 🏗️ Project Structure

```
rust-weather-toolkit/
├── weather-app/              # Main application
│   ├── src/
│   │   ├── main.rs          # Entry point & mode selection
│   │   ├── weather.rs       # Core API logic (shared)
│   │   ├── cli.rs           # CLI argument parsing
│   │   └── gui.rs           # GUI interface
│   ├── Cargo.toml           # Dependencies
│   ├── .env                 # API key (gitignored)
│   └── .env.example         # Template for API key
├── .gitignore               # Git exclusions
├── LICENSE                  # MIT License
└── README.md                # This file
```

## 🛠️ Tech Stack

- **Language**: Rust 1.70+
- **HTTP Client**: reqwest (async HTTP requests)
- **JSON**: serde, serde_json (serialization/deserialization)
- **Async Runtime**: tokio (asynchronous programming)
- **GUI**: egui, eframe (immediate-mode GUI)
- **CLI**: clap (command-line argument parsing)
- **Environment**: dotenv (environment variable management)
- **API**: OpenWeatherMap (weather data provider)

## 📝 Learning Outcomes

By exploring this project, you will learn:

- ✅ Rust syntax and ownership model
- ✅ Async/await programming in Rust
- ✅ HTTP API integration with reqwest
- ✅ JSON deserialization with serde
- ✅ GUI development with egui
- ✅ CLI application design with clap
- ✅ Error handling best practices
- ✅ Project structure and modularity
- ✅ Environment variable management
- ✅ Unit testing in Rust

## 🤖 AI-Powered Learning

This project was developed using AI-assisted learning through:

- Understanding Rust concepts and ownership rules
- Debugging compilation errors
- Learning library APIs (reqwest, serde, egui)
- Best practices guidance
- Code examples and explanations

**Key AI Prompts Used:**
1. "How to make async HTTP requests in Rust using reqwest?"
2. "Explain serde JSON deserialization in Rust with example"
3. "How to create a GUI in Rust using egui?"
4. "How to handle async operations in egui applications?"
5. "Best practices for structuring Rust projects with multiple interfaces"

## 🧪 Testing

```bash
cd weather-app

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code formatting
cargo fmt --check

# Run linter (clippy)
cargo clippy
```

## 🔧 Configuration

### API Key Setup

1. Sign up at [OpenWeatherMap](https://openweathermap.org/api)
2. Get your free API key from the dashboard
3. Copy `.env.example` to `.env`
4. Replace `your_api_key_here` with your actual API key
5. Wait 10-15 minutes for key activation

### Environment Variables

The application looks for the following environment variable:

```
OPENWEATHER_API_KEY=your_api_key_here
```

## 🐛 Troubleshooting

### Issue: "API key not found"

**Solution:** Ensure `.env` file exists in `weather-app/` directory with correct format:
```
OPENWEATHER_API_KEY=your_key_here
```

### Issue: "API request failed with status: 401"

**Solution:**
- Wait 10-15 minutes after creating new API key
- Verify API key is correct (copy from OpenWeatherMap dashboard)
- Check for spaces or quotes in `.env` file

### Issue: "City not found"

**Solution:**
- Check spelling of city name
- Try format: "City, Country Code" (e.g., "London, UK")
- Use English names for cities

### Issue: Compilation errors on Windows

**Solution:** Rust works best on Windows with proper build tools:
- Install Visual C++ Build Tools
- Or use Windows Subsystem for Linux (WSL2)

## 🎯 Project Requirements Met

This project fulfills all Moringa School capstone requirements:

✅ **Technology Exploration**: Rust (not Python/Java/JavaScript)
✅ **Working Example**: Both CLI and GUI fully functional
✅ **Documentation**: Comprehensive setup and usage guide
✅ **AI Prompts**: Documented learning process
✅ **Testing**: Unit tests and manual testing completed
✅ **Code Repository**: GitHub with clear commit history

## 🚧 Future Enhancements

Potential extensions for continued learning:

- [ ] 5-day weather forecast
- [ ] Save favorite cities
- [ ] Weather alerts and notifications
- [ ] Historical weather data
- [ ] Multiple unit systems (Fahrenheit, Kelvin)
- [ ] Weather maps integration
- [ ] Mobile-responsive web interface

## 📊 Project Statistics

- **Lines of Code**: ~500
- **Development Time**: Completed efficiently using AI assistance
- **Dependencies**: 8 main crates
- **Test Coverage**: Core weather functionality
- **Interfaces**: 2 (Terminal CLI + GUI)

## 👤 Author

**Martin Maina**
Moringa School - AI-Powered Learning Capstone Project
GitHub: [@Martin888Maina](https://github.com/Martin888Maina)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Moringa School for the learning opportunity
- OpenWeatherMap for the free weather API
- Rust community for excellent documentation
- AI tools for accelerating the learning process

## 📞 Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [OpenWeatherMap API Docs](https://openweathermap.org/api)

### Crate Documentation
- [reqwest](https://docs.rs/reqwest/)
- [serde](https://serde.rs/)
- [tokio](https://tokio.rs/)
- [egui](https://docs.rs/egui/)
- [clap](https://docs.rs/clap/)

### Learning Resources
- [Rust Programming Course - freeCodeCamp](https://www.youtube.com/watch?v=MsocPEZBd-M)
- [Rustlings - Small exercises](https://github.com/rust-lang/rustlings)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

---

**⭐ If this project helped you learn Rust, please star it on GitHub!**
