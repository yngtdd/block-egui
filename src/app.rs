use std::borrow::Cow;
use eframe::{egui, App};
use egui_node_graph::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Weibull};

/// Data stored in each of the nodes
///
/// Useful to store additional data that does not live in parameters
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub struct NodeData {
    template: NodeTemplate,
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub struct WeibullModel {
    model: Weibull,
    reliability: Vec<f64>,
}

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
pub enum NodeType {
    Component,
}

// #[derive(Clone, Debug)]
// #[cfg_attr(feature = "persistence", derive(Deserialize, Serialize))]
// pub enum ValueType {
//     Component { value: Vec<f64> }
// }

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

type MyGraph = Graph<NodeData, NodeType, NodeParameters>;
type MyEditorState = 
    GraphEditorState<NodeData, NodeType, NodeParameters, NodeTemplate, GraphState>;

impl NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = NodeType;
    type ValueType = NodeParameters;
    type UserState = GraphState;

    /// Label in our menu selection
    fn node_finder_label(&self, user_state: &mut Self::UserState) -> Cow<str> {
        Cow::Borrowed(match self {
            NodeTemplate::CreateComponent => "Create Component"
        })
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_graph_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        // TODO(Todd): figure out how to combine NodeParameters and NodeData here to 
        // produce a Weibull CDF
        NodeData { template: *self }
    }

    fn build_node(
            &self,
            graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
            _user_state: &mut Self::UserState,
            node_id: NodeId,
        ) {
        let node_input = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeType::Component,
                NodeParameters { shape: 0.5, scale: 200.0, time_steps: 730 },
                InputParamKind::ConnectionOnly,
                true
            )
        };

        let node_output = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeType::Component)
        };

        match self {
           NodeTemplate::CreateComponent => {
                node_input(graph, "A");
                node_input(graph, "B");
                node_output(graph, "out");
            } 
        }
        
    }
}

pub struct AllNodeTemplates;

/// Iterator for 
impl NodeTemplateIter for AllNodeTemplates {
    type Item = NodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![NodeTemplate::CreateComponent]
    }
}

#[derive(Default)]
pub struct NodeGraphApp {
    state: MyEditorState,
    user_state: GraphState,
}

#[cfg(feature = "persistence")]
impl NodeGraphApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();

        Self {
            state,
            user_state: GraphState::default(),
        }

    }
}

impl App for NodeGraphApp {
    #[cfg(feature = "persistence")]

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui)
            });
        });

        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state
                    .draw_graph_editor(ui, AllNodeTemplates, &mut self.user_state)
            })
            .inner;

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    MyResponse::SetActiveNode(node) => self.user_state.activate_node = Some(node),
                    MyResponse::ClearActiveNode => self.user_state.activate_node = None,
                }
            }
        }

        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let text = match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
                    Ok(value) => format!("The result is {:?}", value),
                    Err(err) => format!("Execution error: {}", err),
                };

                ctx.debug_painter().text(
                    egui::pos2(10.0, 35.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
            } else {
                self.user_state.active_node = None;
            }
        }
    }
}

type OutputsCache = HashMap<OutputId, NodeType>;

pub fn evaluate_node(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<NodeType> {
    ///
    struct Evaluator<'a> {
        graph: &'a MyGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    }

    impl<'a> Evaluator<'a> {
        fn new(graph: &'a MyGraph, outputs_cache: &'a mut OutputsCache, node_id: NodeId) -> Self {
            Self {
                graph, outputs_cache,
                node_id,
            }
        }

        fn evaluate_input(&mut self, name: &str) -> anyhow::Result<NodeType> {
            evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
        }

        fn populate_output(
            &mut self, 
            name: &str,
            value: NodeType,
        ) -> anyhow::Result<NodeType> {
            populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
        }

        fn node_input(&mut self, name: &str) -> anyhow::Result<NodeType> {
            Ok(self.evaluate_input(name).expect("failed"))
        }

        fn node_output(&mut self, name: &str, value: f64) -> anyhow::Result<NodeType> {
            self.populate_output(name, NodeType::Component)
        }
    }

    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        NodeTemplate::CreateComponent => {
            let a = evaluator.node_input("A")?;
            let b = evaluator.node_input("B")?;
            evaluator.node_input("out")
        }
    }
}

fn populate_output(
    graph: &MyGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: NodeType,
) -> anyhow::Result<NodeType> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value);
    Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
    graph: &MyGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<NodeType> {
    let input_id = graph[node_id].get_input(param_name)?;

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok(*other_value)
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

            // Now that we know the value is cached, return it
            Ok(*outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated"))
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value)
    }
}

// /// Our application
// pub struct MyApp {}

// impl Default for MyApp {
//     fn default() -> Self {
//         Self {}
//     }
// }

// impl App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);

//         egui::TopBottomPanel::top("vis panel")
//             .resizable(true)
//             .show(ctx, |ui| {
//                 example_plot(ui);
//             });

//         egui::CentralPanel::default().show(ctx, |ui| {
//             // TODO(Todd): Add RBD nodes
//         });
//     }
// }

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
