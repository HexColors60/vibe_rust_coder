use crate::command::{Command, CommandExecutor};
use crate::project::Project;
use egui::{Color32, RichText, ScrollArea, TextEdit};
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: String,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Error,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SearchResult {
    pub file_path: String,
    pub line_number: Option<usize>,
    pub content: String,
}

pub struct VibeRustCoderApp {
    project: Option<Project>,
    project_path: String,
    command_input: String,
    chat_history: Vec<ChatMessage>,
    command_executor: CommandExecutor,
    auto_scroll: bool,
    search_results: Vec<SearchResult>,
    last_command: String,
    show_process_window: bool,
    process_text: String,
    process_analysis: String,
    selected_text: String,
}

impl Default for VibeRustCoderApp {
    fn default() -> Self {
        Self {
            project: None,
            project_path: String::new(),
            command_input: String::new(),
            chat_history: Vec::new(),
            command_executor: CommandExecutor::new(),
            auto_scroll: true,
            search_results: Vec::new(),
            last_command: String::new(),
            show_process_window: false,
            process_text: String::new(),
            process_analysis: String::new(),
            selected_text: String::new(),
        }
    }
}

impl VibeRustCoderApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn add_message(&mut self, role: MessageRole, content: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.chat_history.push(ChatMessage {
            role,
            content,
            timestamp,
        });
    }

    fn execute_command(&mut self, command_text: &str) {
        self.add_message(MessageRole::User, command_text.to_string());
        self.last_command = command_text.to_string();

        match Command::parse(command_text) {
            Ok(command) => {
                // Check if it's a search command to parse results
                let is_search = matches!(command, Command::Search { .. });
                
                let result = self.command_executor.execute(command, &mut self.project);
                match result {
                    Ok(output) => {
                        // Parse search results if it was a search command
                        if is_search {
                            self.parse_search_results(&output);
                        }
                        self.add_message(MessageRole::Assistant, output);
                    }
                    Err(e) => {
                        self.add_message(MessageRole::Error, format!("Error: {}", e));
                    }
                }
            }
            Err(e) => {
                self.add_message(MessageRole::Error, format!("Parse error: {}", e));
            }
        }
    }

    fn parse_search_results(&mut self, output: &str) {
        self.search_results.clear();
        
        for line in output.lines() {
            if line.starts_with("File: ") {
                let file_path = line[6..].to_string();
                self.search_results.push(SearchResult {
                    file_path: file_path.clone(),
                    line_number: None,
                    content: file_path,
                });
            } else if line.find(':').is_some() {
                if let Some(dash_pos) = line.find(" - ") {
                    let file_and_line = &line[..dash_pos];
                    if let Some(colon_pos) = file_and_line.rfind(':') {
                        let file_path = file_and_line[..colon_pos].trim().to_string();
                        let line_num = file_and_line[colon_pos + 1..].trim().parse::<usize>().ok();
                        let content = line[dash_pos + 3..].to_string();
                        
                        self.search_results.push(SearchResult {
                            file_path,
                            line_number: line_num,
                            content,
                        });
                    }
                }
            }
        }
    }

    fn load_project(&mut self) {
        let path = PathBuf::from(&self.project_path);
        match Project::load(path.clone()) {
            Ok(project) => {
                self.project = Some(project);
                self.add_message(
                    MessageRole::System,
                    format!("Project loaded from: {}", path.display()),
                );
            }
            Err(e) => {
                self.add_message(MessageRole::Error, format!("Failed to load project: {}", e));
            }
        }
    }

    fn analyze_text(&mut self) {
        let text = self.process_text.clone();
        let mut analysis = String::new();
        
        // Basic analysis
        analysis.push_str(&format!("📊 Text Analysis\n\n"));
        analysis.push_str(&format!("Length: {} characters\n", text.len()));
        analysis.push_str(&format!("Lines: {}\n", text.lines().count()));
        analysis.push_str(&format!("Words: {}\n\n", text.split_whitespace().count()));
        
        // Code detection
        if text.contains("fn ") || text.contains("struct ") || text.contains("impl ") {
            analysis.push_str("🦀 Detected: Rust code\n\n");
            
            // Count functions
            let fn_count = text.matches("fn ").count();
            let struct_count = text.matches("struct ").count();
            let impl_count = text.matches("impl ").count();
            
            if fn_count > 0 {
                analysis.push_str(&format!("Functions: {}\n", fn_count));
            }
            if struct_count > 0 {
                analysis.push_str(&format!("Structs: {}\n", struct_count));
            }
            if impl_count > 0 {
                analysis.push_str(&format!("Implementations: {}\n", impl_count));
            }
            analysis.push_str("\n");
        }
        
        // Extract identifiers
        let words: Vec<&str> = text.split_whitespace().collect();
        let unique_words: std::collections::HashSet<_> = words.iter().collect();
        analysis.push_str(&format!("Unique words: {}\n", unique_words.len()));
        
        self.process_analysis = analysis;
    }

    fn summarize_text(&mut self) {
        let text = self.process_text.clone();
        let lines: Vec<&str> = text.lines().collect();
        
        let mut summary = String::new();
        summary.push_str("📝 Summary\n\n");
        
        if lines.len() <= 5 {
            summary.push_str(&text);
        } else {
            // Show first 3 and last 2 lines
            for line in lines.iter().take(3) {
                summary.push_str(line);
                summary.push('\n');
            }
            summary.push_str(&format!("\n... ({} lines omitted) ...\n\n", lines.len() - 5));
            for line in lines.iter().skip(lines.len() - 2) {
                summary.push_str(line);
                summary.push('\n');
            }
        }
        
        self.process_analysis = summary;
    }

    fn create_patch(&mut self) {
        let text = self.process_text.clone();
        let mut patch = String::new();
        
        patch.push_str("--- Original\n");
        patch.push_str("+++ Modified\n");
        patch.push_str("@@ -1,1 +1,1 @@\n");
        
        for line in text.lines() {
            if !line.trim().is_empty() {
                patch.push_str(&format!(" {}\n", line));
            }
        }
        
        self.process_analysis = patch;
    }

    fn open_process_window(&mut self, text: String) {
        self.process_text = text;
        self.process_analysis.clear();
        self.show_process_window = true;
    }
}

