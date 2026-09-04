# SPEC-0 — Autoria visual de grafos (canvas livre, não catálogo fixo)

## Contexto

O graphviewer-gp começou como um viewer de execução (mostrar um grafo LangGraph já existente
rodando). Esta spec cobre uma direção nova, maior: deixar de só **visualizar** um grafo pronto e
passar a **construir** o grafo visualmente — não com um catálogo fixo de nós pré-feitos (like n8n),
mas descobrindo automaticamente funções que você mesmo escreveu no seu código e deixando arrastar
elas pro canvas.

Motivação de fundo (por que isso existe): Gabriel quer organizar os "loopings" do ATLAS e do PI —
hoje mantidos de cabeça — e fazer isso de um jeito extremamente visual, sem fricção pra transformar
código em fluxo.

## Princípio central

**Código é rei.** O canvas nunca é a fonte da verdade sozinho — ele gera Python real
(`StateGraph`/`add_node`/`add_edge` do LangGraph), legível e editável fora do canvas. Isso está
alinhado com o princípio já usado no `pi-specs/SPEC-0` do hermes-autonomo (fluxo determinístico,
nada de caixa-preta): o canvas é uma camada de autoria sobre código real, não um runtime paralelo
que interpreta um desenho.

Consequência prática: o canvas precisa **reler** um arquivo de grafo já gerado ao reabrir (round-trip
código → posições visuais), não só escrever uma vez e esquecer.

## Escopo do v1

- **Só Python.** Cobre o que já existe (ATLAS, PI, langgraph-client) sem esforço de parsing
  multi-linguagem. Poliglota (TS, Rust, etc.) fica pra depois de provar que o mecanismo funciona.
- **Vive na extensão VS Code**, não no app nativo Rust (`app/crates/ui`). Motivo: essa feature depende de
  ler/escrever arquivos no workspace aberto e mostrar um editor de texto lado a lado com o canvas —
  é literalmente o forte do VS Code, e o app Rust não tem painel de editor. O app nativo Rust
  continua existindo só como viewer de execução (escopo antigo, sem mudança aqui).

## Mecanismo: função marcada → nó descoberto

1. **Marcador**: um decorator `@graphviewer.node("nome_do_no")` (biblioteca Python leve, sem
   dependências pesadas) em cima de qualquer função do projeto.
2. **Descoberta**: a extensão importa os módulos Python do projeto dentro do `.venv` já configurado
   (mesma convenção usada hoje pra rodar `langgraph dev`) e usa `inspect` pra achar as funções
   decoradas, extraindo nome, assinatura (parâmetros/tipos, quando anotados) e docstring.
   - Decisão: introspecção via import real, não parsing estático de AST. É mais simples de
     implementar corretamente e mais preciso (pega assinatura/tipo de verdade), ao custo de rodar
     código do módulo na hora de escanear — risco aceitável porque é o mesmo código que você já vai
     rodar de qualquer jeito, na sua própria máquina.
3. **Paleta**: as funções descobertas aparecem como uma lista de "tipos de nó" disponíveis no
   painel do canvas (separado da área de canvas em si) — arrasta da paleta pro canvas pra
   instanciar.

## O que uma aresta significa (importante — não é como n8n)

LangGraph não é um grafo de dataflow ponto-a-ponto (saída de A pluga na entrada de B, tipado, como
n8n). É uma máquina de estados: todo nó lê o mesmo `State` compartilhado e devolve uma atualização
parcial dele. **Uma aresta no canvas representa só ordem de execução** (e, quando for aresta
condicional, a função de condição que decide o próximo nó) — não mapeamento de dado campo-a-campo.

Isso simplifica o canvas: não precisamos de UI de "pino de saída → pino de entrada" tipada. O que o
usuário desenha é sequência/ramificação; os campos do `State` que cada função lê/escreve vêm da
assinatura da própria função (marcada no passo de descoberta).

## Geração de código

- O canvas emite um arquivo Python (`graph.py` ou nome configurável) com `StateGraph`, um
  `add_node` por função descoberta usada, e `add_edge`/`add_conditional_edges` conforme desenhado.
- Ao reabrir um projeto, a extensão reparsa esse arquivo gerado pra reconstruir: quais nós existem,
  como estão conectados, e (se guardarmos como comentário/metadado no arquivo) a posição de cada
  nó no canvas — sem isso, o layout visual se perde a cada reabertura mesmo com o grafo intacto.

