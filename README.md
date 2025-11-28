# 🦀 Vibe Rust Coder - AI Code Assistant

> A powerful Rust-based source code builder designed for seamless AI collaboration

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## 🎯 What is Vibe Rust Coder?

Vibe Rust Coder is a GUI application that bridges the gap between AI coding assistants and Rust development. It allows you to:

- **Copy-paste code** directly from AI chat conversations
- **Execute simple commands** to search, add, build, and test code
- **Understand your project** structure using Rust's parser
- **Keep chat history** for easy reference and verification
- **Build and test** with integrated cargo commands

## ✨ Key Features

### 🔍 **Smart Code Analysis**
- Uses `syn` crate for full Rust AST parsing
- Understands functions, structs, enums, and variables
- Searches across entire project
- Extracts specific code elements

### 💬 **Chat-Based Interface**
- Beautiful GUI with color-coded messages
- Scrollable chat history
- Timestamps on all messages
- Easy copy-paste workflow

### 🛠️ **Integrated Build Tools**
- `build` - Run cargo build
- `test` - Run cargo test
- `run` - Execute your project
- `profile` - Build with --release

### 📁 **Project Understanding**
- Automatic directory scanning
- Gitignore-aware file detection
- Relative path handling
- File content search

## 🚀 Quick Start

### 1. Build the Application
```powershell
cd U:\godot\vibe_rust_coder
cargo build --release
```

### 2. Run It
```powershell
cargo run --release
```

### 3. Load Your Project
- Enter your Rust project path
- Click "Load Project"
- Start using commands!

### 4. Try Your First Command
```
list files
```

## 📖 Documentation

| Document | Description |
|----------|-------------|
| **[README.md](README.md)** | Full feature documentation and user guide |
| **[QUICKSTART.md](QUICKSTART.md)** | Quick start guide with troubleshooting |
| **[EXAMPLES.md](EXAMPLES.md)** | Detailed usage examples and patterns |
| **[AI_WORKFLOW.md](AI_WORKFLOW.md)** | Complete AI collaboration workflow |
| **[ARCHITECTURE.md](ARCHITECTURE.md)** | System architecture and design |
| **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** | Complete project overview |

## 🎮 Command Reference

```bash
# Search
search <query>              # Search for files, functions, or code

# View
show <file>                 # Display file contents
show <file>::<function>     # Show specific function
list files                  # List all Rust files
list functions <file>       # List functions in a file

# Modify
add into <file>             # Add code to a file
<code here>

# Build & Test
build                       # Build the project
test [name]                 # Run tests
run [args]                  # Run the project
profile                     # Build with --release

# Help
help                        # Show all commands
```

## 💡 Example Usage

### Scenario: Adding an NPC System

**1. Ask AI:**
> "I need an NPC system for my game"

**2. AI Responds:**
```
add into src/npc.rs
pub struct Npc {
    pub name: String,
    pub health: f32,
}

impl Npc {
    pub fn new(name: String) -> Self {
        Self { name, health: 100.0 }
    }
}
```

**3. Copy-Paste into Vibe Rust Coder:**
- Copy the entire AI response
- Paste into command input
- Press Enter

**4. Build and Test:**
```
build
test
```

**Done!** ✅

## 🏗️ Architecture

```
┌─────────────────────────────────────┐
│         GUI (egui)                  │
│      Chat Interface                 │
└──────────────┬──────────────────────┘
               │
    ┌──────────┴──────────┐
    │                     │
    ▼                     ▼
┌─────────┐         ┌──────────┐
│ Command │         │  Parser  │
│ System  │         │  (syn)   │
└────┬────┘         └─────┬────┘
     │                    │
     └──────────┬─────────┘
                │
                ▼
         ┌──────────────┐
         │   Project    │
         │  Management  │
         └──────────────┘
```

## 🛠️ Technology Stack

- **GUI**: egui/eframe (immediate mode)
- **Parser**: syn (Rust AST)
- **File System**: walkdir, ignore
- **Build**: cargo integration
- **Error Handling**: anyhow
- **Async**: tokio

## 📦 Project Structure

```
vibe_rust_coder/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # GUI implementation
│   ├── command.rs       # Command system
│   ├── parser.rs        # Rust parser
│   └── project.rs       # Project management
├── Cargo.toml           # Dependencies
├── build.bat            # Build script
├── README.md            # User guide
├── QUICKSTART.md        # Quick start
├── EXAMPLES.md          # Usage examples
├── AI_WORKFLOW.md       # AI collaboration guide
├── ARCHITECTURE.md      # System design
└── PROJECT_SUMMARY.md   # Project overview
```

## 🎯 Use Cases

### ✅ Perfect For:
- AI-assisted Rust development
- Rapid prototyping
- Learning Rust with AI tutors
- Code exploration and analysis
- Quick build-test cycles

### 🎓 Great For Learning:
- Understanding Rust project structure
- Exploring unfamiliar codebases
- Practicing with AI guidance
- Quick experimentation

## 🤝 AI Collaboration

This tool is specifically designed to work with AI assistants like:
- ChatGPT
- Claude
- Gemini
- GitHub Copilot Chat
- Any AI coding assistant

### For AI Assistants:
When helping users with Vibe Rust Coder, format your responses as:
```
add into <file>
<code>
```

This allows users to copy-paste directly into the tool.

## 🐛 Troubleshooting

### Build Issues
```powershell
# Clean and rebuild
cargo clean
cargo build --release
```

### File Locking (Windows)
- Close any running instances
- Temporarily disable antivirus
- Try building again

### Project Not Loading
- Verify path contains `Cargo.toml`
- Check write permissions
- Ensure path is correct

## 📊 Build Status

✅ **Successfully Built**
- Build Time: ~38 seconds
- Profile: Release (optimized)
- Status: Production ready

## 🔮 Future Features

- [ ] Syntax highlighting in chat
- [ ] Code diff viewer
- [ ] Git integration
- [ ] Multiple project support
- [ ] Export chat history
- [ ] Custom themes
- [ ] Plugin system
- [ ] LSP integration

## 📝 License

MIT License - Feel free to use and modify!

## 🙏 Acknowledgments

Built with:
- [egui](https://github.com/emilk/egui) - Immediate mode GUI
- [syn](https://github.com/dtolnay/syn) - Rust parser
- [tokio](https://tokio.rs/) - Async runtime
- And many other amazing Rust crates!

## 📧 Support

For issues, questions, or suggestions:
1. Check the documentation files
2. Review the examples
3. Consult the architecture guide

## 🎉 Get Started Now!

```powershell
# Clone or navigate to the project
cd U:\godot\vibe_rust_coder

# Build it
cargo build --release

# Run it
cargo run --release

# Start coding with AI! 🚀
```

---

**Made with ❤️ for AI-assisted Rust development**

*Vibe Rust Coder - Where AI meets Rust productivity*
