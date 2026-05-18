mod data;
mod analysis;
mod jobs;
mod gui;
mod app;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MSV70 Analyzer  ·  N52B30 E85_v5  ·  9PPL921S")
            .with_inner_size([1480.0, 880.0])
            .with_min_inner_size([1100.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MSV70 Analyzer",
        native_options,
        Box::new(|cc| Ok(Box::new(app::MSV70App::new(cc)))),
    )
}
