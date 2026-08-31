# Contribuindo

Projeto pequeno, sem processo burocrático. Regras práticas:

## Sugestões e bugs

Abre uma issue. Pra sugestão de melhoria, descreve o problema que você tá tentando resolver, não
só a solução que você imaginou — às vezes tem um jeito mais simples de chegar lá.

## Pull requests

- Um PR, uma mudança lógica — evita misturar refatoração com feature nova.
- CI roda automaticamente por assunto (`.github/workflows/`) — só o stack que você mexeu:
  - Rust (`app/`): `cargo build`, `cargo test`, `cargo clippy -- -D warnings`.
  - Extensão (`vscode-extension/`): `npm test` (compila com `tsc --strict` e roda os testes em
    `src/test/`).
  - Python (`python/graphviewer/`): `pytest tests/`.
- Se mexer no pacote Python (`python/graphviewer/`): roda o scanner contra `examples/` e confirma
  que ainda acha as funções marcadas (`python -m graphviewer.scanner examples`), além do `pytest`.
- Se mexer no servidor MCP (`vscode-extension/mcp-server/`): instala isolado (`pip install -e
  vscode-extension/mcp-server`) e testa as 4 tools contra um projeto LangGraph real — sem precisar
  abrir VS Code.
- Explica o *porquê* da mudança na descrição do PR, não só o que mudou — o diff já mostra o que
  mudou.

## Decisões de arquitetura que já foram tomadas (não reabrir sem motivo forte)

- **Sem parsing estático de código** pra descobrir o grafo — sempre via API do `langgraph dev`
  rodando de verdade. É a decisão central do projeto (ver README).
- **Extensão VS Code**: o webview fala com o `langgraph dev` local diretamente (fetch/SSE) — a
  extensão só gerencia o processo do servidor e a URL, não faz proxy dos dados.
- **App Rust**: arquitetura em camadas (crates separados) pra mudar UI sem recompilar o cliente
  HTTP — `app/crates/graph-model` (tipos), `app/crates/langgraph-client` (HTTP/SSE),
  `app/crates/ui` (egui).
- **Sem acoplamento entre `app/` e `vscode-extension/`**: são dois projetos separados no mesmo
  repo (organizado por assunto, não por monólito) — nenhum importa código do outro. Se escalar o
  suficiente, viram repos separados.
- **Descoberta de nós (SPEC-0)**: funções viram nó marcando com `@graphviewer.node(...)`, achadas
  via introspecção real (`inspect`, importando o módulo) — não parsing de AST, mesmo princípio do
  ponto acima aplicado ao código do usuário, não só ao grafo.
- **Código é rei**: o canvas de autoria (SPEC-0) gera Python real e nunca é fonte da verdade
  sozinho — sempre precisa dar pra ler/editar o `.py` gerado fora do canvas. Ver
  [`specs/SPEC-0-node-canvas.md`](specs/SPEC-0-node-canvas.md) pras decisões completas.
- **Servidor MCP** (`vscode-extension/mcp-server/`): é só mais um cliente da API REST/SSE do
  `langgraph dev`, igual o webview e o app Rust já são — não inventa transporte novo, não duplica o
  motor de grafo. Ver [`specs/SPEC-1-mcp-server.md`](specs/SPEC-1-mcp-server.md).

Se você acha que uma dessas decisões tá errada, abre uma issue explicando o caso antes de mandar o
PR — economiza retrabalho dos dois lados.
