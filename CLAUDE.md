# graphviwer-gp

Viewer nativo (Rust, egui/eframe) pra acompanhar execução de grafos LangGraph ao vivo — nasceu
como alternativa self-hosted ao LangGraph Studio (que exige conta LangSmith só pra rodar uma
execução). Arquitetura por camadas: `crates/ui` (GUI, egui) fica separado de `crates/langgraph-client`
(HTTP/SSE) e `crates/graph-model` (tipos compartilhados) conforme forem sendo criados — mudar a UI
não deve forçar recompilar o client.

## Antivírus/EDR bloqueia o .exe compilado

O `target/release/ui.exe` (e também builds debug) é apagado/bloqueado pelo antivírus corporativo
minutos depois de compilado — falso positivo comum em binários Rust/egui não assinados (usa
`accesskit`/`clipboard-win`/criação de janela nativa, heurística lê como malware). Sem solução
permanente ainda (depende de TI liberar a pasta ou confiar num certificado — ver conversa de
2026-08-27).

**Enquanto isso não for resolvido: sempre rode o app via terminal (`cargo run -p ui` ou
`cargo run --release -p ui`), nunca dependa do `.exe` compilado ficar disponível pra clique.**
Rodar direto do `cargo run` evita o problema porque o binário some do disco só depois de já ter
sido lançado como processo.