impl eframe::App for VibeRustCoderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process Window (modal)
        if self.show_process_window {
            egui::Window::new("📋 Process Text")
                .default_width(600.0)
                .default_height(500.0)
                .show(ctx, |ui| {
                    ui.heading("Text Processing");
                    ui.separator();
                    
                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button("📊 Analyze").clicked() {
                            self.analyze_text();
                        }
                        if ui.button("📝 Summary").clicked() {
                            self.summarize_text();
                        }
                        if ui.button("🔧 Create Patch").clicked() {
                            self.create_patch();
                        }
                        if ui.button("📋 Copy All").clicked() {
                            ui.output_mut(|o| o.copied_text = self.process_text.clone());
                            self.add_message(MessageRole::System, "Text copied to clipboard".to_string());
                        }
                        if ui.button("❌ Close").clicked() {
                            self.show_process_window = false;
                        }
                    });
                    
                    ui.separator();
                    
                    // Original text (now editable and selectable)
                    ui.label(RichText::new("Original Text (editable, select and Ctrl+C to copy):").strong());
                    ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.process_text)
                                    .desired_width(f32::INFINITY)
                                    .code_editor()
                            );
                        });
                    
                    ui.separator();
                    
                    // Analysis results (now editable and selectable)
                    if !self.process_analysis.is_empty() {
                        ui.label(RichText::new("Analysis Results (editable, select and Ctrl+C to copy):").strong());
                        ui.horizontal(|ui| {
                            if ui.button("📋 Copy Analysis").clicked() {
                                ui.output_mut(|o| o.copied_text = self.process_analysis.clone());
                                self.add_message(MessageRole::System, "Analysis copied to clipboard".to_string());
                            }
                        });
                        
                        ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.process_analysis)
                                        .desired_width(f32::INFINITY)
                                        .code_editor()
                                );
                            });
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🦀 Vibe Rust Coder - AI Code Assistant");
            ui.separator();

            // Project path input
            ui.horizontal(|ui| {
                ui.label("Project Path:");
                ui.text_edit_singleline(&mut self.project_path);
                if ui.button("Load Project").clicked() {
                    self.load_project();
                }
                if self.project.is_some() {
                    ui.colored_label(Color32::GREEN, "✓ Loaded");
                }
            });

            ui.separator();

            // Quick command buttons
            ui.horizontal(|ui| {
                ui.label("Quick Commands:");
                
                if ui.button("📋 List Files").clicked() {
                    self.execute_command("list files");
                }
                
                if ui.button("🔨 Build").clicked() {
                    self.execute_command("build");
                }
                
                if ui.button("▶️ Run").clicked() {
                    self.execute_command("run");
                }
                
                if ui.button("🧪 Test").clicked() {
                    self.execute_command("test");
                }
                
                if ui.button("⚡ Profile").clicked() {
                    self.execute_command("profile");
                }
                
                if ui.button("❓ Help").clicked() {
                    self.execute_command("help");
                }
            });

            ui.separator();

            // Search results panel (if any)
            if !self.search_results.is_empty() {
                ui.collapsing("🔍 Search Results (Click to interact)", |ui| {
                    ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            let mut pending_command: Option<String> = None;
                            let mut pending_copy: Option<String> = None;
                            let mut pending_copy_path: Option<String> = None;
                            
                            for (idx, result) in self.search_results.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}.", idx + 1));
                                    
                                    // File path button
                                    if ui.button(&result.file_path).clicked() {
                                        pending_command = Some(format!("show {}", result.file_path));
                                    }
                                    
                                    if let Some(line_num) = result.line_number {
                                        ui.label(format!(":{}", line_num));
                                    }
                                    
                                    // Copy button
                                    if ui.button("📋 Copy").clicked() {
                                        pending_copy = Some(result.content.clone());
                                    }
                                    
                                    // Copy path button
                                    if ui.button("📁 Copy Path").clicked() {
                                        pending_copy_path = Some(result.file_path.clone());
                                    }
                                });
                                
                                ui.label(RichText::new(&result.content).small().color(Color32::GRAY));
                                ui.add_space(4.0);
                            }
                            
                            // Execute pending actions after iteration
                            if let Some(cmd) = pending_command {
                                self.execute_command(&cmd);
                            }
                            if let Some(text) = pending_copy {
                                ui.output_mut(|o| o.copied_text = text.clone());
                                self.add_message(MessageRole::System, 
                                    format!("Copied to clipboard: {}", text));
                            }
                            if let Some(path) = pending_copy_path {
                                ui.output_mut(|o| o.copied_text = path.clone());
                                self.add_message(MessageRole::System, 
                                    format!("Copied path: {}", path));
                            }
                        });
                });
                ui.separator();
            }

            // Chat history display
            ui.label("Chat History (editable - select text and Ctrl+C to copy):");
            let scroll_area = ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(self.auto_scroll)
                .max_height(ui.available_height() - 150.0);

            scroll_area.show(ui, |ui| {
                let mut pending_copy: Option<String> = None;
                let mut pending_process: Option<String> = None;
                let mut pending_analyze: Option<String> = None;
                
                for (_msg_idx, msg) in self.chat_history.iter_mut().enumerate() {
                    let (color, prefix) = match msg.role {
                        MessageRole::User => (Color32::LIGHT_BLUE, "👤 User"),
                        MessageRole::Assistant => (Color32::LIGHT_GREEN, "🤖 Assistant"),
                        MessageRole::System => (Color32::LIGHT_GRAY, "⚙️ System"),
                        MessageRole::Error => (Color32::LIGHT_RED, "❌ Error"),
                    };

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&msg.timestamp).color(Color32::DARK_GRAY));
                        ui.label(RichText::new(prefix).color(color).strong());
                    });

                    ui.add_space(4.0);
                    
                    // Make message content editable and selectable
                    let text_edit = egui::TextEdit::multiline(&mut msg.content)
                        .desired_width(f32::INFINITY)
                        .code_editor();
                    
                    let response = ui.add(text_edit);
                    
                    // Right-click context menu
                    response.context_menu(|ui| {
                        if ui.button("📋 Copy to Clipboard").clicked() {
                            pending_copy = Some(msg.content.clone());
                            ui.close_menu();
                        }
                        
                        if ui.button("🔧 Open in Process Window").clicked() {
                            pending_process = Some(msg.content.clone());
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("📊 Quick Analyze").clicked() {
                            pending_analyze = Some(msg.content.clone());
                            ui.close_menu();
                        }
                    });
                    
                    ui.add_space(8.0);
                    ui.separator();
                }
                
                // Execute pending actions after iteration
                if let Some(text) = pending_copy {
                    ui.output_mut(|o| o.copied_text = text.clone());
                    self.add_message(MessageRole::System, "Message copied to clipboard".to_string());
                }
                if let Some(text) = pending_process {
                    self.open_process_window(text);
                }
                if let Some(text) = pending_analyze {
                    let analysis = format!(
                        "Length: {} chars, {} lines, {} words",
                        text.len(),
                        text.lines().count(),
                        text.split_whitespace().count()
                    );
                    self.add_message(MessageRole::System, analysis);
                }
            });

            ui.add_space(10.0);

            // Command input
            ui.horizontal(|ui| {
                ui.label("Command:");
                let response = ui.add(
                    TextEdit::singleline(&mut self.command_input)
                        .desired_width(ui.available_width() - 100.0)
                        .hint_text("e.g., search npc.rs, add into npc.rs, build, run, test"),
                );

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let command = self.command_input.clone();
                    self.command_input.clear();
                    self.execute_command(&command);
                    response.request_focus();
                }

                if ui.button("Execute").clicked() {
                    let command = self.command_input.clone();
                    self.command_input.clear();
                    self.execute_command(&command);
                }
            });

            // Help text
            ui.add_space(5.0);
            ui.label(
                RichText::new("💡 Tip: Chat is editable! Select text with mouse and Ctrl+C to copy. Right-click for more options.")
                    .small()
                    .color(Color32::GRAY),
            );
        });
    }
}
