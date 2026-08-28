//! Passo 1: só a janela. Sem estado, sem client, sem grafo — vem depois.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct EmptyApp;

impl eframe::App for EmptyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("graphviewer-gp — tela vazia por enquanto");
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 720.0])
            .with_title("graphviewer-gp"),
        ..Default::default()
    };

    eframe::run_native(
        "graphviewer-gp",
        options,
        Box::new(|_cc| Ok(Box::new(EmptyApp))),
    )
}
