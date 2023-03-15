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
    pub shape: f64,
    /// Weibull scale
    pub scale: f64,
    /// Number of time steps to draw the CDF over
    pub time_steps: u32,
}

/// Default parameters of our nodes
///
/// When a node is created, we start with default 
/// parameters. This will give us something to immediately
/// render and use.
impl Default for NodeParameters {
    fn default() -> Self {
        Self {
            shape: 0.5,
            scale: 200.0,
            time_steps: 730,
        }
    }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub enum NodeType {
    Component,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub enum ValueType {
    Component { value: Vec<f64> }
}

/// The Graph's global state
///
/// This is passed between nodes, allowing us to 
/// highlight which node is active. this is useful
/// when rendering our Weibull CDFs over time for 
/// each of the nodes.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub struct GraphState {
    pub active_node: Option<NodeId>
}

/// Node Template
///
/// Represents the possible types of nodes we can create
#[derive(Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub enum NodeTemplate {
    CreateComponent,
}

impl NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = NodeType;
    type ValueType = ValueType;
    type UserState = GraphState;

    /// Todo(): implement missing functions
}

/// Our application
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
