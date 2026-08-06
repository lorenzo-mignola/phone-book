# AGENTS.md

File memoria del progetto. opencode lo carica automaticamente in ogni sessione
(per tutti gli agenti, incluso rust-master): contiene il contesto di phone-book
affinché non serva ricostruirlo da zero ogni volta.

## Panoramica

API REST di una rubrica telefonica (contatti e numeri di telefono) in Rust.
Crate unico `phone-book` con `src/main.rs` (binario) e `src/lib.rs` (libreria,
usata anche dai test di integrazione).

## Stack

- **axum** 0.8 — web framework
- **sea-orm** 2.0 (features: `sqlx-sqlite`, `runtime-tokio`, `macros`,
  `with-json`, `schema-sync`, `entity-registry`) — ORM su SQLite
- **tokio**, **serde**/**serde_json**, **tower-http** (`trace`),
  **tracing**/**tracing-subscriber**, **dotenvy**
- Dev: **axum-test** (test di integrazione)

## Architettura

Layered, dalla rete al DB:

```
routes (handler axum) → dto (in/out HTTP) → repository (query, transaction)
→ entity (modelli sea-orm)
```

- `src/main.rs` — bootstrap: dotenv, tracing, `db::connect`, `routes::router`,
  listener su `0.0.0.0:3000`
- `src/lib.rs` — dichiara tutti i moduli (`db`, `dto`, `entity`, `error`,
  `repository`, `routes`, `state`)
- `src/state.rs` — `AppState { db: DatabaseConnection }` (Clone)
- `src/db.rs` — `connect()` apre il DB e chiama `setup_schema()`, che sincronizza
  lo schema via entity-registry (`get_schema_registry("phone_book::entity::*")`)
- `src/error.rs` — `AppError { NotFound, Db }` con `IntoResponse` (body JSON)
- `src/routes/mod.rs` — `router(AppState)`: merge dei sub-router + `TraceLayer`
- `src/entity/` — `contacts`, `phone_numbers`, `country_code`, `number`
- `src/dto/` — `contact_dto`, `create_contact_dto`, `phone_number_dto`,
  `create_phone_number_dto`
- `src/repository/` — `contacts` (find_all, find_by_id, create_contact),
  `contact_with_numbers` (struct aggregata `{ contact, numbers }`)

## Dominio

- `contacts` 1→N `phone_numbers`
- `contacts::Model`: `id: i32` (PK), `first_name: String`, `last_name: Option<String>`
- `phone_numbers::Model`: `id: i32` (PK), `country_code: CountryCode`,
  `number: Number`, `contact_id: i32` (FK)
- `CountryCode` — enum attivo sea-orm (`CH`, `IT`) con `prefix()` → `+41`/`+39`
- `Number` — newtype `pub struct Number(pub String)` con `Display`
- Conversioni con `From`: DTO → `ActiveModel` (in `dto/`), entity → DTO
  (in `dto/`), tuple `(contact, numeri)` → `ContactWithNumbers` (in `repository/`)

## API

| Metodo | Path             | Body                               | Risposta               |
|--------|------------------|------------------------------------|------------------------|
| GET    | `/contacts`      | —                                  | `200` `Vec<ContactDto>` |
| GET    | `/contacts/{id}` | —                                  | `200` `ContactDto` / `404` |
| POST   | `/contacts`      | `CreateContactDto`                 | `201` `ContactDto`     |

- `ContactDto`: `{ id, first_name, last_name, phone_numbers }` dove ogni numero
  è una stringa formattata `"<prefisso> <numero>"` (es. `"+41 1234"`)
- `CreateContactDto`: `{ first_name, last_name?, phone_numbers: [{ country_code, number }] }`
- La creazione è in una transazione (contatto + tutti i numeri) e poi rilancia
  `find_by_id` per la risposta

## Test

- Integrazione in `tests/contacts.rs` (axum-test): list, get, 404, create
- `tests/util.rs` — `setup_test()`: DB SQLite in-memory, `setup_schema`, seed
  di un contatto `id=1` con numero `+41 1234`, `TestServer`

## Convenzioni

Tutti gli agenti devono seguire le best practice idiomatiche di Rust:

- Rust idiomatico: `map`/`and_then` su `Option`/`Result`, `let ... else` quando
  più chiaro, match esaustivo, niente `unsafe`.
- Niente `unwrap()`/`expect()` in percorsi raggiungibili: propaga gli errori con `?`.
- Prima di committare: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`
  e `cargo test` puliti.

Seguire anche le best practice di clean code:

- Funzioni corte con una sola responsabilità.
- Nomi descrittivi che dicono l'azione (niente "handle"/"do_stuff").
- Niente duplicazione: estrai e riusa.
- Codice leggibile prima di tutto: niente dead code, codice commentato o
  "per il futuro".

In più, per questo progetto:

- Conversioni tra layer tramite `From`.

## Manutenzione di questo file (MEMORIA)

Questo file è la memoria del progetto: deve sempre riflettere lo stato reale
del codice, così ogni agente (in particolare rust-master) riparte da qui senza
rileggere tutto il progetto.

Ogni volta che una modifica tocca una delle aree sotto, **aggiorna questo file
nella stessa modifica**:

- stack o dipendenze (`Cargo.toml`)
- moduli o struttura (`src/`, `tests/`)
- entità, relazioni, enum, tipi di dominio
- rotte API, payload, risposte
- convenzioni o decisioni di progetto

Voci concise e autorevoli: chi legge deve ripartire da qui.

Se la memoria è insufficiente o non allineata (es. in una sessione di mentore),
l'agente può lanciare il subagent `context-updater`
(`.opencode/agents/context-updater.md`), che esplora il codice reale e
aggiorna questo file.
