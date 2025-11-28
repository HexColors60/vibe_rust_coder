# Quick Start Guide

## Building the Project

If you encounter file locking issues during build (common on Windows with antivirus):

### Option 1: Retry Build
```powershell
cargo clean
cargo build --release
```

### Option 2: Exclude from Antivirus
Add `U:\godot\vibe_rust_coder\target` to your antivirus exclusion list.

### Option 3: Use Different Target Directory
```powershell
$env:CARGO_TARGET_DIR="C:\temp\vibe_rust_coder_target"
cargo build --release
```

## Running the Application

Once built successfully:

```powershell
cargo run --release
```

Or run the executable directly:
```powershell
.\target\release\vibe_rust_coder.exe
```

## First Time Setup

1. **Launch the application**
2. **Load a Rust project**:
   - Enter the path to your Rust project (e.g., `U:\godot\my_game`)
   - Click "Load Project"
   - Wait for confirmation message

3. **Try your first command**:
   ```
   list files
   ```

4. **Search for something**:
   ```
   search main
   ```

5. **View a file**:
   ```
   show src/main.rs
   ```

## Common Build Issues

### Issue: "File is being used by another process"
**Solution**: 
- Close any running instances of the application
- Run `cargo clean`
- Temporarily disable antivirus
- Try building again

### Issue: "No project loaded"
**Solution**: 
- Make sure you've entered a valid project path
- Click "Load Project" button
- Check that the path contains a `Cargo.toml` file

### Issue: Command not recognized
**Solution**: 
- Type `help` to see all available commands
- Check command syntax in README.md

## Tips for Best Experience

1. **Keep the chat window scrolled**: Auto-scroll is enabled by default
2. **Use Enter key**: Press Enter to execute commands quickly
3. **Copy-paste from AI**: The tool is designed for copy-paste workflows
4. **Build frequently**: Run `build` after adding code to catch errors early

## Example Session

```
# Load project
[Enter path: U:\godot\my_game]
[Click Load Project]

# Explore the project
list files

# Search for specific code
search Player

# View a file
show src/player.rs

# Add new code (from AI suggestion)
add into src/player.rs
pub fn new_function() {
    println!("Hello!");
}

# Build and test
build
test
```

## Next Steps

- Read `README.md` for full documentation
- Check `EXAMPLES.md` for detailed usage scenarios
- Start collaborating with your AI assistant!

## Troubleshooting

If you encounter any issues:
1. Check the error message in the chat window
2. Try `cargo clean` and rebuild
3. Ensure your project path is correct
4. Verify Rust and Cargo are properly installed
5. Check that you have write permissions to the project directory
