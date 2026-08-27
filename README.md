# graphviwer-gp

Viewer ao vivo de execuções [LangGraph](https://github.com/langchain-ai/langgraph) — mostra o
grafo (nós, arestas, subgrafos) e acompanha a execução nó a nó em tempo real, sem depender de
conta na LangSmith.

## Por quê

O [LangGraph Studio](https://github.com/langchain-ai/langgraph-studio) hospedado exige login
mesmo pra rodar uma execução local. `graphviwer-gp` fala direto com a API REST/SSE do `langgraph
dev` (que já roda 100% local, MIT) — sem passar por conta nenhuma.

Diferença de outras ferramentas parecidas (VizLang Studio, LangGraph Visualizers, etc.): elas
descobrem o grafo fazendo *parsing estático* do código Python, o que quebra quando o grafo é
montado dinamicamente ou tem subgrafos. `graphviwer-gp` nunca lê o código-fonte — ele pergunta pro
servidor `langgraph dev` de verdade (`/assistants/{id}/graph`, `/assistants/{id}/subgraphs`), então
o que você vê é garantidamente o que vai executar.

## O que tem aqui

Duas frentes, mesmo objetivo:

- **`vscode-extension/`** — extensão VS Code. Clica com botão direito num `langgraph.json` no
  Explorer → sobe o `langgraph dev` daquele projeto → abre um painel lateral com o grafo e o log de
  eventos ao vivo. **Funcional (v1).**
- **`crates/`** — app nativo em Rust (egui/eframe), pra quem quer um executável separado em vez de
  depender do VS Code. Motivo de existir também: falso positivo de antivírus corporativo com
  binário Rust não assinado atrapalhou a extensão em algum momento, então a versão nativa é a
  alternativa que não depende do VS Code estar aberto. **Ainda no início** — arquitetura em
  camadas (`graph-model` / `langgraph-client` / `ui`) desenhada, só a janela vazia (`crates/ui`)
  está pronta.

## Rodando a extensão VS Code

```
cd vscode-extension
npm install
npm run compile
```

Depois, no VS Code: `Run and Debug` → `Extension Development Host` (ou `F5`), abre uma pasta com
um `langgraph.json` válido, clica com botão direito nele → "graphviwer-gp: Abrir grafo".

Pré-requisito: o projeto do `langgraph.json` precisa ter um `.venv` com `langgraph-cli[inmem]`
instalado (mesma convenção de qualquer projeto LangGraph local).

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
