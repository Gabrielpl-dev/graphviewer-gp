# SPEC-1 — Servidor MCP (graphviewer-gp pra agentes, não só humano)

## Contexto

Hoje o `graphviewer-gp` só serve humano olhando um webview. Motivação: o público-alvo da
ferramenta é gente que usa AI — faz sentido a própria AI conseguir abrir/rodar/inspecionar um
grafo LangGraph programaticamente, sem depender de clique em UI. Caso concreto que motivou isso:
um bug real na paleta da extensão (`specs/STATUS-node-canvas.md`) só foi achado por leitura de
código às cegas — não existia nenhuma camada programática pra testar sem clicar na UI do VS Code.

**Esta spec cobre só o servidor MCP.** Não mexe em extensão VS Code, app Rust, ou no pacote
`graphviewer` além de importar dele (ver limites abaixo).

## O que construir

Um servidor MCP que expõe, como tools, o que o `langgraph dev` (a API REST/SSE local que já roda
100% aberta, MIT+Elastic-2.0 — ver `README.md`) já sabe fazer. O servidor MCP é **um cliente a
mais** dessa API, igual o webview e o app Rust já são — não inventa transporte novo, não duplica o
motor de grafo.

Tools sugeridas (ponto de partida, não travado — refine dentro do escopo se fizer sentido, mas
documente a decisão na spec ou num `STATUS-mcp-server.md` próprio):

- `list_discovered_nodes(project_dir)` — reusa `graphviewer.scanner.discover()` (já existe, não
  reimplementa).
- `get_graph_structure(project_dir)` — sobe/reusa um `langgraph dev` daquele projeto (mesma lógica
  de `ensureGraphServerRunning` do `extension.ts`, reimplementada em Python — duplicar essa
  lógica pequena é aceitável, não vale a complexidade de compartilhar código entre TS e Python
  agora) e devolve `GET /assistants/{id}/graph`.
- `run_graph(project_dir, input)` — roda via `POST /runs/wait`, devolve o estado final.
- `stream_run_events(project_dir, input)` — roda via `POST /runs/stream`, devolve a lista de
  eventos coletados (início/fim de cada nó) — MCP tools são request/resposta, não tem por que virar
  streaming de verdade nessa v1.

## Stack

Python, no mesmo espírito do pacote `graphviewer` (SDK oficial `mcp` da Anthropic). Fica num
projeto novo, não dentro de `python/graphviewer/`.

## Onde vive (importante — isso é o que evita colisão)

Decisão do Gabriel: fica dentro de `vscode-extension/`, não solto na raiz — mesmo assim, **sem
acoplamento** com o resto da extensão (não importa `extension.ts`, não depende do webview rodando).

```
vscode-extension/
└── mcp-server/        ← criar do zero, tudo daqui pra baixo é território livre do agente
    ├── pyproject.toml
    └── graphviewer_mcp/
        └── ...
```

## O que o agente PODE mexer

- Criar e editar qualquer coisa dentro de `vscode-extension/mcp-server/` (novo, ainda não existe).
- **Ler** (import, nunca editar) `python/graphviewer/` — é uma dependência (`pip install -e
  ../../python/graphviewer` ou publicada), não território dele.
- Adicionar uma seção nova em `README.md` descrevendo o `mcp-server/` (só *adicionar*, não reescrever
  seções existentes).
- Criar `vscode-extension/mcp-server/STATUS.md` ou `specs/STATUS-mcp-server.md` próprio pra
  acompanhar o progresso dele (arquivo temporário, mesma convenção dos outros `STATUS-*.md` do repo).

## O que o agente NÃO PODE mexer (outro agente trabalhando nisso agora)

- Qualquer coisa em `vscode-extension/` **fora** de `mcp-server/` (`src/extension.ts`,
  `media/viewer.html`, `package.json`, etc.) — trabalho em andamento na SPEC-0, mexer aqui gera
  conflito de merge na certa. `mcp-server/` é uma ilha dentro de `vscode-extension/`, o resto não é
  seu território.
- `python/graphviewer/` (o pacote em si, não o import) — mesmo motivo.
- `app/` e `src/` na raiz (app Rust) — fora de escopo total, nem toca.
- `specs/SPEC-0-node-canvas.md` e `specs/STATUS-node-canvas.md` — outro trabalho em andamento.
- `.gitignore`, `CONTRIBUTING.md`, `CLAUDE.md` na raiz — se achar que precisa mudar algo aqui,
  **para e pergunta ao Gabriel antes**, não decide sozinho.

## Se bater numa dessas situações, para e pergunta (não decide sozinho)

- Achar que precisa mudar algo fora de `mcp-server/` pra fazer sentido.
- Achar que a lista de tools sugerida acima tá errada de um jeito que muda a forma como as outras
  frentes (extensão, app Rust) deveriam funcionar.
- Qualquer dúvida sobre licenciamento/dependências novas (mesmo princípio do resto do projeto:
  Elastic-2.0 do `langgraph-api` não impede uso local, mas não reproduzir ela como serviço
  hospedado de terceiro).

## Verificação

- `mcp-server/` instala e roda isolado (`pip install -e vscode-extension/mcp-server`).
- Testado contra `~/dev/lg-demo` (mesmo projeto de teste usado no resto do repo): as tools
  conseguem listar nós descobertos, pegar estrutura do grafo, e rodar uma execução — sem precisar
  abrir VS Code nem clicar em nada.
