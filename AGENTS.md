# AGENTS.md

Tutti gli agenti devono seguire le best practice idiomatiche di Rust:

- Rust idiomatico: `map`/`and_then` su `Option`/`Result`, `let ... else` quando più chiaro, match esaustivo, niente `unsafe`.
- Niente `unwrap()`/`expect()` in percorsi raggiungibili: propaga gli errori con `?`.
- Prima di committare: `cargo fmt`, `cargo clippy --all-targets -- -D warnings` e `cargo test` puliti.

Seguire anche le best practice di clean code:

- Funzioni corte con una sola responsabilità.
- Nomi descrittivi che dicono l'azione (niente "handle"/"do_stuff").
- Niente duplicazione: estrai e riusa.
- Codice leggibile prima di tutto: niente dead code, codice commentato o "per il futuro".
