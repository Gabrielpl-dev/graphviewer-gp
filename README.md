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

Três frentes:

- **`vscode-extension/`** — extensão VS Code. Clica com botão direito num `langgraph.json` no
  Explorer → sobe o `langgraph dev` daquele projeto → abre um painel lateral com o grafo e o log de
  eventos ao vivo. Já mostra também uma lista dos nós descobertos via `@graphviewer.node` (ver
  abaixo), ainda não arrastável. **Funcional (v1 do viewer).**
- **`python/graphviewer/`** — pacote Python com o decorator `@graphviewer.node("nome")`, que marca
  qualquer função como um nó descobrível, e um scanner que importa o projeto de verdade (via
  `inspect`, não parsing de AST) e acha essas funções. É a base do próximo passo: autoria visual.
- **`crates/`** — app nativo em Rust (egui/eframe), pra quem quer um executável separado em vez de
  depender do VS Code. Motivo de existir também: falso positivo de antivírus corporativo com
  binário Rust não assinado atrapalhou a extensão em algum momento, então a versão nativa é a
  alternativa que não depende do VS Code estar aberto. **Ainda no início** — arquitetura em
  camadas (`graph-model` / `langgraph-client` / `ui`) desenhada, só a janela vazia (`crates/ui`)
  está pronta.

## Pra onde isso vai (SPEC-0)

O plano deixou de ser só "visualizar um grafo pronto" e virou "**construir** o grafo visualmente" —
não com um catálogo fixo de nós tipo n8n, mas descobrindo automaticamente funções marcadas no seu
próprio código (`@graphviewer.node`) e deixando arrastar elas num canvas, sem escrever
`add_node`/`add_edge` na mão. O canvas gera Python real (nunca é fonte da verdade sozinho — código
é rei) e sabe reler um grafo já gerado ao reabrir. Detalhes e decisões de design completos em
[`specs/SPEC-0-node-canvas.md`](specs/SPEC-0-node-canvas.md); progresso em
[`specs/STATUS-node-canvas.md`](specs/STATUS-node-canvas.md).

Ideia relacionada, ainda sem spec própria: expor grafo/execução/schema como **servidor MCP**, pra
qualquer agente (Claude Code, ATLAS) consultar programaticamente em vez de depender de um humano
olhando o webview — registrado em [`vscode-extension/STATUS.md`](vscode-extension/STATUS.md).

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
cargo run -p ui
```

Se o antivírus da sua máquina apagar o `.exe` compilado (falso positivo comum com Rust/egui não
assinado), rode sempre via `cargo run` em vez de depender do binário ficar no disco — não é
comportamento esperado do projeto, é uma limitação de ambiente.

## Contribuindo

Sugestões, bugs e PRs são bem-vindos — ver [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Licença

[MIT](LICENSE)
