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
- [x] **Modo Editar vs Observar** — toggle no `viewer.html` (decisão 3 da spec). Editar: arrasta
      função da paleta pro canvas (posição vira nó), clica em dois nós pra criar aresta (ordem, não
      pino de dado — como a spec define). Estado de autoria (`authoredNodes`/`authoredEdges`)
      sobrevive trocar de aba. **Ainda não testado manualmente no VS Code de verdade** — só
      validado sintaxe (`node --check`) e revisão de código, falta clicar de verdade.
- [~] **Codegen** — botão "ver código gerado" já produz o Python (`StateGraph`/`add_node`/
      `add_edge`/`add_conditional_edges`... ainda sem condicional) a partir do grafo desenhado, mas
      **só mostra em preview** — não escreve `graph.py` no disco, não sabe o import real da função
      (assume que já tá no mesmo arquivo), e o `State` fica com `...` placeholder. Falta: escrever
      de fato (via extensão, que tem acesso a filesystem) e inferir/mapear os imports certos.
- [ ] **`.graphviewer/layout.json`** — posição X/Y por ID estável de nó, sincronizado com o `.py`
      (hoje a posição só existe em memória do webview, se perde ao fechar o painel)
- [ ] **Merge de 3 vias** — base (última geração) vs. disco (edição manual) vs. canvas novo

## Bug real encontrado e corrigido (2026-08-28)

Paleta não aparecia mesmo com `graphviewer` instalado. Causa: `provider.reveal()` chamava
`this.view?.show?.(true)` — mas `this.view` só existe depois que o VS Code chama
`resolveWebviewView`, que só acontece quando o painel já foi mostrado pelo menos uma vez na sessão.
Na primeira abertura, `setBaseUrl`/`setDiscoveredNodes` mandavam a mensagem pro vazio (webview nem
existia ainda). Corrigido: `ensureResolved()` usa o comando auto-gerado
`graphviewerGp.graphView.focus` (força o VS Code a resolver o webview) antes de mandar qualquer
mensagem. Também adicionado `retainContextWhenHidden: true` — sem isso, o VS Code descartava o
conteúdo do painel toda vez que ele ficava escondido (perderia o estado de autoria do canvas ao
trocar de aba). **Ainda não confirmado pelo Gabriel que resolveu de verdade** — próximo teste.

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
