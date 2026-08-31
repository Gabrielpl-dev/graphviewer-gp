use std::collections::HashMap;

use egui::{Color32, RichText};
use graph_model::GraphEvent;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

use crate::graph_view;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Idle,
    Active,
    Done,
    /// Reservado — nenhum `GraphEvent` mapeia pra esse estado ainda (o servidor não distingue
    /// falha de um nó específico dos outros tipos de erro em `GraphEvent::Error`).
    #[allow(dead_code)]
    Error,
}

pub struct LogEntry {
    pub node: String,
    pub label: String,
    pub payload: String,
}

/// Comandos que a GUI manda pra thread que roda o tokio runtime.
pub enum Command {
    LoadGraph,
    RunGraph(Value),
}

pub struct GraphViewerApp {
    pub base_url: String,
    pub assistant_id: Option<String>,
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
    pub status: HashMap<String, NodeStatus>,
    pub input_json: String,
    pub log: Vec<LogEntry>,
    pub running: bool,
    pub status_msg: String,

    pub pan: egui::Vec2,
    pub zoom: f32,

    cmd_tx: UnboundedSender<Command>,
    event_rx: std::sync::mpsc::Receiver<GraphEvent>,
}

impl GraphViewerApp {
    pub fn new(
        cmd_tx: UnboundedSender<Command>,
        event_rx: std::sync::mpsc::Receiver<GraphEvent>,
    ) -> Self {
        let _ = cmd_tx.send(Command::LoadGraph);
        Self {
            base_url: "http://127.0.0.1:2024".to_string(),
            assistant_id: None,
            nodes: Vec::new(),
            edges: Vec::new(),
            status: HashMap::new(),
            input_json: r#"{"tarefa": "teste via graphviewer-gp", "etapa": "s0"}"#.to_string(),
            log: Vec::new(),
            running: false,
            status_msg: "carregando grafo...".to_string(),
            pan: egui::Vec2::ZERO,
            zoom: 1.0,
            cmd_tx,
            event_rx,
        }
    }

    fn drain_events(&mut self) {
        while let Ok(ev) = self.event_rx.try_recv() {
            match ev {
                GraphEvent::GraphLoaded { assistant_id, nodes, edges } => {
                    self.status = nodes.iter().map(|n| (n.clone(), NodeStatus::Idle)).collect();
                    self.nodes = nodes;
                    self.edges = edges;
                    self.assistant_id = Some(assistant_id);
                    self.status_msg = "grafo carregado.".to_string();
                }
                GraphEvent::NodeStarted { node, input } => {
                    self.status.insert(node.clone(), NodeStatus::Active);
                    self.log.insert(0, LogEntry {
                        node,
                        label: "iniciou".to_string(),
                        payload: serde_json::to_string_pretty(&input).unwrap_or_default(),
                    });
                }
                GraphEvent::NodeCompleted { node, output } => {
                    self.status.insert(node.clone(), NodeStatus::Done);
                    self.log.insert(0, LogEntry {
                        node,
                        label: "concluiu".to_string(),
                        payload: serde_json::to_string_pretty(&output).unwrap_or_default(),
                    });
                }
                GraphEvent::RunFinished => {
                    self.running = false;
                    self.status_msg = "execução concluída.".to_string();
                }
                GraphEvent::Error(msg) => {
                    self.running = false;
                    self.status_msg = format!("erro: {msg}");
                }
            }
        }
    }

    fn reset_run(&mut self) {
        for v in self.status.values_mut() {
            *v = NodeStatus::Idle;
        }
        self.log.clear();
    }
}

impl eframe::App for GraphViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain_events();
        if self.running {
            ctx.request_repaint();
        }

        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(420.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("server:");
                    ui.text_edit_singleline(&mut self.base_url);
                    if ui.button("recarregar").clicked() {
                        self.status_msg = "carregando grafo...".to_string();
                        let _ = self.cmd_tx.send(Command::LoadGraph);
                    }
                });
                ui.add_space(8.0);

                graph_view::show(ui, self);

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(RichText::new("INPUT").strong().color(Color32::GRAY));
                ui.add_space(4.0);
                ui.text_edit_multiline(&mut self.input_json);
                ui.add_space(6.0);

                let run_clicked = ui
                    .add_enabled(!self.running, egui::Button::new("rodar"))
                    .clicked();

                if run_clicked {
                    match serde_json::from_str::<Value>(&self.input_json) {
                        Ok(input) => {
                            self.reset_run();
                            self.running = true;
                            self.status_msg = "rodando...".to_string();
                            let _ = self.cmd_tx.send(Command::RunGraph(input));
                        }
                        Err(e) => {
                            self.status_msg = format!("JSON de entrada inválido: {e}");
                        }
                    }
                }

                ui.add_space(6.0);
                ui.label(RichText::new(&self.status_msg).color(Color32::GRAY));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("EVENTOS AO VIVO").strong().color(Color32::GRAY));
            ui.add_space(8.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.log {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&entry.node).color(Color32::from_rgb(124, 140, 255)).strong());
                        ui.label(format!("— {}", entry.label));
                    });
                    ui.label(RichText::new(&entry.payload).color(Color32::GRAY).small());
                    ui.separator();
                }
            });
        });
    }
}