## Decisões de design (resolvidas)

1. **Metadado de posição — arquivo separado, sincronização forçada.** Posição X/Y de cada nó vive
   em `.graphviewer/layout.json`, nunca como comentário dentro do `graph.py` gerado. Pra manter os
   dois sempre em sincronia: todo nó tem um **ID estável** (não o nome da função — sobrevive a
   rename). A cada save do canvas, reconcilia os dois arquivos juntos: nó que sumiu do `.py` →
   remove do `layout.json`; nó novo sem posição salva → recebe posição default. Nunca existe um
   sem o outro.

2. **Edição manual do `.py` gerado — merge de 3 vias, como o git.** Guardamos a última versão que o
   canvas gerou (a "base"). No próximo save, compara: base → o que existe agora no disco (edição
   manual do Gabriel) vs. base → o que o canvas quer escrever agora (edição via UI). Aplica os dois
   onde não colidem; sinaliza conflito em vez de sobrescrever silenciosamente quando colidem (ex:
   Gabriel apagou um nó na mão que o canvas ainda acha que existe).

3. **Um painel só, com toggle Editar/Observar.** `media/viewer.html` vira o mesmo canvas nos dois
   modos — o que muda é só o que colore as caixas: paleta estática de nós disponíveis (Editar) vs.
   status de execução ao vivo (Observar), estilo Figma Design/Prototype. Não são painéis separados.

## Referências de mercado (inspiração, não decisão tomada)

Notas de pesquisa sobre como outras ferramentas resolvem partes do mesmo problema (código ↔ nó
visual). Registradas aqui como possível inspiração pra decisões futuras desta spec — nenhuma foi
adotada ainda.

### Windmill — assinatura de função → campos de UI automáticos

Windmill lê a assinatura de uma função (ex: `async function dispararEmail(to: string, texto:
string)`) via parsing de tipos (Zod/JSON Schema) e gera dinamicamente os campos de input no
frontend a partir dela. Relevante aqui porque é essencialmente o mesmo mecanismo já decidido no
["Mecanismo: função marcada → nó descoberto"](#mecanismo-função-marcada--nó-descoberto) (introspecção via
`inspect`, extrai assinatura/tipos) — vale olhar a implementação deles como referência de UX pra
como desenhar os campos da paleta a partir dos parâmetros descobertos.

### Inngest — passos sequenciais (`step.run`) com estado resiliente

Padrão de escrever uma função linear onde o motor consegue pausar/retomar sem perder estado:

```typescript
export default async function meuFluxo({ step }) {
  const usuario = await step.run('buscar-banco', () => db.users.find());
  await step.sleep('esperar-3-dias', '3d');
  await step.run('enviar-notificacao', () => enviarEmail(usuario.email));
}
```

Cada `step.run` é registrado numa tabela (`step_runs` ou equivalente) — se o processo reiniciar
durante um `step.sleep`, o estado do passo anterior já está salvo e não precisa re-executar. Só é
relevante aqui se o graphviewer-gp algum dia cobrir grafos de execução longa (LangGraph já tem
checkpointing próprio, então isso seria mais um paralelo conceitual que algo pra copiar direto).

### Temporal — idempotência e retentativa automática por nó

Ideia central (sem clonar a arquitetura de Event Sourcing do Temporal, complexa demais pra este
projeto): tratar cada nó como uma transação isolada. Se um nó falhar (ex: "enviar webhook" com o
sistema alvo fora do ar), o motor tenta de novo automaticamente (N tentativas, ex: 5) antes de
marcar o nó como falho no canvas — via fila (BullMQ ou polling simples em Postgres/SQLite).
Relacionado a uma necessidade futura de visualizar estado de falha/retry por nó no modo Observar
(ver decisão 3, toggle Editar/Observar), ainda não coberta por esta spec.

## Fora de escopo nesta spec

- Poliglota (TS/Rust/etc. como linguagem de nó).
- MCP (expor os nós/grafo como tools pra outros agentes) — ideia relacionada, registrada em
  `vscode-extension/STATUS.md`, mas é uma spec própria depois que esta aqui estiver rodando.
- Qualquer mudança no app nativo Rust (`app/crates/ui`).
