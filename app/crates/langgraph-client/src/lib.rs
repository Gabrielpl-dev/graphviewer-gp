//! Fala HTTP/SSE com o servidor `langgraph dev`. A GUI nunca faz essa chamada
//! diretamente — só recebe `GraphEvent` por um canal.

use graph_model::GraphEvent;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Deserialize)]
struct Assistant {
    assistant_id: String,
}

#[derive(Debug, Deserialize)]
struct GraphNode {
    id: String,
}

#[derive(Debug, Deserialize)]
struct GraphEdge {
    source: String,
    target: String,
}

#[derive(Debug, Deserialize)]
struct GraphStructure {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

/// Busca o primeiro assistant disponível e a estrutura do grafo dele.
pub async fn load_graph(base_url: &str, tx: &UnboundedSender<GraphEvent>) {
    let client = reqwest::Client::new();

    let assistants: Vec<Assistant> = match client
        .post(format!("{base_url}/assistants/search"))
        .json(&serde_json::json!({}))
        .send()
        .await
    {
        Ok(resp) => match resp.json().await {
            Ok(a) => a,
            Err(e) => {
                let _ = tx.send(GraphEvent::Error(format!("resposta inválida de /assistants/search: {e}")));
                return;
            }
        },
        Err(e) => {
            let _ = tx.send(GraphEvent::Error(format!("não consegui falar com {base_url}: {e}")));
            return;
        }
    };

    let Some(assistant) = assistants.into_iter().next() else {
        let _ = tx.send(GraphEvent::Error("nenhum assistant encontrado no servidor".into()));
        return;
    };

    let graph: GraphStructure = match client
        .get(format!("{base_url}/assistants/{}/graph", assistant.assistant_id))
        .send()
        .await
    {
        Ok(resp) => match resp.json().await {
            Ok(g) => g,
            Err(e) => {
                let _ = tx.send(GraphEvent::Error(format!("resposta inválida de /graph: {e}")));
                return;
            }
        },
        Err(e) => {
            let _ = tx.send(GraphEvent::Error(format!("erro buscando grafo: {e}")));
            return;
        }
    };

    let nodes: Vec<String> = graph
        .nodes
        .into_iter()
        .map(|n| n.id)
        .filter(|id| id != "__start__" && id != "__end__")
        .collect();

    let edges: Vec<(String, String)> = graph
        .edges
        .into_iter()
        .map(|e| (e.source, e.target))
        .filter(|(s, t)| {
            s != "__start__" && s != "__end__" && t != "__start__" && t != "__end__"
        })
        .collect();

    let _ = tx.send(GraphEvent::GraphLoaded {
        assistant_id: assistant.assistant_id,
        nodes,
        edges,
    });
}

/// Roda o grafo e transmite os eventos node-a-node conforme chegam via SSE.
pub async fn run_graph(
    base_url: String,
    assistant_id: String,
    input: Value,
    node_ids: Vec<String>,
    tx: UnboundedSender<GraphEvent>,
) {
    let client = reqwest::Client::new();

    let resp = match client
        .post(format!("{base_url}/runs/stream"))
        .json(&serde_json::json!({
            "assistant_id": assistant_id,
            "input": input,
            "stream_mode": ["events"],
        }))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            let _ = tx.send(GraphEvent::Error(format!("erro iniciando run: {e}")));
            return;
        }
    };

    use futures_util::StreamExt;
    let mut stream = resp.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = match chunk {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(GraphEvent::Error(format!("erro lendo stream: {e}")));
                return;
            }
        };

        buffer.push_str(&String::from_utf8_lossy(&chunk).replace("\r\n", "\n"));

        while let Some(pos) = buffer.find("\n\n") {
            let raw_event: String = buffer.drain(..pos + 2).collect();
            let raw_event = raw_event.trim();

            let mut event_type = None;
            let mut data_line = None;
            for line in raw_event.lines() {
                if let Some(v) = line.strip_prefix("event:") {
                    event_type = Some(v.trim().to_string());
                } else if let Some(v) = line.strip_prefix("data:") {
                    data_line = Some(v.trim().to_string());
                }
            }

            let (Some(event_type), Some(data_line)) = (event_type, data_line) else {
                continue;
            };
            if event_type != "events" {
                continue;
            }

            let Ok(data): Result<Value, _> = serde_json::from_str(&data_line) else {
                continue;
            };

            let node_name = data
                .get("metadata")
                .and_then(|m| m.get("langgraph_node"))
                .and_then(|v| v.as_str())
                .or_else(|| data.get("name").and_then(|v| v.as_str()))
                .unwrap_or_default()
                .to_string();

            if !node_ids.contains(&node_name) {
                continue;
            }

            let kind = data.get("event").and_then(|v| v.as_str()).unwrap_or_default();
            match kind {
                "on_chain_start" => {
                    let input = data
                        .get("data")
                        .and_then(|d| d.get("input"))
                        .cloned()
                        .unwrap_or(Value::Null);
                    let _ = tx.send(GraphEvent::NodeStarted { node: node_name, input });
                }
                "on_chain_end" => {
                    let output = data
                        .get("data")
                        .and_then(|d| d.get("output"))
                        .cloned()
                        .unwrap_or(Value::Null);
                    let _ = tx.send(GraphEvent::NodeCompleted { node: node_name, output });
                }
                _ => {}
            }
        }
    }

    let _ = tx.send(GraphEvent::RunFinished);
}
