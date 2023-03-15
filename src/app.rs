use eframe::egui;

pub struct MyApp {
    pub name: String,
    pub age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Todd".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);

        egui::TopBottomPanel::top("vis panel")
            .resizable(true)
            .show(ctx, |ui| {
                example_plot(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO(Todd): Add RBD nodes
        });
    }
}

fn example_plot(ui: &mut egui::Ui) -> egui::Response {
    use egui::plot::{Line, PlotPoints};
    let n = 128;
    let line_points: PlotPoints = (0..=n)
        .map(|i| {
            use std::f64::consts::TAU;
            let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
            [x, x.sin()]
        })
        .collect();
    let line = Line::new(line_points);
    egui::plot::Plot::new("example_plot")
        .height(300.0)
        .data_aspect(1.0)
        .show(ui, |plot_ui| plot_ui.line(line))
        .response
}
