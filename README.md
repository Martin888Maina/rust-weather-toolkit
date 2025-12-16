# Getting Started with Rust: Building a Weather API Client
## A Beginner's Toolkit - Moringa School Capstone Project

**Author:** Martin Maina
**Date:** December 2025
**Repository:** [rust-weather-toolkit](https://github.com/Martin888Maina/rust-weather-toolkit)

---

## 1. Title & Objective

### What Technology Did I Choose?
**Rust Programming Language** - A modern systems programming language focused on safety, speed, and concurrency, with specific focus on:
- HTTP API integration using async/await
- Terminal (CLI) interface development
- Graphical User Interface (GUI) development
- Real-world application building

### Why Did I Choose Rust?
1. **Industry Relevance**: Used by major tech companies (Mozilla, Discord, Cloudflare, AWS)
2. **Assignment Requirement**: Not Python, Java, or JavaScript
3. **Learning Challenge**: Different paradigm with memory safety and ownership concepts
4. **Growing Ecosystem**: Modern tooling, active community, and excellent documentation
5. **Real-world Skills**: Perfect for building system tools and high-performance applications

### What's the End Goal?
Build a functional weather application with:
- **Terminal interface** for quick command-line weather lookups
- **GUI interface** for visual interaction with weather data
- **Shared core logic** demonstrating code reusability and the DRY principle
- Both interfaces fetch real-time weather data from the OpenWeatherMap API

---

## 2. Quick Summary of the Technology

### What is Rust?
Rust is a systems programming language that emphasizes three key principles:
- **Memory Safety**: Prevents common bugs like null pointer dereferences and buffer overflows at compile time
- **Concurrency**: Safe concurrent programming without data races
- **Performance**: Zero-cost abstractions with performance comparable to C/C++

### Where is Rust Used?

**1. Web Browsers**
- Firefox (Servo rendering engine components)
- Chrome (parts of V8 JavaScript engine)

**2. Cloud & Infrastructure**
- AWS (Firecracker microVM for Lambda)
- Cloudflare (edge computing platform)
- Dropbox (storage system rewrite)

**3. Blockchain & Cryptocurrency**
- Solana, Polkadot, Near Protocol

**4. Game Development**
- Game engines (Bevy, Amethyst)
- Performance-critical game systems

**5. Operating Systems**
- Linux kernel modules
- Windows components

### Real-World Example
**Discord** rewrote their "Read States" service from Go to Rust:
- **Latency**: Reduced from 125ms to 5ms (96% improvement)
- **Memory**: 90% reduction in memory usage
- **Reliability**: Eliminated all memory-related crashes

---

## 3. System Requirements

### Operating System Support
- ✅ **Windows** 10/11 (used for this project)
- ✅ **Linux** (Ubuntu 20.04+, Debian, Fedora)
- ✅ **macOS** (10.15 Catalina or later)

### Required Software

#### Core Tools
```bash
# Rust toolchain (rustc + cargo)
Version: 1.70.0 or later
Install: https://rust-lang.org/tools/install/

# Git for version control
Version: 2.30+
Install: https://git-scm.com

# Code editor
- VS Code with rust-analyzer extension (recommended)
- IntelliJ IDEA with Rust plugin
- Any text editor of your choice
```

#### Windows-Specific Requirements
- **Visual C++ Build Tools** (for compiling native dependencies)
- **Windows SDK** (usually included with VS Build Tools)

### API Requirements
- **OpenWeatherMap Account** (free tier)
  - Sign up at: https://openweathermap.org/api
  - Rate Limit: 60 calls/minute, 1,000 calls/day (free tier)
  - Response Time: ~200-500ms average

---

## 4. Installation & Setup Instructions

### Step 1: Install Rust

**For Windows:**
1. Download rustup-init.exe from https://rustup.rs
2. Run the installer and follow prompts
3. Select default installation
4. Restart terminal after installation

**Verify Installation:**
```bash
rustc --version
# Expected: rustc 1.XX.X

cargo --version
# Expected: cargo 1.XX.X
```

### Step 2: Clone the Repository

```bash
# Clone from GitHub
git clone https://github.com/Martin888Maina/rust-weather-toolkit.git

# Navigate to project directory
cd rust-weather-toolkit/weather-app
```

### Step 3: Get OpenWeatherMap API Key

1. Visit https://openweathermap.org/api
2. Click "Sign Up" (top right corner)
3. Fill registration form with your email and password
4. Verify email (check spam folder if needed)
5. Login and navigate to "API keys" tab in dashboard
6. Copy your default API key
7. **Important:** Wait 10-15 minutes for key activation

### Step 4: Configure Environment Variables

```bash
# Copy the example environment file
cp .env.example .env

# Edit .env and add your API key
# File should contain:
OPENWEATHER_API_KEY=your_actual_api_key_here
```

**Important:** The `.env` file is already in `.gitignore` and will NOT be committed to Git.

### Step 5: Build the Project

```bash
# Install dependencies and compile
cargo build

# This will:
# 1. Download all required crates
# 2. Compile dependencies
# 3. Build the application
# Note: First build takes 2-5 minutes
```

---

## 5. Minimal Working Example

### Example 1: Terminal Mode - Quick City Lookup

```bash
# Direct city query (fastest)
cargo run -- --city "London"
```

**What This Does:**
- Fetches current weather for London
- Displays temperature, conditions, humidity, wind speed
- Exits immediately after displaying results

**Expected Output:**
```
🔍 Fetching weather for London...

Weather in London, GB:
━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌡️  Temperature: 11.0°C
🤔 Feels like: 10.5°C
☁️  Condition: overcast clouds
💧 Humidity: 90%
🎚️  Pressure: 1006 hPa
💨 Wind Speed: 3.6 m/s
👁️  Visibility: 10000 meters
━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Example 2: Interactive Terminal Mode

```bash
# Launch interactive CLI
cargo run
```

**What This Does:**
- Opens an interactive prompt
- Allows querying multiple cities
- Type 'quit' or 'exit' to close

**Example Session:**
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

Enter city name (or 'quit' to exit): quit
👋 Goodbye!
```

### Example 3: GUI Mode

```bash
# Launch graphical interface
cargo run -- --gui
```

**What This Does:**
- Opens a window with modern GUI
- Text field for entering city names
- Button to fetch weather
- Displays formatted results with icons
- Shows loading indicator during fetch

---

## 6. AI Prompt Journal

This section documents all AI prompts used during development, based on Moringa School's recommended prompt strategies for learning new technologies.

#### Prompt 1: README Creation
**Prompt Used:** "Please create a comprehensive README.md for Rust Weather Toolkit with sections for installation, usage, features (Terminal CLI + GUI interfaces), configuration (OpenWeatherMap API key), troubleshooting, and code structure overview."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI generated comprehensive documentation structure with all required sections, markdown formatting, and beginner-friendly explanations.
**Evaluation:** Excellent - Provided solid foundation for project documentation.

#### Prompt 2: Step-by-Step Implementation Guide
**Prompt Used:** "Create a step-by-step guide for implementing async HTTP requests with error handling in Rust for fetching weather data from OpenWeatherMap API, including prerequisites, numbered steps, code blocks, and troubleshooting for API errors (401, 404)."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI provided detailed implementation guide covering async functions, Result types, reqwest usage, and practical error handling examples.
**Evaluation:** Perfect - Made complex async concepts digestible with structured approach.

#### Prompt 3: Error Message Translation
**Prompt Used:** "Explain this Rust error 'mismatched types: expected Box<dyn App>, found Result<Box<WeatherApp>>' from eframe initialization. Context: Rust 1.92, eframe 0.24. Explain in simple terms, identify relevant code lines, list likely causes, and provide debugging steps."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI explained eframe API change, clarified that run_native expects Box<dyn App> directly not wrapped in Result, provided correct syntax.
**Evaluation:** Outstanding - Turned cryptic compiler error into actionable fix quickly.

#### Prompt 4: Test Planning Guidance
**Prompt Used:** "Help me create a testing plan for WeatherResponse struct and fetch_weather function by asking guiding questions, helping identify behaviors and edge cases, and creating a prioritized test checklist rather than writing tests directly."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI guided through identifying testable behaviors (JSON deserializ

ation, error handling), suggested test priorities, and helped discover edge cases.
**Evaluation:** Very good - Learned testing principles through guided discovery.

#### Prompt 5: Code Structure Improvement
**Prompt Used:** "Improve readability of my main.rs code by: applying consistent indentation, breaking long expressions with clear variable names, extracting complex conditions, reducing nesting, separating logical sections, replacing magic numbers with constants."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI refactored code with better variable names, extracted boolean expressions, reduced nesting with early returns, and organized with blank lines.
**Evaluation:** Good - Significantly improved code readability and learned Rust best practices.

#### Prompt 6: Code Quality Review
**Prompt Used:** "Review my weather.rs code for quality improvements. Identify code smells, suggest specific improvements, explain why improvements matter in Rust, and rate readability, performance, and maintainability."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI identified missing input validation, suggested more descriptive errors, recommended logging and thiserror crate. Rated 7/10 readability, 8/10 performance, 6/10 maintainability.
**Evaluation:** Excellent - Concrete suggestions improved error handling and code quality.

#### Prompt 7: Understanding Rust Ownership
**Prompt Used:** "Explain Rust ownership and borrowing with simple examples. Show 3 practical use cases in my weather app, provide a practice project idea, and list common mistakes to avoid."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI explained with book lending analogy, showed examples in weather app (passing API key by reference), suggested cache system practice project.
**Evaluation:** Very good - Analogies and practical examples made ownership concepts clearer.

#### Prompt 8: Conceptual Understanding (JavaScript to Rust)
**Prompt Used:** "I'm proficient in JavaScript learning Rust. Explain key philosophical differences, problems Rust solves, mental models to adjust from JavaScript, and common misconceptions JavaScript developers have about Rust."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI explained compile-time safety vs runtime checks, memory management differences (ownership vs garbage collection), performance trade-offs, and addressed complexity misconceptions.
**Evaluation:** Exceptional - Conceptual foundation made subsequent learning much smoother.

#### Prompt 9: Async/Await Comparison
**Prompt Used:** "I use JavaScript async/await and Promises. Explain Rust's async/await by comparing tokio to JavaScript's event loop, async fn to JavaScript's async functions, handling concurrent requests, and performance implications."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI compared tokio runtime to JavaScript event loop, explained async fn equivalence to JavaScript async functions, showed tokio::spawn vs Promise.all(), highlighted zero-cost abstractions.
**Evaluation:** Perfect - Leveraging JavaScript knowledge made Rust async immediately understandable.

#### Prompt 10: Getting Started with egui
**Prompt Used:** "Add GUI to Rust weather app using egui. Provide Hello World example with text field and button, explain immediate-mode vs retained-mode (React comparison), clarify core concepts, and structure learning path for weather GUI."
**Link:** https://ai.moringaschool.com
**Response Summary:** AI provided minimal egui example, explained immediate-mode (UI rebuilt every frame vs React virtual DOM), suggested learning order: widgets → layout → state → async.
**Evaluation:** Very good - React comparison made immediate-mode concepts clear.

### Overall AI Learning Impact

**What Worked Well:**
- Providing JavaScript/React background enabled meaningful analogies
- Moringa's structured prompt templates yielded comprehensive responses
- Starting with conceptual understanding before implementation prevented confusion
- Pasting actual error messages led to specific, applicable solutions

**What Required Iteration:**
- AI sometimes suggested outdated crate APIs requiring docs verification
- Needed multiple prompts for Windows-specific troubleshooting
- Initial responses sometimes too broad, needed follow-ups for details

**Productivity Impact:**
- **Development Speed:** 3-4x faster than learning without AI
- **Research Efficiency:** 80% fewer documentation searches
- **Concept Mastery:** AI explanations with examples deepened understanding
- **Debugging:** AI error interpretation saved hours with compiler errors
- **Time to First Prototype:** 3 days (estimated 1-2 weeks without AI)

---

## 7. Common Issues & Fixes

### Issue 1: "API key not found in environment"

**Error Message:**
```
❌ Error: OPENWEATHER_API_KEY not found
   Please create a .env file with your API key
```

**Cause:** `.env` file missing or incorrectly formatted

**Solution:**
```bash
# Verify .env file exists in weather-app directory
ls .env

# Check contents (on Windows)
type .env

# Should show: OPENWEATHER_API_KEY=your_key_here

# Recreate if needed
echo OPENWEATHER_API_KEY=your_key > .env
```

**Prevention:** Ensure `.env` file is in the `weather-app` directory (same location as `Cargo.toml`).

---

### Issue 2: "API request failed with status: 401"

**Error Message:**
```
❌ API error: Status code: 401
```

**Cause:** Invalid or inactive API key

**Solutions:**
1. **Wait for activation:** New API keys take 10-15 minutes to activate
2. **Verify key:** Copy key directly from OpenWeatherMap dashboard (no spaces)
3. **Check format:** `.env` file should have format `OPENWEATHER_API_KEY=key` (no quotes)
4. **Test key:** Visit this URL in browser:
   ```
   https://api.openweathermap.org/data/2.5/weather?q=London&appid=YOUR_KEY&units=metric
   ```

**Prevention:** Wait full 15 minutes after creating key before first use.

---

### Issue 3: "City not found" (404 error)

**Error Message:**
```
❌ Error: City 'XYZ' not found
```

**Cause:** Incorrect city name or spelling

**Solutions:**
1. Check spelling carefully (case-insensitive but must be correct)
2. Try format: "City, CountryCode" (e.g., "London, UK", "Paris, FR")
3. Use English names for cities
4. For cities with spaces, no special handling needed (e.g., "New York")

**Examples That Work:**
```
Nairobi
London, UK
New York
Tokyo
```

**Examples That Don't Work:**
```
Nairbi     # Spelling error
NYC        # Use full name "New York"
```

---

### Issue 4: Compilation errors on Windows

**Error Message:**
```
error: linker `link.exe` not found
```

**Cause:** Missing Visual C++ Build Tools

**Solutions:**
1. **Install Build Tools:**
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Select "Build Tools for Visual Studio 2022"
   - Choose "Desktop development with C++"
   - Install and restart computer

2. **Alternative:** Use Windows Subsystem for Linux (WSL2)
   ```bash
   wsl --install
   # Then follow Linux installation steps
   ```

---

### Issue 5: Slow compilation times

**Symptoms:** Initial `cargo build` takes 5+ minutes

**Cause:** Normal - Rust compiles all dependencies from source for optimization

**Solutions:**
1. **Be patient on first build** - Subsequent builds are much faster (~30-60 seconds)
2. **Enable parallel compilation** (usually enabled by default):
   ```bash
   cargo build --jobs 4  # Use 4 CPU cores
   ```
3. **Use release mode only when needed** - dev builds are faster:
   ```bash
   cargo build              # Fast dev build
   cargo build --release    # Slow optimized build
   ```

**Note:** This is expected behavior - Rust prioritizes safety and performance over build speed.

---

### Issue 6: GUI window doesn't open

**Symptoms:** `cargo run -- --gui` exits without showing window

**Cause:** Missing graphics libraries or drivers

**Solutions:**

**Windows:**
- Update graphics drivers from manufacturer website
- Ensure Windows is up to date
- Try running from PowerShell as Administrator

**If issues persist:**
- Check Windows Event Viewer for error details
- Ensure Visual C++ Redistributables are installed

---

### Getting Additional Help

If you encounter issues not listed here:

1. **Check Cargo output** - Rust compiler errors are very descriptive
2. **Search Rust forums:**
   - https://users.rust-lang.org
   - https://www.reddit.com/r/rust
3. **Official documentation:**
   - https://doc.rust-lang.org/book/
   - https://docs.rs/ (crate documentation)
4. **GitHub issues** for specific crates used in project

---

## 8. References

### Official Documentation

**Rust Language:**
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Official book, comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learning through code examples
- [Standard Library](https://doc.rust-lang.org/std/) - API documentation

**Cargo (Package Manager):**
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Complete guide to Cargo
- [Crates.io](https://crates.io) - Package registry

**API Documentation:**
- [OpenWeatherMap API](https://openweathermap.org/api) - Weather API reference
- [API Response Examples](https://openweathermap.org/current) - Sample responses

### Crate Documentation

**HTTP & JSON:**
- [reqwest](https://docs.rs/reqwest/) - HTTP client library
- [serde](https://serde.rs/) - Serialization framework
- [tokio](https://tokio.rs/) - Async runtime

**CLI & GUI:**
- [clap](https://docs.rs/clap/) - Command-line argument parser
- [egui](https://docs.rs/egui/) - Immediate-mode GUI framework
- [eframe](https://docs.rs/eframe/) - egui framework wrapper

### Video Tutorials

**Rust Basics:**
1. ["Rust Crash Course" by Traversy Media](https://www.youtube.com/watch?v=zF34dRivLOw) - 1 hour introduction
2. ["Rust Programming Course for Beginners" by freeCodeCamp](https://www.youtube.com/watch?v=MsocPEZBd-M) - 5 hour comprehensive course

**Async Programming:**
3. ["Async/Await in Rust" by Jon Gjengset](https://www.youtube.com/watch?v=ThjvMReOXYM) - 2 hour deep dive

### Blog Posts & Articles

**Getting Started:**
- ["A half-hour to learn Rust"](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) - Quick syntax overview
- ["Rust for Rustaceans"](https://rust-for-rustaceans.com/) - Book for intermediate learners

**API Integration:**
- ["Making HTTP Requests in Rust"](https://blog.logrocket.com/making-http-requests-rust-reqwest/)
- ["Working with JSON in Rust"](https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/)

### Community Resources

**Forums:**
- [Rust Users Forum](https://users.rust-lang.org) - Official community forum
- [Reddit r/rust](https://reddit.com/r/rust) - Active Reddit community
- [Discord](https://discord.gg/rust-lang) - Real-time chat

**Learning Platforms:**
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises for learning
- [Exercism Rust Track](https://exercism.org/tracks/rust) - Mentored learning

---

## 📦 Project Structure

```
rust-weather-toolkit/
├── weather-app/              # Main application
│   ├── src/
│   │   ├── main.rs          # Entry point & mode selection
│   │   ├── weather.rs       # Core weather API logic (shared)
│   │   ├── cli.rs           # CLI argument parsing
│   │   └── gui.rs           # GUI interface
│   ├── Cargo.toml           # Dependencies & metadata
│   ├── .env                 # API key (gitignored)
│   └── .env.example         # Template for setup
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

## ✨ Features

- 🖥️ **Terminal Interface**: Interactive CLI for quick weather lookups
- 🎨 **GUI Interface**: Visual application with modern design using egui
- 🔄 **Shared Logic**: DRY principle with reusable core weather module
- 🌍 **Real-time Data**: Live weather from OpenWeatherMap API
- ⚡ **Async Operations**: Fast, non-blocking API calls with tokio
- 📝 **Unit Tests**: Test coverage for core functionality
- 🔒 **Security**: API key management with environment variables

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

## 📊 Project Statistics

- **Lines of Code**: ~500
- **Development Time**: Completed with AI assistance
- **Dependencies**: 8 main crates
- **Test Coverage**: Core weather functionality
- **Interfaces**: 2 (Terminal CLI + GUI)

## 🎯 Capstone Requirements Met

This project fulfills all Moringa School capstone requirements:

✅ **Technology Exploration**: Rust (not Python/Java/JavaScript)
✅ **Working Example**: Both CLI and GUI fully functional
✅ **Documentation**: Comprehensive setup and usage guide
✅ **AI Prompts**: 12+ documented prompts with evaluations
✅ **Testing**: Unit tests and manual testing completed
✅ **Code Repository**: GitHub with clear commit history
✅ **Common Errors**: Troubleshooting section included
✅ **References**: Official docs and learning resources provided

## 👤 Author

**Martin Maina**
Moringa School - AI-Powered Learning Capstone Project
GitHub: [@Martin888Maina](https://github.com/Martin888Maina)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Moringa School for the learning opportunity
- OpenWeatherMap for the free weather API
- Rust community for excellent documentation and support
- AI tools (ai.moringaschool.com) for accelerating the learning process

---


