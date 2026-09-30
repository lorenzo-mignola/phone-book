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
- **reqwest** 0.13 — client HTTP, usato solo per l'upstream dicebear (avatar)
- **tokio** (`full`), **serde**/**serde_json**, **tower-http** (`trace`),
  **tracing**/**tracing-subscriber**, **dotenvy**
- Dev: **axum-test** 21.0.0 (test di integrazione), **wiremock** 0.6.5
  (mock del server dicebear nei test dell'avatar)

## Architettura

Organizzata a feature folder: ogni feature (oggi `contacts`) racchiude i suoi
layer, dalla rete al DB:

```
feature/contacts: routes (handler axum) → service (logica, conversioni DTO)
→ repository → entity (modelli sea-orm globali in src/entity)
```

- `src/main.rs` — bootstrap: tracing, `dotenv`, `state::build_state`,
  `routes::router`, listener su `0.0.0.0:3000`
- `src/lib.rs` — dichiara i moduli (`db`, `entity`, `error`, `features`,
  `routes`, `state`); `dto`/`repository` non esistono più a livello globale
- `src/state.rs` — `AppState { db: DatabaseConnection, dicebar_url: String }`
  (Clone) + `build_state()`, che legge `DATABASE_URL` e `DICEBAR_URL` dall'env
  e apre il DB
- `src/db.rs` — `connect()` apre il DB e chiama `setup_schema()`, che sincronizza
  lo schema via entity-registry (`get_schema_registry("phone_book::entity::*")`)
- `src/error.rs` — `AppError { NotFound, Db }` con `IntoResponse` (body JSON)
- `src/routes/mod.rs` — `router(AppState)`:
  `.nest("/api", contacts::router())` + `.merge(ui_router())` + `TraceLayer`
- `src/features/mod.rs` — `pub(crate) mod contacts; pub(crate) mod ui;`
- `src/features/contacts/` — `mod.rs` (riesporta `router` come
  `contacts_router` e `index_handler`), `router.rs` (sub-router `/contacts`),
  `dto/`, `repository/`, `routes/`, `service/`, `view/`
- `src/features/contacts/router.rs` — `Router<AppState>` senza parametri, 6
  route: GET/POST `/contacts`, GET/PUT `/contacts/{id}`,
  GET `/contacts/{id}/phone_numbers`, GET `/contacts/{id}/avatar`
- `src/features/contacts/repository/` — `contacts` (find_all, find_by_id,
  create_contact, update_contact), `phone_numbers`
  (find_phone_numbers_by_contact_id), `contact_with_numbers` (struct aggregata
  `{ contact, numbers }`)
- `src/features/contacts/service/` — `contacts`: `find_all`, `find_by_id`,
  `create_contact`, `update_contact` (orchestra il repository, splitta i DTO e
  converte entità → DTO); `phone_numbers`: `find_phone_numbers_by_contact_id`
  (entità → `PhoneNumberDto`); `avatar`: `get_avatar` + `fetch_avatar`
  (chiama l'upstream dicebar)
- `src/features/contacts/routes/` — handler axum: `list_contacts`,
  `get_contact`, `save_contact`, `update_contact` (delegano al service di
  `contacts`), `get_all_phone_number_by_contact_id` (→ `Json<Vec<PhoneNumberDto>>`),
  `get_contact_avatar` (→ `Html<Body>`)
- `src/features/contacts/view/` — `index` (template askama, vedi UI sotto)
- `src/features/ui/` — `mod.rs` + `router.rs`: `ui_router()`
  (`Router<AppState>`) con GET `/` → `index_handler`

### Avatar (dicebear)

- `service/avatar::get_avatar` carica il contatto con
  `repository::contacts::find_by_id` (`404` se assente), compone il seed
  `"<first_name>_<last_name>"` (`last_name` assente → seed con underscore
  finale) e chiama `fetch_avatar`
- `fetch_avatar` fa `GET {DICEBAR_URL}/10.x/blobs/svg?seed=<seed>` con
  `reqwest::get` e mappa il corpo testuale in `Svg`
- `dto/svg.rs` — `Svg(String)` newtype con `From<String>` e `From<Svg> for Body`;
  nota: pur essendo in `dto/` è un tipo di *presentazione/transport*, non un
  DTO di dominio
- Nota (debito noto): in `fetch_avatar` gli errori di rete (`reqwest`) sono
  mappati a `AppError::NotFound`, quindi un upstream irraggiungibile produce un
  `404` indistinguibile da "contatto inesistente"; `AppError` non ha un
  variant dedicato

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
| PUT    | `/api/contacts/{id}` | `CreateContactDto`                 | `200` `ContactDto` / `404` |
| GET    | `/api/contacts/{id}/phone_numbers` | —              | `200` `Vec<PhoneNumberDto>` |
| GET    | `/api/contacts/{id}/avatar` | —                     | `200` SVG (dicebar) / `404` |

- `ContactDto`: `{ id, first_name, last_name, phone_numbers }` dove `last_name`
  è `String` (default `""` quando assente) e ogni numero è una stringa formattata
  `"<prefisso> <numero>"` (es. `"+41 1234"`)
- `PhoneNumberDto(String)` newtype serializzato come **stringa** (non oggetto),
  costruita da `phone_numbers::Model` con `format!("{} {}", prefix, number)`;
  `Display` delegato al contenuto
- `CreateContactDto`: `{ first_name, last_name?, phone_numbers: [{ country_code, number }] }`
- Creazione: `save_contact` delega a `service::contacts::create_contact`, che
  splitta `CreateContactDto` in `(contact, numeri)` via `into()`; poi
  `repository::contacts::create_contact` esegue una transazione (inserisce
  contatto + tutti i numeri), committa e *dopo il commit* richiama `find_by_id`
  per restituire il `ContactWithNumbers` completo; il service lo converte in
  `ContactDto` e il handler risponde `201` (la ricerca per risposta avviene nel
  repository, le conversioni DTO nel service, non nel handler)
- Aggiornamento: `update_contact` (route) delega a
  `service::contacts::update_contact`, che splitta il DTO; poi
  `repository::contacts::update_contact` in transazione verifica che il
  contatto esista (`404`), sostituisce i numeri (delete + insert) e aggiorna i
  campi del contatto, committa e *dopo il commit* richiama `find_by_id` per
  rispondere `200` con il `ContactDto` completo
- Numeri di un contatto: `GET /contacts/{id}/phone_numbers` → service dedicato
  (`service::phone_numbers`) che riusa
  `repository::phone_numbers::find_phone_numbers_by_contact_id`; nota che
  `find_phone_numbers_by_contact_id` non verifica l'esistenza del contatto,
  quindi su un id inesistente risponde `200` con lista vuota
- Avatar: `GET /contacts/{id}/avatar` → `Html<Body>` con l'SVG di dicebar
  (vedi "Avatar (dicebear)")

## Test

- Integrazione (axum-test), un file per area:
  - `tests/contacts.rs` — list, get, 404, create, update su `/api/contacts`
  - `tests/phone_numbers.rs` — `GET /api/contacts/1/phone_numbers` (1 numero)
  - `tests/avatar.rs` — successo con **wiremock** che mocka
    `/10.x/blobs/svg?seed=test_`, e `404` su contatto inesistente
- `tests/util.rs` — `connect_in_memory()` (SQLite in-memory, max_connections 1)
  + `setup_schema`; `seed_db()` inserisce un contatto `id=1` (`first_name:
  "test"`, senza `last_name`) con numero `+41 1234`; due entry point:
  `setup_test()` (dicebar `""`) e `setup_test_with_dicebar(url)` che
  costruiscono `AppState { db, dicebar_url }` e il `TestServer` con
  `routes::router`
- I test dichiarano `mod util;` localmente: `util.rs` non è un integration
  test autonomo

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