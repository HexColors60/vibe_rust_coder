# Architecture Overview

## System Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Vibe Rust Coder                          │
│                   (GUI Application)                          │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   app.rs     │    │ command.rs   │    │  parser.rs   │
│              │    │              │    │              │
│ - GUI        │    │ - Parser     │    │ - syn crate  │
│ - Chat UI    │◄───┤ - Executor   │◄───┤ - AST visit  │
│ - Input      │    │ - Commands   │    │ - Extract    │
└──────────────┘    └──────────────┘    └──────────────┘
        │                   │                   │
        │                   ▼                   │
        │           ┌──────────────┐            │
        └──────────►│  project.rs  │◄───────────┘
                    │              │
                    │ - File scan  │
                    │ - Search     │
                    │ - Code add   │
                    └──────────────┘
                            │
                            ▼
                    ┌──────────────┐
                    │ Rust Project │
                    │  (Target)    │
                    └──────────────┘
```

## Data Flow

### 1. User Input Flow
```
User types command
    │
    ▼
GUI captures input (app.rs)
    │
    ▼
Command::parse() (command.rs)
    │
    ▼
CommandExecutor::execute()
    │
    ├─► Project operations (project.rs)
    │   ├─► File search
    │   ├─► Code insertion
    │   └─► File reading
    │
    ├─► Parser operations (parser.rs)
    │   ├─► AST parsing
    │   ├─► Function extraction
    │   └─► Structure analysis
    │
    └─► Cargo operations
        ├─► Build
        ├─► Test
        └─► Run
    │
    ▼
Result displayed in chat (app.rs)
```

### 2. Code Analysis Flow
```
Rust source file
    │
    ▼
syn::parse_file() (parser.rs)
    │
    ▼
AST (Abstract Syntax Tree)
    │
    ├─► FunctionVisitor
    │   └─► List all functions
    │
    ├─► StructVisitor
    │   └─► List all structs
    │
    └─► EnumVisitor
        └─► List all enums
    │
    ▼
Formatted output to user
```

### 3. Project Loading Flow
```
User enters path
    │
    ▼
Project::load() (project.rs)
    │
    ▼
WalkBuilder scans directory
    │
    ▼
Filter .rs files
    │
    ▼
Store file paths
    │
    ▼
Ready for commands
```

## Module Responsibilities

### app.rs (GUI Layer)
- **Responsibilities**:
  - Render GUI using egui
  - Handle user input
  - Display chat history
  - Manage application state
- **Key Types**:
  - `VibeRustCoderApp`: Main application struct
  - `ChatMessage`: Message in chat history
  - `MessageRole`: User/Assistant/System/Error

### command.rs (Command Layer)
- **Responsibilities**:
  - Parse command strings
  - Execute commands
  - Coordinate between modules
  - Handle cargo operations
- **Key Types**:
  - `Command`: Enum of all commands
  - `CommandExecutor`: Executes commands

### parser.rs (Analysis Layer)
- **Responsibilities**:
  - Parse Rust source code
  - Extract code elements
  - Analyze code structure
  - Use syn crate for AST
- **Key Types**:
  - `RustParser`: Main parser
  - `FunctionVisitor`: AST visitor for functions
  - `StructVisitor`: AST visitor for structs
  - `EnumVisitor`: AST visitor for enums

### project.rs (Project Layer)
- **Responsibilities**:
  - Manage project files
  - Search functionality
  - File I/O operations
  - Directory traversal
- **Key Types**:
  - `Project`: Project representation
  - File scanning
  - Search engine

## Command Processing Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Input: "search npc.rs"                                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Parse: Command::Search { query: "npc.rs" }              │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Execute: project.search("npc.rs")                       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Search:                                                  │
│    - Scan file names                                        │
│    - Scan file contents                                     │
│    - Collect matches                                        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Format results                                           │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 6. Display in chat UI                                       │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core Dependencies
- **egui/eframe**: Immediate mode GUI framework
- **syn**: Rust parser (proc-macro technology)
- **quote**: Code generation
- **walkdir/ignore**: File system traversal with gitignore support

### Utility Dependencies
- **tokio**: Async runtime (for future async operations)
- **anyhow**: Error handling
- **serde/serde_json**: Serialization
- **chrono**: Timestamps
- **regex**: Pattern matching

## Extension Points

### Adding New Commands
1. Add variant to `Command` enum in `command.rs`
2. Add parsing logic in `Command::parse()`
3. Add execution logic in `CommandExecutor::execute()`
4. Update help text

### Adding New Parsers
1. Create new visitor in `parser.rs`
2. Implement `Visit` trait from syn
3. Add extraction method to `RustParser`
4. Expose via command

### Adding New Project Features
1. Add method to `Project` struct in `project.rs`
2. Call from `CommandExecutor`
3. Return formatted results

## Performance Considerations

- **File Scanning**: Uses `ignore` crate for efficient gitignore-aware traversal
- **Parsing**: Lazy parsing - only parse when needed
- **Search**: In-memory search for small projects, could be optimized with indexing
- **GUI**: Immediate mode GUI (egui) is very efficient

## Future Enhancements

1. **Incremental Parsing**: Cache parsed ASTs
2. **Index Building**: Build search index for large projects
3. **LSP Integration**: Use Language Server Protocol
4. **Git Integration**: Show diffs, commit from GUI
5. **Multi-Project**: Support multiple projects simultaneously
6. **Plugins**: Plugin system for custom commands
