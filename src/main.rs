mod app;
mod command;
mod parser;
mod project;

use anyhow::Result;
use tracing_subscriber;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Run the GUI application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Vibe Rust Coder - AI Code Assistant"),
        ..Default::default()
    };

    eframe::run_native(
        "Vibe Rust Coder",
        options,
        Box::new(|cc| Ok(Box::new(app::VibeRustCoderApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))?;

    Ok(())
}
