use eframe::{egui, App};
use egui_node_graph::*;
use serde::{Deserialize, Serialize};

/// Data stored in each of the nodes
///
/// Useful to store additional data that does not live in parameters
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub struct NodeData {
    weibull_cdf: Vec<f64>,
}

/// Node input parameters
///
/// Used to create Weibull distribution for the node
///
/// # Note:
/// There is a correspondence between the `scale` and
/// the number of `time_steps`. Reliability engineers 
/// using this tool will intuitivedly consider how the
/// scale of the Weibull distribution stretches out its
/// CDF over time. This time must then be reflected in 
/// the number of time steps. Think of `time_steps` as 
/// unitless measure of time that relates to the Weibull's scale
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub struct NodeParameters {
    /// Weibull shape
    shape: f64,
    /// Weibull scale
    scale: f64,
    /// Number of time steps to draw the CDF over
    time_steps: u32,
}

impl Default for NodeParameters {
    fn default() -> Self {
        Self {
            shape: 0.5,
            scale: 200.0,
            time_steps: 730,
        }
    }
}



pub struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl App for MyApp {
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

// Todo(Todd): Replace this with plotting CDF functions
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
