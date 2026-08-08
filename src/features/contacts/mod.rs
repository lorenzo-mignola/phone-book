mod dto;
mod repository;
mod router;
mod routes;
mod service;
mod view;

pub(crate) use router::contacts_router as router;
pub(crate) use view::index::index_handler;
