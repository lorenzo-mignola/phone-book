# 📒 phone-book

A phone book written in Rust, built to learn the language. A small but real
project: it exposes a **REST API** under `/api` plus a **tiny web UI** served
as HTML, backed by SQLite.

## 🎯 Why it exists

It's a learning project to practice the Rust fundamentals:

- ownership, `Option`/`Result`, `?`, `From` for layer conversions
- a real web app with **axum** (HTTP framework)
- persistence with **sea-orm** (ORM) on SQLite
- HTML templates with **askama**
- API integration tests
- feature folders: each feature contains all its layers, from network to DB

## 🧱 Stack

| What      | Crate                         |
|-----------|-------------------------------|
| HTTP      | axum 0.8                      |
| ORM       | sea-orm 2.0 (SQLite)          |
| Templates | askama 0.16                   |
| Runtime   | tokio                         |
| Logging   | tracing / tracing-subscriber  |
| Tests     | axum-test (integration)       |

## 🚀 Getting started

```sh
cargo run
```

It needs the `DATABASE_URL` variable (a `.env` file works fine), e.g.:

```sh
DATABASE_URL="sqlite://data/phone_book.db?mode=rwc"
```

The app listens on `http://localhost:3000`: that's where the UI lives, while
the API is under `/api`.

## 🔌 API

| Method | Path                 | Description                             |
|--------|----------------------|-----------------------------------------|
| GET    | `/api/contacts`      | list contacts (with their numbers)      |
| GET    | `/api/contacts/{id}` | single contact or `404`                 |
| POST   | `/api/contacts`      | create a contact with its numbers       |
| PUT    | `/api/contacts/{id}` | replace a contact and its numbers (`404` if missing) |

Each number is formatted with its country prefix, e.g. `+41 1234`.

## 🧪 Tests

```sh
cargo test
```

## 🗂️ Structure (in short)

- `src/main.rs` — app bootstrap
- `src/features/contacts/` — the "contacts" feature: router, handlers, service,
  repository, entity
- `src/features/ui/` — the web page served at the root
- `src/entity/` — the sea-orm models (contacts and numbers)
- `tests/` — API integration tests

## 🤖 Agent docs

`AGENTS.md` — the project memory file read by the coding agents — is written
in **Italian**, on purpose: it's a way to test how well an AI model handles
context switching between languages (English codebase, Italian documentation).
