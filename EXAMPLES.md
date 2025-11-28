# Vibe Rust Coder - Example Usage

This document shows practical examples of using Vibe Rust Coder with AI assistants.

## Scenario 1: Creating a New Game Entity

**AI Chat:**
```
User: I need to create an NPC system for my game

AI: I'll help you create an NPC module. First, let's create the file structure.

add into src/npc.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct Npc {
    pub name: String,
    pub health: f32,
    pub position: Vec3,
}

impl Npc {
    pub fn new(name: String, position: Vec3) -> Self {
        Self {
            name,
            health: 100.0,
            position,
        }
    }
    
    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
    }
}
```

**In Vibe Rust Coder:**
1. Copy the entire `add into src/npc.rs` command with code
2. Paste into command input
3. Press Enter
4. The code is automatically added to your project

## Scenario 2: Finding and Modifying Code

**Workflow:**
```
1. search spawn_npc
   → Shows all occurrences of "spawn_npc" in your project

2. show src/systems/spawn.rs
   → View the entire file

3. list functions src/systems/spawn.rs
   → See all functions in the file

4. show src/systems/spawn.rs::spawn_npc
   → View just the spawn_npc function
```

## Scenario 3: Build-Test-Fix Cycle

**AI Chat:**
```
User: The NPC system isn't compiling

AI: Let's check the build errors first.

build
```

**Response shows:**
```
Build failed!

error[E0433]: failed to resolve: use of undeclared type `Vec3`
  --> src/npc.rs:5:23
   |
5  |     pub position: Vec3,
   |                   ^^^^ not found in this scope
```

**AI continues:**
```
AI: You need to import Vec3. Let me add the import.

add into src/npc.rs
use bevy::math::Vec3;
```

**Then:**
```
build
→ Build successful!

test
→ All tests pass
```

## Scenario 4: Adding Tests

**AI Chat:**
```
AI: Let's add some tests for the NPC system.

add into src/npc.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_creation() {
        let npc = Npc::new("Guard".to_string(), Vec3::ZERO);
        assert_eq!(npc.name, "Guard");
        assert_eq!(npc.health, 100.0);
    }

    #[test]
    fn test_npc_damage() {
        let mut npc = Npc::new("Guard".to_string(), Vec3::ZERO);
        npc.take_damage(30.0);
        assert_eq!(npc.health, 70.0);
    }
}
```

**Run tests:**
```
test
→ running 2 tests
→ test npc::tests::test_npc_creation ... ok
→ test npc::tests::test_npc_damage ... ok
```

## Scenario 5: Project Exploration

**When starting with a new codebase:**
```
1. list files
   → See all Rust files in the project

2. search main
   → Find the main entry point

3. show src/main.rs
   → View the main file

4. list functions src/main.rs
   → See what functions exist

5. search TODO
   → Find all TODO comments
```

## Scenario 6: Performance Profiling

**AI Chat:**
```
User: The game is running slowly

AI: Let's build a release version and profile it.

profile
→ Builds with --release flag

run --release
→ Runs the optimized version
```

## Tips for AI Collaboration

### 1. Always Include the Command
When AI suggests code, it should format it as:
```
add into <file>
<code>
```

### 2. Chain Commands
You can execute multiple commands in sequence:
```
build
test
run
```

### 3. Use Search Before Adding
```
search Player
→ Check if Player struct already exists before creating it
```

### 4. Verify After Changes
```
add into src/player.rs
<code>

build
→ Immediately verify the code compiles
```

## Common Patterns

### Pattern 1: New Feature
```
1. search <feature_name>          # Check if it exists
2. add into src/<module>.rs       # Add the code
3. build                          # Verify compilation
4. test                           # Run tests
```

### Pattern 2: Debug Issue
```
1. search <error_keyword>         # Find related code
2. show <file>                    # View the file
3. add into <file>                # Fix the code
4. build                          # Verify fix
```

### Pattern 3: Code Review
```
1. list files                     # See project structure
2. show <file>                    # Review each file
3. list functions <file>          # Check function list
4. show <file>::<function>        # Deep dive into functions
```

## Integration with AI Assistants

### For Claude/ChatGPT/Gemini:

**Prompt Template:**
```
I'm using Vibe Rust Coder. When suggesting code changes, please format them as:

add into <filepath>
<code>

This allows me to copy-paste directly into the tool.
```

### Example AI Response:
```
I'll help you add a health regeneration system:

add into src/npc.rs
impl Npc {
    pub fn regenerate(&mut self, amount: f32) {
        self.health = (self.health + amount).min(100.0);
    }
}

After adding this, run:
build
test test_npc_regeneration
```

## Keyboard Shortcuts

- **Enter**: Execute command
- **Scroll**: Navigate chat history
- **Copy/Paste**: Standard Ctrl+C/Ctrl+V in command input

## Best Practices

1. **Always load project first** before running commands
2. **Use relative paths** from project root
3. **Build after adding code** to catch errors early
4. **Search before creating** to avoid duplicates
5. **Keep chat history** for reference and debugging

## Troubleshooting

### "No project loaded"
```
Solution: Enter project path and click "Load Project"
```

### "File not found"
```
Solution: Use 'list files' to see available files
         Use relative paths like 'src/main.rs'
```

### "Parse error"
```
Solution: Check command syntax with 'help'
         Ensure proper spacing and format
```

### Build fails
```
Solution: Read error messages in chat
         Use 'show <file>' to view problematic code
         Fix and rebuild
```
