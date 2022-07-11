#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Oxidized Selenium",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc)))
    );
}
#[derive(Default)]
struct MyEguiApp {}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
} 

pub trait Demo {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback. https://www.egui.rs/#demo
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            // Window title
            ui.heading("Oxidized Selenium v0.1.0");

            // Context Menu
            ui.horizontal (|ui| {
                ui.menu_button("File", Self::file_menu);
                ui.menu_button("Test", Self::test_menu);
                ui.menu_button("About", Self::about_menu);
            });
       });
   }
}

impl MyEguiApp {
    fn file_menu(ui: &mut egui::Ui) {
        if ui.button("Open...").clicked() {
            ui.close_menu();
        }
    }

    fn test_menu(ui: &mut egui::Ui) {
        if ui.button("New Test").clicked() {
            ui.close_menu();
        }
    }

    fn about_menu(ui: &mut egui::Ui) {
        if ui.button("About Oxidized Selenium").clicked() {
            egui::Window::new("About")
            .default_width(320.0)
            .open(&mut true);
        }
    }
}

pub struct CodeEditor {
    language: String,
    code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            language: "rs".into(),
            code: "// A very simple example\n\
fn main() {\n\
\tprintln!(\"Hello world!\");\n\
}\n\
"
            .into(),
        }
    }
}
