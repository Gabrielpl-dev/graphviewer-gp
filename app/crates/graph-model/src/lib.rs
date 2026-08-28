//! Tipos compartilhados entre o cliente HTTP/SSE (`langgraph-client`) e a GUI (`ui`) — nenhum dos
//! dois depende do outro diretamente, só desse contrato.

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum GraphEvent {
    GraphLoaded {
        assistant_id: String,
        nodes: Vec<String>,
        edges: Vec<(String, String)>,
    },
    NodeStarted { node: String, input: Value },
    NodeCompleted { node: String, output: Value },
    RunFinished,
    Error(String),
}
