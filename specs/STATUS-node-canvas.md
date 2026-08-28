ARQUIVO TEMPORÁRIO — apagar ao concluir o roadmap (perguntar ao Gabriel antes).

# Status: implementação da SPEC-0 (autoria visual de grafos)

Spec completa em `specs/SPEC-0-node-canvas.md`. Este arquivo é só o checklist de execução.

## Roadmap

- [x] **Pacote `graphviewer` (Python)** — `python/graphviewer/`
  - [x] `graphviewer.node(name=None)` decorator — anexa metadado, não altera comportamento
  - [x] `graphviewer.scanner.discover(root_dir)` — importa `.py` do projeto, acha funções
        marcadas via `inspect`, extrai nome/assinatura/docstring/linha
  - [x] Testado com `python/graphviewer/examples/example_graph.py` (3 nós descobertos, função
        não-marcada corretamente ignorada)
- [x] **CLI/subprocess da extensão chamando o scanner** — `extension.ts`: `resolvePythonBin` +
      `discoverNodes` (spawna `python -m graphviewer.scanner <root>`, parseia o JSON, falha
      silenciosa se o pacote não estiver instalado). Chamado dentro de `openGraph`.
      Testado de ponta a ponta contra `~/dev/lg-demo` (instalei `graphviewer` no `.venv` dele,
      marquei as 3 funções do `graph.py`, scanner achou todas certas).
- [x] **Lista dos nós descobertos no webview** — `viewer.html`: seção `#palette`, mensagem
      `setDiscoveredNodes`, renderiza nome/assinatura/docstring. **Ainda não é arrastável** — é
      só lista por enquanto; drag-and-drop pro canvas fica pro próximo passo.
- [ ] **Modo Editar vs Observar** — toggle no `viewer.html` (decisão 3 da spec)
- [ ] **Codegen** — canvas → `graph.py` (`StateGraph`/`add_node`/`add_edge`)
- [ ] **`.graphviewer/layout.json`** — posição X/Y por ID estável de nó, sincronizado com o `.py`
- [ ] **Merge de 3 vias** — base (última geração) vs. disco (edição manual) vs. canvas novo

## Observações

- Scanner importa módulos de verdade (não AST) — decisão registrada na spec, risco aceito.
- `graphviewer` instalado via `pip install -e` em `~/dev/lg-demo` pra teste — ainda manual, sem
  decisão de como ele chega no `.venv` de um projeto qualquer que o usuário abrir (mesma convenção
  manual do `.venv` do `langgraph dev`, por enquanto).
- Rename `graphviwer` → `graphviewer` (faltava um "e") feito em 2026-08-28: pacote Python, IDs de
  comando/view da extensão, repo no GitHub. Pasta local (`~/dev/graphviwer-gp`) ficou com o nome
  antigo de propósito — Gabriel decidiu não vale a pena travar em VS Code aberto só por isso, só
  online (GitHub) precisa estar certo.
- **Pendência de frontend:** `viewer.html` hoje é HTML/CSS/JS cru, herdado direto do protótipo
  `lg-demo`. Gabriel quer revisar isso "com mais calma" — bom momento é quando chegar no modo
  Editar/Observar (vai precisar de mais estrutura de UI de qualquer jeito).
