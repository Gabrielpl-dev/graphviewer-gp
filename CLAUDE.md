# graphviewer-gp

Ferramentas pra acompanhar (e, cada vez mais, construir) execução de grafos LangGraph ao vivo, sem
depender de conta na LangSmith. Repo organizado por **assunto**, não por monólito: `app/` (Rust) e
`vscode-extension/` (TypeScript) são projetos separados, sem acoplamento entre si — ver
`CONTRIBUTING.md` e `README.md` pra visão geral de cada frente, e `specs/` pras specs de design
(`SPEC-0`: autoria visual de grafos; `SPEC-1`: servidor MCP).

## Antivírus/EDR bloqueia o .exe compilado (app/, Rust)

O `app/target/release/ui.exe` (e também builds debug) é apagado/bloqueado pelo antivírus
corporativo minutos depois de compilado — falso positivo comum em binários Rust/egui não assinados
(usa `accesskit`/`clipboard-win`/criação de janela nativa, heurística lê como malware). Sem solução
permanente ainda (depende de TI liberar a pasta ou confiar num certificado — ver conversa de
2026-08-27).

**Enquanto isso não for resolvido: sempre rode o app via terminal (`cd app && cargo run -p ui` ou
`cargo run --release -p ui`), nunca dependa do `.exe` compilado ficar disponível pra clique.**
Rodar direto do `cargo run` evita o problema porque o binário some do disco só depois de já ter
sido lançado como processo.
