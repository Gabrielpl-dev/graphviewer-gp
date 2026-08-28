# Contribuindo

Projeto pequeno, sem processo burocrático. Regras práticas:

## Sugestões e bugs

Abre uma issue. Pra sugestão de melhoria, descreve o problema que você tá tentando resolver, não
só a solução que você imaginou — às vezes tem um jeito mais simples de chegar lá.

## Pull requests

- Um PR, uma mudança lógica — evita misturar refatoração com feature nova.
- Se mexer no lado Rust (`app/`): `cargo check` (ou `cargo build`) tem que passar sem erro antes
  do PR.
- Se mexer na extensão (`vscode-extension/`): `npm run compile` tem que passar sem erro.
- Se mexer no pacote Python (`python/graphviewer/`): roda o scanner contra `examples/` e confirma
  que ainda acha as funções marcadas (`python -m graphviewer.scanner examples`).
- Explica o *porquê* da mudança na descrição do PR, não só o que mudou — o diff já mostra o que
  mudou.

## Decisões de arquitetura que já foram tomadas (não reabrir sem motivo forte)

- **Sem parsing estático de código** pra descobrir o grafo — sempre via API do `langgraph dev`
  rodando de verdade. É a decisão central do projeto (ver README).
- **Extensão VS Code**: o webview fala com o `langgraph dev` local diretamente (fetch/SSE) — a
  extensão só gerencia o processo do servidor e a URL, não faz proxy dos dados.
- **App Rust**: arquitetura em camadas (crates separados) pra mudar UI sem recompilar o cliente
  HTTP. Ver `app/crates/ui`, `app/crates/graph-model`, `app/crates/langgraph-client` conforme forem
  existindo.
- **Sem acoplamento entre `app/` e `vscode-extension/`**: são dois projetos separados no mesmo
  repo (organizado por assunto, não por monólito) — nenhum importa código do outro. Se escalar o
  suficiente, viram repos separados.
- **Descoberta de nós (SPEC-0)**: funções viram nó marcando com `@graphviewer.node(...)`, achadas
  via introspecção real (`inspect`, importando o módulo) — não parsing de AST, mesmo princípio do
  ponto acima aplicado ao código do usuário, não só ao grafo.
- **Código é rei**: o canvas de autoria (SPEC-0) gera Python real e nunca é fonte da verdade
  sozinho — sempre precisa dar pra ler/editar o `.py` gerado fora do canvas. Ver
  [`specs/SPEC-0-node-canvas.md`](specs/SPEC-0-node-canvas.md) pras decisões completas.

Se você acha que uma dessas decisões tá errada, abre uma issue explicando o caso antes de mandar o
PR — economiza retrabalho dos dois lados.
