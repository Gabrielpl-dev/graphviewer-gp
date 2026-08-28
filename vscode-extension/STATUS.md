ARQUIVO TEMPORÁRIO — apagar ao concluir o roadmap (perguntar ao Gabriel antes).

# Status: extensão VS Code do graphviewer-gp

Plano completo em `~/.claude/plans/bora-planejar-dapper-lamport.md`. Este arquivo é só o
checklist de execução + observações que não quero redescobrir se a sessão for resumida.

## Roadmap (v1 — concluído)

- [x] `package.json`: `contributes.viewsContainers` (ícone na Activity Bar) + `contributes.views`
      (o `WebviewView`) + `contributes.menus.explorer/context` (botão direito em `langgraph.json`)
- [x] `media/viewer.html`: adaptar `~/dev/lg-demo/viewer.html` — CSP com nonce, `connect-src
      http://127.0.0.1:*`, layout vertical (grafo em cima, log embaixo), baseUrl vem de
      `postMessage` da extensão em vez de input manual
- [x] `src/extension.ts`: `WebviewViewProvider`, comando `graphviewerGp.openGraph(uri)` que lê
      `langgraph.json`, spawna `langgraph dev` do `.venv` do projeto, faz polling em `/ok`,
      manda a URL pro webview
- [x] `npm install` em `vscode-extension/`
- [x] `npm run compile`
- [x] Testar via Extension Development Host (`code --extensionDevelopmentPath=...`), usando
      `~/dev/lg-demo` como alvo — confirmado por Gabriel: ícone aparece, menu de contexto funciona,
      grafo carrega. Painel movido manualmente pra secondary sidebar (não dá via API, ver
      `~/.claude/plans/bora-planejar-dapper-lamport.md`).

## Próximos passos (pesquisa de concorrentes em 2026-08-27 — VizLang Studio, LangGraph
Visualizers/smazee, langgraph-visualizer/Naveenkumarar)

Ideias vale a pena copiar (conceito, não código — os três são parsing estático, a nossa não):

- [ ] **Time Capsule / replay de checkpoint** — LangGraph já persiste checkpoint a cada passo;
      dá pra voltar no histórico de uma execução e inspecionar estado anterior. Já temos acesso via
      API (thread/checkpoints), só falta expor na UI.
- [ ] **Painel de schema do state** — campo/tipo/default do state do grafo. Puxar de
      `GET /assistants/{id}/schemas` (visto no OpenAPI, nunca usado).
- [ ] **Busca/filtro de nó** no canvas (destaca nó pelo nome).
- [ ] **Minimap** pra grafos grandes.

Fora de escopo, não vale copiar: "jump to code" (clique no nó abre a definição no arquivo) — exige
parsing estático, que a gente evita de propósito (quebra com grafo dinâmico/subgrafo).

## Ideias maiores levantadas em 2026-08-27 (Gabriel, sem spec ainda)

- [x] **MCP server** — spec própria escrita: `specs/SPEC-1-mcp-server.md` (escopado pra outro
      agente trabalhar em paralelo sem colidir com esta frente). Segue aqui só o contexto original:
      expor grafo/execução/schema do `graphviewer-gp` como servidor MCP, pra
      qualquer agente (Claude Code, ATLAS) consultar programaticamente, não só humano olhando
      webview. Direção confirmada (é o padrão que o Gabriel já usa — `vault-query-mcp`), mas ainda
      não desenhado: que tools expor, roda junto com a extensão ou separado.
      **Motivo reforçado em 2026-08-28**: o público-alvo da ferramenta é gente que usa AI — faz
      sentido a própria AI conseguir dirigir a ferramenta, não só o humano clicando. Caso concreto
      vivido nesse dia: um bug real na paleta (webview não resolvido a tempo, ver
      `specs/STATUS-node-canvas.md`) só foi achado por leitura de código às cegas, porque não existe
      nenhuma camada programática pra abrir grafo/rodar/inspecionar sem clicar na UI do VS Code —
      nem pro Gabriel testar rápido, nem pra mim (Claude) testar sozinho.
- [ ] **Portabilidade e manipulação de grafos/subgrafos** — três coisas juntas, ainda sem spec
      (Gabriel pediu explicitamente pra planejar direito antes de construir):
      1. exportar/compartilhar a estrutura do grafo (json/mermaid) sem precisar do `langgraph dev`
         rodando pra visualizar de novo;
      2. abrir grafo de qualquer repo com menos rigidez na convenção atual (hoje exige
         `langgraph.json` + `.venv` no mesmo formato exato de `lg-demo`);
      3. manipular o grafo de forma mais simples — não só visualizar, editar (escopo real ainda
         indefinido).
      **Não começar a construir isso sem sessão de planejamento própria primeiro.**

## Observações

- Subgrafos: NÃO fazer parsing estático. Usar `GET /assistants/{id}/subgraphs` do langgraph-api
  (visto no OpenAPI, nunca testado na prática — validar quando chegar nessa parte).
- Porta fixa 2024 no v1. Se já tiver um `langgraph dev` respondendo em `/ok`, reusa em vez de
  subir outro processo.
- `.venv` do projeto clicado precisa já existir (mesma convenção manual usada em `lg-demo`) —
  sem auto-detecção de ambiente Python nesta versão.
