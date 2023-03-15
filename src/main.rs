use block::app::MyApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Block: Reliability Block Programming",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
