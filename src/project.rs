use anyhow::{anyhow, Result};
use ignore::WalkBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use crate::patch::CodeInserter;

pub struct Project {
    pub root_path: PathBuf,
    rust_files: Vec<PathBuf>,
}

impl Project {
    pub fn load(root_path: PathBuf) -> Result<Self> {
        if !root_path.exists() {
            return Err(anyhow!("Path does not exist: {}", root_path.display()));
        }

        let mut project = Self {
            root_path: root_path.clone(),
            rust_files: Vec::new(),
        };

        project.scan_rust_files()?;
        Ok(project)
    }

    fn scan_rust_files(&mut self) -> Result<()> {
        self.rust_files.clear();

        for entry in WalkBuilder::new(&self.root_path)
            .hidden(false)
            .git_ignore(true)
            .build()
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                self.rust_files.push(path.to_path_buf());
            }
        }

        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<String>> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        // Search in file names
        for file in &self.rust_files {
            if let Some(file_name) = file.file_name().and_then(|s| s.to_str()) {
                if file_name.to_lowercase().contains(&query_lower) {
                    results.push(format!("File: {}", self.relative_path(file)?));
                }
            }
        }

        // Search in file contents
        for file in &self.rust_files {
            if let Ok(content) = fs::read_to_string(file) {
                for (line_num, line) in content.lines().enumerate() {
                    if line.to_lowercase().contains(&query_lower) {
                        results.push(format!(
                            "{}:{} - {}",
                            self.relative_path(file)?,
                            line_num + 1,
                            line.trim()
                        ));
                    }
                }
            }
        }

        Ok(results)
    }

    pub fn add_code(&mut self, file_path: &str, code: &str) -> Result<()> {
        let full_path = self.root_path.join(file_path);

        // Create parent directories if they don't exist
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Use CodeInserter for intelligent code insertion
        let inserter = CodeInserter::new();
        let mut existing_content = String::new();
        let file_existed = full_path.exists();

        if file_existed {
            existing_content = fs::read_to_string(&full_path)?;
        }

        let new_content = inserter.insert_code(&existing_content, code)?;
        fs::write(&full_path, new_content)?;

        // If this is a new module file, try to add module declaration to main.rs or lib.rs
        if !file_existed && file_path.starts_with("src/") && file_path.ends_with(".rs") {
            self.add_module_declaration(file_path)?;
        }

        // Rescan files
        self.scan_rust_files()?;

        Ok(())
    }

    fn add_module_declaration(&mut self, file_path: &str) -> Result<()> {
        // Extract module name from file path
        if let Some(module_name) = file_path
            .strip_prefix("src/")
            .and_then(|s| s.strip_suffix(".rs"))
            .and_then(|s| s.split('/').last())
        {
            // Try to add to main.rs first, then lib.rs
            for root_file in ["src/main.rs", "src/lib.rs"] {
                let root_path = self.root_path.join(root_file);
                if root_path.exists() {
                    let content = fs::read_to_string(&root_path)?;

                    // Check if module is already declared
                    if !content.contains(&format!("mod {};", module_name)) {
                        let inserter = CodeInserter::new();
                        let module_decl = format!("mod {};", module_name);
                        let updated_content = inserter.insert_code(&content, &module_decl)?;
                        fs::write(&root_path, updated_content)?;
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn read_file(&self, file_path: &str) -> Result<String> {
        let full_path = self.root_path.join(file_path);
        
        if !full_path.exists() {
            // Try to find the file in rust_files
            for rust_file in &self.rust_files {
                if rust_file.ends_with(file_path) {
                    return fs::read_to_string(rust_file)
                        .map_err(|e| anyhow!("Failed to read file: {}", e));
                }
            }
            return Err(anyhow!("File not found: {}", file_path));
        }

        fs::read_to_string(&full_path).map_err(|e| anyhow!("Failed to read file: {}", e))
    }

    pub fn list_rust_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        for file in &self.rust_files {
            files.push(self.relative_path(file)?);
        }
        Ok(files)
    }

    fn relative_path(&self, path: &Path) -> Result<String> {
        path.strip_prefix(&self.root_path)
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| anyhow!("Failed to get relative path: {}", e))
    }

    pub fn get_directory_structure(&self) -> Result<String> {
        let mut structure = String::new();
        self.build_tree(&self.root_path, &mut structure, "", true)?;
        Ok(structure)
    }

    fn build_tree(&self, path: &Path, output: &mut String, prefix: &str, is_last: bool) -> Result<()> {
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let connector = if is_last { "└── " } else { "├── " };
        output.push_str(&format!("{}{}{}\n", prefix, connector, file_name));

        if path.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(path)?
                .filter_map(|e| e.ok())
                .collect();
            entries.sort_by_key(|e| e.path());

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

            for (i, entry) in entries.iter().enumerate() {
                let is_last_entry = i == entries.len() - 1;
                self.build_tree(&entry.path(), output, &new_prefix, is_last_entry)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_project_load() {
        let temp_dir = TempDir::new().unwrap();
        let project = Project::load(temp_dir.path().to_path_buf());
        assert!(project.is_ok());
    }

    #[test]
    fn test_search() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        let project = Project::load(temp_dir.path().to_path_buf()).unwrap();
        let results = project.search("main").unwrap();
        assert!(!results.is_empty());
    }
}
