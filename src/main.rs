use eframe::egui::{self, Color32};
use rfd::FileDialog;
use std::fs;
use std::path::{PathBuf};
use dirs::document_dir;

struct NotesApp {
    title: String,
    content: String,
    use_file_picker: bool,
}

impl Default for NotesApp {
    fn default() -> Self {
        Self {
            title: "New note".to_string(),
            content: String::new(),
            use_file_picker: true,
        }
    }
}

impl eframe::App for NotesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Notes");

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Note Title:");
                ui.text_edit_singleline(&mut self.title);
            });

            ui.add_space(10.0);

            ui.add_sized(
                [ui.available_width(), 500.0],
                egui::TextEdit::multiline(&mut self.content)
                .font(egui::TextStyle::Heading)
                .text_color(Color32::WHITE),
            );

            ui.add_space(10.0);
            ui.checkbox(&mut self.use_file_picker, "Use file picker");

            ui.add_space(10.0);

            ui.vertical(|ui| {
                let button_size = egui::vec2(140.0, 40.0);

                let save_color = egui::Color32::from_rgb(50, 150, 50);
                let load_color = egui::Color32::from_rgb(50, 50, 200);
                let clear_color = egui::Color32::from_rgb(200, 50, 50);

                if ui
                    .add(
                        egui::Button::new("💾 Save")
                            .fill(save_color)
                            .min_size(button_size),
                    )
                    .clicked()
                {
                    let path = if self.use_file_picker {
                        FileDialog::new()
                            .add_filter("Text", &["txt"])
                            .set_file_name(&format!("{}.txt", self.title))
                            .save_file()
                            .unwrap_or(default_note_path(&self.title))
                    } else {
                        default_note_path(&self.title)
                    };

                    let _ = fs::create_dir_all(path.parent().unwrap());
                    let _ = fs::write(path, &self.content);
                }

                ui.add_space(8.0);

                if ui
                    .add(
                        egui::Button::new("📂 Load")
                            .fill(load_color)
                            .min_size(button_size),
                    )
                    .clicked()
                {
                    let path = if self.use_file_picker {
                        FileDialog::new()
                            .add_filter("Text", &["txt"])
                            .pick_file()
                    } else {
                        Some(default_note_path(&self.title))
                    };

                    if let Some(path) = path {
                        if let Ok(content) = fs::read_to_string(&path) {
                            self.content = content;
                        }
                    }
                }

                ui.add_space(8.0);

                if ui
                    .add(
                        egui::Button::new("🧹 Clear")
                            .fill(clear_color)
                            .min_size(button_size),
                    )
                    .clicked()
                {
                    self.content.clear();
                }
            });
        });
    }
}

// Build a path like ~/Documents/rust_notes/title.txt
fn default_note_path(title: &str) -> PathBuf {
    let mut dir = document_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("rust_notes");
    dir.push(format!("{}.txt", title));
    dir
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0]) // This is the default window size
            .with_min_inner_size([500.0, 400.0]), 
        ..Default::default()
    };

    eframe::run_native(
        "Rust Notes",
        native_options,
        Box::new(|_cc| Box::new(NotesApp::default())),
    )
}
