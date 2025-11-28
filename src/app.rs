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

pub struct VibeRustCoderApp {
    project: Option<Project>,
    project_path: String,
    command_input: String,
    chat_history: Vec<ChatMessage>,
    command_executor: CommandExecutor,
    auto_scroll: bool,
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

        match Command::parse(command_text) {
            Ok(command) => {
                let result = self.command_executor.execute(command, &mut self.project);
                match result {
                    Ok(output) => {
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
}

impl eframe::App for VibeRustCoderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

            // Chat history display
            ui.label("Chat History:");
            let scroll_area = ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(self.auto_scroll)
                .max_height(ui.available_height() - 100.0);

            scroll_area.show(ui, |ui| {
                for msg in &self.chat_history {
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
                    ui.label(&msg.content);
                    ui.add_space(8.0);
                    ui.separator();
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
                RichText::new("Commands: search <file>, add into <file>, build, run, test, profile, list files, show <file>")
                    .small()
                    .color(Color32::GRAY),
            );
        });
    }
}
