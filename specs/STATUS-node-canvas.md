ARQUIVO TEMPORÁRIO — apagar ao concluir o roadmap (perguntar ao Gabriel antes).

# Status: implementação da SPEC-0 (autoria visual de grafos)

Spec completa em `specs/SPEC-0-node-canvas.md`. Este arquivo é só o checklist de execução.

## Roadmap

- [x] **Pacote `graphviwer` (Python)** — `python/graphviwer/`
  - [x] `graphviwer.node(name=None)` decorator — anexa metadado, não altera comportamento
  - [x] `graphviwer.scanner.discover(root_dir)` — importa `.py` do projeto, acha funções
        marcadas via `inspect`, extrai nome/assinatura/docstring/linha
  - [x] Testado com `python/graphviwer/examples/example_graph.py` (3 nós descobertos, função
        não-marcada corretamente ignorada)
- [x] **CLI/subprocess da extensão chamando o scanner** — `extension.ts`: `resolvePythonBin` +
      `discoverNodes` (spawna `python -m graphviwer.scanner <root>`, parseia o JSON, falha
      silenciosa se o pacote não estiver instalado). Chamado dentro de `openGraph`.
      Testado de ponta a ponta contra `~/dev/lg-demo` (instalei `graphviwer` no `.venv` dele,
      marquei as 3 funções do `graph.py`, scanner achou todas certas).
- [x] **Lista dos nós descobertos no webview** — `viewer.html`: seção `#palette`, mensagem
      `setDiscoveredNodes`, renderiza nome/assinatura/docstring. **Ainda não é arrastável** — é
      só lista por enquanto; drag-and-drop pro canvas fica pro próximo passo.
- [ ] **Modo Editar vs Observar** — toggle no `viewer.html` (decisão 3 da spec)
- [ ] **Codegen** — canvas → `graph.py` (`StateGraph`/`add_node`/`add_edge`)
- [ ] **`.graphviwer/layout.json`** — posição X/Y por ID estável de nó, sincronizado com o `.py`
- [ ] **Merge de 3 vias** — base (última geração) vs. disco (edição manual) vs. canvas novo

## Observações

- Scanner importa módulos de verdade (não AST) — decisão registrada na spec, risco aceito.
- `graphviwer` ainda não instalado em nenhum projeto real (só no `.venv` de teste em
  `python/graphviwer/.venv`) — falta decidir como ele chega no `.venv` de um projeto qualquer que
  o usuário abrir (pip install manual por enquanto, como o `.venv` do `langgraph dev` já exige).
