# graphviewer-gp

Viewer ao vivo de execuções [LangGraph](https://github.com/langchain-ai/langgraph), direto no VS Code — sem depender de conta na LangSmith.

Clique com o botão direito num `langgraph.json` no Explorer e o painel lateral abre já mostrando o grafo, com pan/zoom, e acompanha a execução nó a nó em tempo real assim que você roda algo.

## Por quê

O [LangGraph Studio](https://github.com/langchain-ai/langgraph-studio) hospedado exige login mesmo pra rodar uma execução local. Esta extensão fala direto com a API REST/SSE do `langgraph dev` — que já roda 100% na sua máquina — sem passar por conta nenhuma.

Diferente de ferramentas parecidas que fazem *parsing estático* do código pra descobrir o grafo (e quebram com grafos montados dinamicamente ou com subgrafos), o graphviewer-gp nunca lê o código-fonte: ele pergunta pro próprio servidor `langgraph dev` (`/assistants/{id}/graph`), então o que você vê é garantidamente o que vai executar.

## Uso

1. Tenha um projeto LangGraph com `langgraph.json` e um `.venv` local com `langgraph-cli[inmem]` instalado (convenção padrão de qualquer projeto LangGraph rodando local).
2. No Explorer do VS Code, clique com o botão direito no `langgraph.json` → **graphviewer-gp: Abrir grafo**.
3. A extensão sobe o `langgraph dev` daquele projeto, espera ficar pronto e abre o painel com o grafo.
4. Rode uma execução (pela sua aplicação, script, etc.) e acompanhe os nós acendendo em tempo real no painel.

## Requisitos

- Um projeto LangGraph válido (`langgraph.json` + `.venv` com as dependências do LangGraph instaladas).
- Porta `2024` livre (v1 usa porta fixa; se já houver um `langgraph dev` respondendo nela, a extensão reaproveita em vez de subir outro processo).

## Limitações conhecidas (v1)

- Um grafo por vez, porta fixa.
- `.venv` precisa já existir no projeto — sem auto-detecção de ambiente Python.

## Licença

[MIT](https://github.com/Gabrielpl-dev/graphviewer-gp/blob/master/LICENSE)
