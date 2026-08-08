# AGENTS.md

File memoria del progetto. opencode lo carica automaticamente in ogni sessione
(per tutti gli agenti, incluso rust-master): contiene il contesto di phone-book
affinché non serva ricostruirlo da zero ogni volta.

## Panoramica

Rubrica telefonica (contatti e numeri di telefono) in Rust: API REST sotto
`/api` più una piccola UI web servita come HTML (askama). Crate unico
`phone-book` con `src/main.rs` (binario) e `src/lib.rs` (libreria, usata anche
dai test di integrazione).

## Stack

- **axum** 0.8 — web framework
- **sea-orm** 2.0 (features: `sqlx-sqlite`, `runtime-tokio`, `macros`,
  `with-json`, `schema-sync`, `entity-registry`) — ORM su SQLite
- **askama** 0.16 + **askama_web** 0.16 (`axum-0.8`) — template engine per la UI
- **tokio** (`full`), **serde**/**serde_json**, **tower-http** (`trace`),
  **tracing**/**tracing-subscriber**, **dotenvy**
- Dev: **axum-test** 21.0.0 (test di integrazione)

## Architettura

Organizzata a feature folder: ogni feature (oggi `contacts`) racchiude i suoi
layer, dalla rete al DB:

```
feature/contacts: routes (handler axum) → service (logica, conversioni DTO)
→ repository → entity (modelli sea-orm globali in src/entity)
```

- `src/main.rs` — bootstrap: dotenv, tracing, `db::connect` con `DATABASE_URL`,
  `routes::router`, listener su `0.0.0.0:3000`
- `src/lib.rs` — dichiara i moduli (`db`, `entity`, `error`, `features`,
  `routes`, `state`); `dto`/`repository` non esistono più a livello globale
- `src/state.rs` — `AppState { db: DatabaseConnection }` (Clone)
- `src/db.rs` — `connect()` apre il DB e chiama `setup_schema()`, che sincronizza
  lo schema via entity-registry (`get_schema_registry("phone_book::entity::*")`)
- `src/error.rs` — `AppError { NotFound, Db }` con `IntoResponse` (body JSON)
- `src/routes/mod.rs` — `router(AppState)`:
  `.nest("/api", contacts::router())` + `.merge(ui_router())` + `TraceLayer`
- `src/features/mod.rs` — `pub(crate) mod contacts; pub(crate) mod ui;`
- `src/features/contacts/` — `mod.rs` (riesporta `router` e `index_handler`),
  `router.rs` (sub-router `/contacts`), `dto/`, `repository/`, `routes/`,
  `service/`, `view/`
- `src/features/contacts/router.rs` — `Router<AppState>` senza parametri:
  GET/POST `/contacts`, GET `/contacts/{id}` (montato sotto `/api`)
- `src/features/contacts/repository/` — `contacts` (find_all, find_by_id,
  create_contact), `contact_with_numbers` (struct aggregata `{ contact, numbers }`)
- `src/features/contacts/service/` — `contacts`: `find_all`, `find_by_id`,
  `create_contact` (orchestra il repository, splitta i DTO e converte entità →
  DTO)
- `src/features/contacts/routes/` — handler axum: `list_contacts`,
  `get_contact`, `save_contact` (delegano al service)
- `src/features/contacts/view/` — `index` (template askama, vedi UI sotto)
- `src/features/ui/` — `mod.rs` + `router.rs`: `ui_router()`
  (`Router<AppState>`) con GET `/` → `index_handler`

### UI web

- `IndexTemplate` (in `view/index.rs`): `#[derive(Template, WebTemplate)]` con
  `#[template(path = "index.html")]`, campo `contacts: Vec<ContactDto>`;
  `index_handler` la popola via `service::contacts::find_all` (accede al DB)
- `templates/index.html` — pagina HTML (pico.css via CDN) che itera i contatti
- UI montata da `ui_router()` alla radice (`/`), le API restano sotto `/api`

## Dominio

- `contacts` 1→N `phone_numbers`
- `contacts::Model`: `id: i32` (PK), `first_name: String`, `last_name: Option<String>`
- `phone_numbers::Model`: `id: i32` (PK), `country_code: CountryCode`,
  `number: Number`, `contact_id: i32` (FK)
- `CountryCode` — enum attivo sea-orm (`CH`, `IT`) con `prefix()` → `+41`/`+39`
- `Number` — newtype `pub struct Number(pub String)` con `Display`
- Conversioni con `From`:
  - `CreateContactDto` → `(contacts::ActiveModel, Vec<phone_numbers::ActiveModel>)`
  - `CreatePhoneNumberDto` → `phone_numbers::ActiveModel`
  - entity per entity: `(contacts::Model, Vec<phone_numbers::Model>)` →
    `ContactWithNumbers` (in `repository/`)
  - `ContactWithNumbers` → `ContactDto` e `phone_numbers::Model` →
    `phone_number_dto::PhoneNumberDto` (in `dto/`)

## API

| Metodo | Path               | Body                               | Risposta               |
|--------|--------------------|------------------------------------|------------------------|
| GET    | `/api/contacts`    | —                                  | `200` `Vec<ContactDto>` |
| GET    | `/api/contacts/{id}` | —                                  | `200` `ContactDto` / `404` |
| POST   | `/api/contacts`    | `CreateContactDto`                 | `201` `ContactDto`     |

- `ContactDto`: `{ id, first_name, last_name, phone_numbers }` dove `last_name`
  è `String` (default `""` quando assente) e ogni numero è una stringa formattata
  `"<prefisso> <numero>"` (es. `"+41 1234"`)
- `CreateContactDto`: `{ first_name, last_name?, phone_numbers: [{ country_code, number }] }`
- Creazione: `save_contact` delega a `service::contacts::create_contact`, che
  splitta `CreateContactDto` in `(contact, numeri)` via `into()`; poi
  `repository::contacts::create_contact` esegue una transazione (inserisce
  contatto + tutti i numeri), committa e *dopo il commit* richiama `find_by_id`
  per restituire il `ContactWithNumbers` completo; il service lo converte in
  `ContactDto` e il handler risponde `201` (la ricerca per risposta avviene nel
  repository, le conversioni DTO nel service, non nel handler)

## Test

- Integrazione in `tests/contacts.rs` (axum-test): list, get, 404, create —
  tutte sull'endpoint `/api/contacts`
- `tests/util.rs` — `setup_test()`: DB SQLite in-memory (max_connections 1),
  `setup_schema`, seed di un contatto `id=1` con numero `+41 1234`,
  `TestServer` costruito con `routes::router`

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