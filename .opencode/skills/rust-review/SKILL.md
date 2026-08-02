---
name: rust-review
description: "Revisione didattica del codice Rust appena scritto nel progetto phone-book (axum 0.8 + sea-orm 2.0 + SQLite). Verifica correttezza, best practice idiomatiche e manutenibilità SENZA modificare nulla. Produce un report strutturato che spiega ogni rilievo col perché. Usala quando l'utente chiede di controllare il codice, verificare che sia ok, revisionare un file, fare review, chiede se è corretto, se rispetta le best practice o com'è la manutenibilità — anche senza nominare esplicitamente la parola review. È pensata per l'agente rust-master, ma funziona con qualsiasi agente."
---

# Rust Review — code review didattica per phone-book

## Principi

1. **Read-only.** Non scrivere, modificare o formattare codice. Il tuo compito è analizzare e riportare. L'utente applicherà le correzioni da solo.
2. **Didattico.** Ogni rilievo spiega il *perché*, non solo il *cosa*: il problema, la regola violata, e il pattern idiomatico alternativo.
3. **Focalizzato.** Revisiona il codice indicato (o il lavoro recente), non l'intero progetto.
4. **Chirurgico.** Non divagare su codice non revisionato né su concetti estranei ai rilievi.

## Workflow

### 1. Identifica l'oggetto della revisione
- Se l'utente indica file specifici: revisiona quelli.
- Se chiede generico "controlla il codice": esegui `git status` e `git diff` per individuare il lavoro non committato, poi revisiona quei file.

> **Git autonomo.** `git status` e `git diff` (con qualsiasi flag, es. `--stat`, `--cached`, `-- <file>`) sono pre-autorizzati per l'agente: eseguili direttamente, senza chiedere conferma. Sono comandi di sola lettura: mai `git add`, `git commit`, `git reset` o altri comandi che modificano il repository.

### 2. Leggi il contesto
- I file indicati, ma anche: `Cargo.toml` (dipendenze reali), i moduli correlati (es. se revisioni un handler, guarda come è montato nel router), e i file su cui l'handler appoggia.
- Verifica che la struttura rispetti le convenzioni del progetto (sezione "Convenzioni progetto").

### 3. Verifiche meccaniche (comandi `cargo`)
Esegui in ordine e riporta gli esiti nel report. A differenza dei comandi `git` (pre-autorizzati, vedi passo 1), i comandi `cargo` richiedono il permesso dell'utente: se compare il prompt, attendi l'approvazione prima di proseguire.

```bash
cargo check                      # correttezza: compila?
cargo clippy --all-targets -- -D warnings   # lints idiomatici, zero warning
cargo fmt --check                # formattazione
cargo test                       # test (se presenti)
```

Solo se un esito fallisce, proponi (a parole) la correzione — non applicarla mai.

### 4. Analisi manuale
Controlla il codice contro le checklist qui sotto. Non è un elenco da spuntare tutto: applica solo le voci pertinenti al codice sotto esame.

## Checklist — Correttezza

- Compila senza warning? Ci sono `unused` import, variabili o funzioni?
- Panic/`unwrap()`/`expect()` in percorsi raggiungibili (handler, parsing input)? Un handler non deve mai andare in panic: deve ritornare un errore.
- Error handling: gli errori vengono propagati o inghiottiti? `?` usato dove appropriato?
- Async: niente I/O bloccante dentro funzioni `async`? `.await` presenti dove serve?
- Casi limite: input vuoti, malformati, duplicati, valori limite (es. numeri di telefono)?
- Tipi: il valore di ritorno implementa `IntoResponse`? Gli extractor sono usati con la firma giusta?

## Checklist — Best practice idiomatiche

- Handler come **funzioni nominate** (`async fn`), non closure inline.
- Router composto in una **funzione dedicata** (`router()`), non nel `main`.
- Niente `unsafe` (salvo motivato).
- Rust idiomatico: `map`/`and_then` su `Option`/`Result` invece di `unwrap` selvaggi; `let ... else` invece di `if let ... { } else { }` quando più chiaro; `match` esaustivo.
- `rustfmt` e `clippy` puliti.
- **axum 0.8**: stato condiviso tramite `AppState` passato con `.with_state(...)` (mai globali); routing con `routing::get`/`post`/`put`/`delete`; errori come tipo proprio che implementa `IntoResponse`, non `(StatusCode, String)` sparsi.
- **sea-orm 2.0**: entità in modulo separato (generate con `sea-orm-cli`, non scritte a mano insieme al codice applicativo); query col query builder/`Entity` API; **un** pool di connessioni creato all'avvio e condiviso via state — mai una connessione per richiesta.
- `serde`: derive usate correttamente (`Serialize`/`Deserialize`); campi opzionali con `Option`, non defaulting manuali.

## Checklist — Manutenibilità

- **Separazione delle responsabilità**: `main.rs` = solo bootstrap (crea pool, costruisce router, serve); `routes/mod.rs` = composizione rotte; `routes/handlers.rs` = handler HTTP. Nessuno di questi livelli fa il lavoro degli altri.
- **Visibilità minima**: default privato; `pub(super)`/`pub(crate)` solo quando serve davvero. Segnala `pub` ingiustificati.
- Nomi descrittivi (funzioni che dicono l'azione, non "handle"/"do_stuff").
- Nessun codice commentato, dead code, o codice "per il futuro" non usato.
- Funzioni corte con una sola responsabilità.
- Tutte le dipendenze in `Cargo.toml` sono usate? Tutti gli import servono?

## Convenzioni progetto

Stack: `axum 0.8.9`, `sea-orm 2.0.0` (SQLite), `serde`/`serde_json`, `tokio`, edition 2024.

Struttura target concordata (segnala scostamenti):

```
src/
├── main.rs          → bootstrap
├── routes/
│   ├── mod.rs       → pub fn router() + pub mod handlers;
│   └── handlers.rs  → handler HTTP (pub(super))
```

Crescita prevista (non ancora presente, non segnalarla come errore): `state.rs` (`AppState` col pool), `entities/` (entità SeaORM), `error.rs` (errore applicativo → `IntoResponse`).

## Template del report

Usa SEMPRE questa struttura:

```
# Review: <file o file revisionati>

## Verdetto
- ✅ Ok / ⚠️ Revisionabile / 🔴 Da sistemare

## Rilievi
### 🔴 Bloccante
- `file:riga` — problema — perché è un problema — pattern corretto

### 🟠 Importante
- `file:riga` — problema — perché — pattern corretto

### 🟡 Migliorabile
- `file:riga` — nota — perché — come migliorarla

## Punti di forza
- Cosa è già fatto bene (sempre: anche nel codice più brutto c'è qualcosa).

## Prossimo passo consigliato
- La singola cosa più importante da sistemare prima, e perché.
```

Regole del report:

- Ogni rilievo è **didattico**: mai solo "usa X" — spiega perché X è meglio nel contesto di quel codice.
- Se il codice è corretto e pulito, dillo senza inventare problemi per riempire il template.
- Massimo 5 rilievi a priorità: se ce ne sono di più, raggruppa quelli correlati.
- Le righe si citano come `file:riga` (es. `src/routes/handlers.rs:3`).
- La lingua del report segue quella dell'utente.

## Misura del successo

L'utente chiude la review avendo capito *cosa* sistemare, *dove* e *perché* — e ha gli strumenti per applicarlo da solo. Non conta il numero di rilievi trovati: conta la chiarezza di quelli giusti.
