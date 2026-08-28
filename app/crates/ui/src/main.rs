#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod graph_view;

use app::{Command, GraphViewerApp};
use graph_model::GraphEvent;

fn main() -> eframe::Result<()> {
    let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::unbounded_channel::<Command>();
    let (event_tx, event_rx) = std::sync::mpsc::channel::<GraphEvent>();

    // thread própria pro runtime async — a GUI (eframe) roda em modo imediato na main thread
    // e nunca pode bloquear esperando rede.
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("falha criando tokio runtime");
        rt.block_on(async move {
            let base_url = "http://127.0.0.1:2024".to_string();
            let mut nodes: Vec<String> = Vec::new();
            let mut assistant_id: Option<String> = None;

            while let Some(cmd) = cmd_rx.recv().await {
                match cmd {
                    Command::LoadGraph => {
                        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                        langgraph_client::load_graph(&base_url, &tx).await;
                        drop(tx);
                        let mut rx = rx;
                        while let Ok(ev) = rx.try_recv() {
                            if let GraphEvent::GraphLoaded { assistant_id: id, nodes: ns, .. } = &ev {
                                assistant_id = Some(id.clone());
                                nodes = ns.clone();
                            }
                            let _ = event_tx.send(ev);
                        }
                    }
                    Command::RunGraph(input) => {
                        if let Some(id) = assistant_id.clone() {
                            langgraph_client::run_graph(
                                base_url.clone(),
                                id,
                                input,
                                nodes.clone(),
                                to_unbounded(event_tx.clone()),
                            )
                            .await;
                        } else {
                            let _ = event_tx.send(GraphEvent::Error(
                                "grafo ainda não carregado".to_string(),
                            ));
                        }
                    }
                }
            }
        });
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 720.0])
            .with_title("graphviewer-gp"),
        ..Default::default()
    };

    eframe::run_native(
        "graphviewer-gp",
        options,
        Box::new(|_cc| Ok(Box::new(GraphViewerApp::new(cmd_tx, event_rx)))),
    )
}

/// `run_graph` espera um `UnboundedSender<GraphEvent>` (tokio); a GUI recebe por um
/// `std::sync::mpsc::Sender`. Essa ponte evita que `langgraph-client` precise saber qual canal
/// a GUI usa por baixo.
fn to_unbounded(
    std_tx: std::sync::mpsc::Sender<GraphEvent>,
) -> tokio::sync::mpsc::UnboundedSender<GraphEvent> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<GraphEvent>();
    tokio::spawn(async move {
        while let Some(ev) = rx.recv().await {
            let _ = std_tx.send(ev);
        }
    });
    tx
}
