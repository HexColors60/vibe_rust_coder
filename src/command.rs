use crate::parser::RustParser;
use crate::project::Project;
use anyhow::{anyhow, Result};
use std::process::Command as ProcessCommand;

#[derive(Debug, Clone)]
pub enum Command {
    Search { query: String },
    AddInto { file: String, code: String },
    Build,
    Run { args: Vec<String> },
    Test { test_name: Option<String> },
    Profile,
    ListFiles,
    ShowFile { file: String },
    ShowFunction { file: String, function: String },
    ListFunctions { file: String },
    Help,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self> {
        let input = input.trim();
        let parts: Vec<&str> = input.splitn(2, ' ').collect();

        match parts[0].to_lowercase().as_str() {
            "search" => {
                let query = parts.get(1).ok_or_else(|| anyhow!("Missing search query"))?;
                Ok(Command::Search {
                    query: query.to_string(),
                })
            }
            "add" => {
                if parts.len() < 2 {
                    return Err(anyhow!("Usage: add into <file>\n<code>"));
                }
                let rest = parts[1];
                if rest.starts_with("into ") {
                    let file_and_code: Vec<&str> = rest[5..].splitn(2, '\n').collect();
                    let file = file_and_code[0].trim().to_string();
                    let code = file_and_code
                        .get(1)
                        .unwrap_or(&"")
                        .to_string();
                    Ok(Command::AddInto { file, code })
                } else {
                    Err(anyhow!("Expected 'add into <file>'"))
                }
            }
            "build" => Ok(Command::Build),
            "run" => {
                let args = parts
                    .get(1)
                    .map(|s| s.split_whitespace().map(String::from).collect())
                    .unwrap_or_default();
                Ok(Command::Run { args })
            }
            "test" => {
                let test_name = parts.get(1).map(|s| s.to_string());
                Ok(Command::Test { test_name })
            }
            "profile" => Ok(Command::Profile),
            "list" => {
                if let Some(rest) = parts.get(1) {
                    if rest.starts_with("files") {
                        Ok(Command::ListFiles)
                    } else if rest.starts_with("functions") {
                        let file_parts: Vec<&str> = rest.splitn(2, ' ').collect();
                        let file = file_parts
                            .get(1)
                            .ok_or_else(|| anyhow!("Missing file name"))?
                            .to_string();
                        Ok(Command::ListFunctions { file })
                    } else {
                        Err(anyhow!("Unknown list command"))
                    }
                } else {
                    Ok(Command::ListFiles)
                }
            }
            "show" => {
                let rest = parts.get(1).ok_or_else(|| anyhow!("Missing file name"))?;
                if rest.contains("::") {
                    let file_func: Vec<&str> = rest.splitn(2, "::").collect();
                    Ok(Command::ShowFunction {
                        file: file_func[0].to_string(),
                        function: file_func[1].to_string(),
                    })
                } else {
                    Ok(Command::ShowFile {
                        file: rest.to_string(),
                    })
                }
            }
            "help" => Ok(Command::Help),
            _ => Err(anyhow!("Unknown command: {}", parts[0])),
        }
    }
}

pub struct CommandExecutor {
    parser: RustParser,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            parser: RustParser::new(),
        }
    }

    pub fn execute(&mut self, command: Command, project: &mut Option<Project>) -> Result<String> {
        match command {
            Command::Search { query } => self.search(project, &query),
            Command::AddInto { file, code } => self.add_into(project, &file, &code),
            Command::Build => self.build(project),
            Command::Run { args } => self.run(project, args),
            Command::Test { test_name } => self.test(project, test_name),
            Command::Profile => self.profile(project),
            Command::ListFiles => self.list_files(project),
            Command::ShowFile { file } => self.show_file(project, &file),
            Command::ShowFunction { file, function } => self.show_function(project, &file, &function),
            Command::ListFunctions { file } => self.list_functions(project, &file),
            Command::Help => Ok(self.help()),
        }
    }

    fn search(&self, project: &Option<Project>, query: &str) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let results = project.search(query)?;
        
        if results.is_empty() {
            Ok(format!("No results found for '{}'", query))
        } else {
            let mut output = format!("Found {} result(s) for '{}':\n\n", results.len(), query);
            for (i, result) in results.iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, result));
            }
            Ok(output)
        }
    }

    fn add_into(&self, project: &mut Option<Project>, file: &str, code: &str) -> Result<String> {
        let project = project.as_mut().ok_or_else(|| anyhow!("No project loaded"))?;
        project.add_code(file, code)?;
        Ok(format!("Code added to {}", file))
    }

    fn build(&self, project: &Option<Project>) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let output = ProcessCommand::new("cargo")
            .arg("build")
            .current_dir(&project.root_path)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(format!("Build successful!\n\n{}{}", stdout, stderr))
        } else {
            Ok(format!("Build failed!\n\n{}{}", stdout, stderr))
        }
    }

    fn run(&self, project: &Option<Project>, args: Vec<String>) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let mut cmd = ProcessCommand::new("cargo");
        cmd.arg("run").current_dir(&project.root_path);
        
        if !args.is_empty() {
            cmd.arg("--").args(&args);
        }

        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(format!("Run output:\n\n{}{}", stdout, stderr))
    }

    fn test(&self, project: &Option<Project>, test_name: Option<String>) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let mut cmd = ProcessCommand::new("cargo");
        cmd.arg("test").current_dir(&project.root_path);

        if let Some(name) = test_name {
            cmd.arg(&name);
        }

        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(format!("Test output:\n\n{}{}", stdout, stderr))
    }

    fn profile(&self, project: &Option<Project>) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let output = ProcessCommand::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&project.root_path)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(format!("Profile build output:\n\n{}{}", stdout, stderr))
    }

    fn list_files(&self, project: &Option<Project>) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let files = project.list_rust_files()?;
        
        let mut output = format!("Found {} Rust file(s):\n\n", files.len());
        for (i, file) in files.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, file));
        }
        Ok(output)
    }

    fn show_file(&self, project: &Option<Project>, file: &str) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let content = project.read_file(file)?;
        Ok(format!("Content of {}:\n\n{}", file, content))
    }

    fn show_function(&self, project: &Option<Project>, file: &str, function: &str) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let content = project.read_file(file)?;
        let func_code = self.parser.extract_function(&content, function)?;
        Ok(format!("Function '{}' in {}:\n\n{}", function, file, func_code))
    }

    fn list_functions(&self, project: &Option<Project>, file: &str) -> Result<String> {
        let project = project.as_ref().ok_or_else(|| anyhow!("No project loaded"))?;
        let content = project.read_file(file)?;
        let functions = self.parser.list_functions(&content)?;
        
        let mut output = format!("Functions in {}:\n\n", file);
        for (i, func) in functions.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, func));
        }
        Ok(output)
    }

    fn help(&self) -> String {
        r#"Available Commands:

search <query>              - Search for files, functions, or variables
add into <file>             - Add code into a file (multiline)
build                       - Build the project with cargo build
run [args]                  - Run the project with cargo run
test [name]                 - Run tests with cargo test
profile                     - Build with --release for profiling
list files                  - List all Rust files in the project
list functions <file>       - List all functions in a file
show <file>                 - Show file contents
show <file>::<function>     - Show specific function
help                        - Show this help message

Examples:
  search npc.rs
  add into src/npc.rs
  build
  run --verbose
  test test_npc
  list files
  show src/main.rs
  show src/npc.rs::spawn_npc
"#.to_string()
    }
}
