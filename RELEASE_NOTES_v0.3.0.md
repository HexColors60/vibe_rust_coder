# Version 0.3.0 - Process Window & Context Menu

## 🎉 New Features

### 1. **Right-Click Context Menu on Chat Messages** 🖱️

Every message in the chat history now has a right-click context menu with powerful options:

#### Available Options:
- **📋 Copy to Clipboard** - Copy the entire message to clipboard
- **🔧 Open in Process Window** - Open the message in the advanced process window
- **📊 Quick Analyze** - Get instant statistics (chars, lines, words)

#### How to Use:
```
1. Right-click on any message in chat history
2. Select an option from the menu
3. Action is performed instantly
```

### 2. **Process Window** 🔧

A powerful modal window for advanced text processing and analysis.

#### Features:
- **📊 Analyze** - Detailed text analysis including:
  - Character count
  - Line count
  - Word count
  - Rust code detection (functions, structs, implementations)
  - Unique word count
  
- **📝 Summary** - Smart text summarization:
  - Shows first 3 and last 2 lines for long text
  - Full text for short content
  - Line omission counter
  
- **🔧 Create Patch** - Generate patch format:
  - Standard diff format
  - Ready for version control
  - Clean line-by-line view
  
- **📋 Copy All** - Copy entire text to clipboard
- **📋 Copy Analysis** - Copy analysis results to clipboard
- **❌ Close** - Close the process window

#### How to Use:
```
1. Right-click any message
2. Select "🔧 Open in Process Window"
3. Use the action buttons:
   - Click "📊 Analyze" for detailed analysis
   - Click "📝 Summary" for quick summary
   - Click "🔧 Create Patch" for patch format
   - Click "📋 Copy All" to copy text
4. Click "❌ Close" when done
```

### 3. **Enhanced Text Selection** ✨

- Messages are now fully selectable
- Left-click and drag to select text
- Standard copy (Ctrl+C) works
- Right-click for advanced options

## 🎯 Use Cases

### Use Case 1: Analyzing AI Responses
```
1. AI provides code in chat
2. Right-click the message
3. Select "📊 Quick Analyze"
4. See: "Length: 450 chars, 15 lines, 75 words"
```

### Use Case 2: Processing Long Code Snippets
```
1. AI provides long code
2. Right-click → "🔧 Open in Process Window"
3. Click "📊 Analyze"
4. See detailed breakdown:
   - 🦀 Detected: Rust code
   - Functions: 3
   - Structs: 2
   - Implementations: 1
5. Click "📋 Copy Analysis" to save results
```

### Use Case 3: Creating Patches
```
1. Get code from AI
2. Right-click → "🔧 Open in Process Window"
3. Click "🔧 Create Patch"
4. Get patch format output
5. Click "📋 Copy Analysis"
6. Paste into version control
```

### Use Case 4: Quick Summary
```
1. Long build output in chat
2. Right-click → "🔧 Open in Process Window"
3. Click "📝 Summary"
4. See condensed version with first/last lines
```

## 📊 Process Window Analysis Examples

### Example 1: Rust Code Analysis
**Input:**
```rust
pub struct Player {
    pub name: String,
    pub health: f32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { name, health: 100.0 }
    }
}
```

**Analysis Output:**
```
📊 Text Analysis

Length: 156 characters
Lines: 10
Words: 24

🦀 Detected: Rust code

Functions: 1
Structs: 1
Implementations: 1

Unique words: 18
```

### Example 2: Summary
**Input:** (50 lines of code)

**Summary Output:**
```
📝 Summary

pub struct Player {
    pub name: String,
    pub health: f32,

... (45 lines omitted) ...

    }
}
```

### Example 3: Patch Format
**Input:**
```rust
fn hello() {
    println!("Hello!");
}
```

**Patch Output:**
```
--- Original
+++ Modified
@@ -1,1 +1,1 @@
 fn hello() {
     println!("Hello!");
 }
```

## 🎨 UI Improvements

### Context Menu
- Clean, modern design
- Icon-based options
- Instant feedback
- Auto-close on selection

### Process Window
- Modal overlay
- Resizable (600x500 default)
- Scrollable text areas
- Code editor styling
- Separate sections for input/output

## 🚀 Workflow Examples

### Workflow 1: Code Review
```
1. AI suggests code
2. Right-click → "🔧 Open in Process Window"
3. Click "📊 Analyze"
4. Review statistics
5. Click "📋 Copy All" if approved
6. Paste into your project
```

### Workflow 2: Error Analysis
```
1. Build error appears in chat
2. Right-click error message
3. Select "📊 Quick Analyze"
4. See error length and complexity
5. Right-click → "🔧 Open in Process Window"
6. Click "📝 Summary" for key points
```

### Workflow 3: Documentation
```
1. AI provides documentation
2. Right-click → "🔧 Open in Process Window"
3. Click "📊 Analyze"
4. Click "📋 Copy Analysis"
5. Add statistics to your docs
```

## 💡 Pro Tips

### Tip 1: Quick Copy
```
Right-click → "📋 Copy to Clipboard"
Faster than selecting and Ctrl+C!
```

### Tip 2: Analyze Before Using
```
Before using AI code:
1. Right-click → "🔧 Open in Process Window"
2. Click "📊 Analyze"
3. Check if it's Rust code
4. See complexity (function count, etc.)
```

### Tip 3: Compare Versions
```
1. Open first version in Process Window
2. Click "📊 Analyze"
3. Copy analysis
4. Open second version
5. Click "📊 Analyze"
6. Compare statistics
```

### Tip 4: Batch Processing
```
1. Right-click multiple messages
2. Open each in Process Window
3. Analyze all
4. Copy all analyses
5. Compare in external tool
```

## 🔧 Technical Details

### Context Menu Implementation
- Uses egui's `context_menu` feature
- Deferred action execution (borrow checker safe)
- Pending action queue
- Clean menu closure

### Process Window
- Modal window overlay
- Independent state management
- Real-time analysis
- Clipboard integration

### Text Analysis
- Character counting
- Line counting
- Word counting
- Rust code detection
- Pattern matching for keywords

## 📝 Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Select text | Left-click + drag |
| Copy selected | Ctrl+C |
| Context menu | Right-click |
| Close window | Click ❌ or outside |

## 🎯 Summary

Version 0.3.0 adds powerful text processing capabilities:

✅ **Right-click context menu** on all messages  
✅ **Process Window** with analysis tools  
✅ **Text analysis** (chars, lines, words, code detection)  
✅ **Smart summarization** for long content  
✅ **Patch generation** for version control  
✅ **Multiple copy options** for flexibility  
✅ **Enhanced text selection** throughout  

**Upgrade now for advanced text processing!** 🚀
