# graphviewer-gp

Viewer ao vivo de execuções [LangGraph](https://github.com/langchain-ai/langgraph) — mostra o
grafo (nós, arestas, subgrafos) e acompanha a execução nó a nó em tempo real, sem depender de
conta na LangSmith.

## Por quê

O [LangGraph Studio](https://github.com/langchain-ai/langgraph-studio) hospedado exige login
mesmo pra rodar uma execução local. `graphviewer-gp` fala direto com a API REST/SSE do `langgraph
dev` (que já roda 100% local, MIT) — sem passar por conta nenhuma.

Diferença de outras ferramentas parecidas (VizLang Studio, LangGraph Visualizers, etc.): elas
descobrem o grafo fazendo *parsing estático* do código Python, o que quebra quando o grafo é
montado dinamicamente ou tem subgrafos. `graphviewer-gp` nunca lê o código-fonte — ele pergunta pro
servidor `langgraph dev` de verdade (`/assistants/{id}/graph`, `/assistants/{id}/subgraphs`), então
o que você vê é garantidamente o que vai executar.

## O que tem aqui

O repo é organizado por **assunto** (visualização de grafos LangGraph), não por um monólito só —
`app/` e `vscode-extension/` são dois projetos separados, sem acoplamento nenhum entre si (nenhum
dos dois importa código do outro). Se um dia escalar o suficiente pra justificar, cada um vira
repo próprio; por enquanto convivem no mesmo lugar porque tratam do mesmo problema.

- **`vscode-extension/`** — extensão VS Code (TypeScript). Clica com botão direito num
  `langgraph.json` no Explorer → sobe o `langgraph dev` daquele projeto → abre um painel lateral
  com o grafo e o log de eventos ao vivo. Já mostra também uma lista dos nós descobertos via
  `@graphviewer.node` (ver abaixo), ainda não arrastável. **Funcional (v1 do viewer).**
- **`python/graphviewer/`** — pacote Python com o decorator `@graphviewer.node("nome")`, que marca
  qualquer função como um nó descobrível, e um scanner que importa o projeto de verdade (via
  `inspect`, não parsing de AST) e acha essas funções. Usado pela extensão VS Code (leitura, sem
  acoplamento reverso).
- **`app/`** — app nativo em Rust (egui/eframe), pra quem quer um executável separado em vez de
  depender do VS Code. Motivo de existir também: falso positivo de antivírus corporativo com
  binário Rust não assinado atrapalhou a extensão em algum momento, então a versão nativa é a
  alternativa que não depende do VS Code estar aberto. Arquitetura em camadas: `graph-model`
  (tipos compartilhados), `langgraph-client` (HTTP/SSE) e `ui` (egui — canvas com pan/zoom, log de
  eventos). **Funcional** — carrega grafo, roda execução, acompanha nó a nó ao vivo.
- **`vscode-extension/mcp-server/`** — servidor MCP: expõe grafo/execução/schema como tools
  (`list_discovered_nodes`, `get_graph_structure`, `run_graph`, `stream_run_events`), pra qualquer
  agente (Claude Code, ATLAS) consultar programaticamente em vez de depender de um humano olhando o
  webview. Só mais um cliente da API REST/SSE do `langgraph dev` — não inventa transporte novo.
  **Funcional** — testado ponta a ponta contra um projeto real, sem abrir VS Code. Ver
  [`specs/SPEC-1-mcp-server.md`](specs/SPEC-1-mcp-server.md).

## Pra onde isso vai (SPEC-0)

O plano deixou de ser só "visualizar um grafo pronto" e virou "**construir** o grafo visualmente" —
não com um catálogo fixo de nós tipo n8n, mas descobrindo automaticamente funções marcadas no seu
próprio código (`@graphviewer.node`) e deixando arrastar elas num canvas, sem escrever
`add_node`/`add_edge` na mão. O canvas gera Python real (nunca é fonte da verdade sozinho — código
é rei) e sabe reler um grafo já gerado ao reabrir. Detalhes e decisões de design completos em
[`specs/SPEC-0-node-canvas.md`](specs/SPEC-0-node-canvas.md); progresso em
[`specs/STATUS-node-canvas.md`](specs/STATUS-node-canvas.md).

## Rodando a extensão VS Code

```
cd vscode-extension
npm install
npm run compile
```

Depois, no VS Code: `Run and Debug` → `Extension Development Host` (ou `F5`), abre uma pasta com
um `langgraph.json` válido, clica com botão direito nele → "graphviewer-gp: Abrir grafo".

Pré-requisito: o projeto do `langgraph.json` precisa ter um `.venv` com `langgraph-cli[inmem]`
instalado (mesma convenção de qualquer projeto LangGraph local).

Opcional: instale `python/graphviewer` (`pip install -e python/graphviewer`) no `.venv` do projeto
alvo e marque funções com `@graphviewer.node("nome")` — elas aparecem listadas no painel.

## Rodando o app nativo (Rust)

```
cd app
cargo run -p ui
```

Se o antivírus da sua máquina apagar o `.exe` compilado (falso positivo comum com Rust/egui não
assinado), rode sempre via `cargo run` em vez de depender do binário ficar no disco — não é
comportamento esperado do projeto, é uma limitação de ambiente.

## Contribuindo

Sugestões, bugs e PRs são bem-vindos — ver [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Licença

[MIT](LICENSE)
