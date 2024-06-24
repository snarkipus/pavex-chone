use anyhow::Result;
use pavex::request::body::errors::ExtractJsonBodyError;
use pavex::request::body::{BufferedBody, JsonBody};
use pavex::request::RequestHead;
use pavex::response::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

pub type TicketResult<T> = std::result::Result<T, TicketError>;

// Ticket Model Errors
#[derive(Debug, Error)]
pub enum TicketError {
    #[error("Ticket not found")]
    NotFound { id: u64 },
    #[error("Failed to extract the JSON payload")]
    ExtractJsonBody(#[from] ExtractJsonBodyError),
}

pub fn invalid_ticket(e: &TicketError) -> Response {
    match e {
        TicketError::ExtractJsonBody(_) => Response::bad_request().set_typed_body("Invalid JSON"),
        TicketError::NotFound { id } => {
            Response::not_found().set_typed_body(format!("Ticket not found: {}", id))
        }
    }
}

// region:    -- Ticket Types --
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

impl TicketForCreate {
    pub fn extract(request_head: &RequestHead, buffered_body: &BufferedBody) -> TicketResult<Self> {
        let ticket_fc = JsonBody::<TicketForCreate>::extract(request_head, buffered_body)
            .map_err(TicketError::ExtractJsonBody)?;

        Ok(ticket_fc.0)
    }
}

// endregion: -- Ticket Types --

// region:    -- Model Controller --
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> TicketResult<Ticket> {
        let mut store = self.tickets_store.lock().await;

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().await;
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket, TicketError> {
        let mut store = self.tickets_store.lock().await;
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(TicketError::NotFound { id })
    }
}

// endregion: -- Model Controller --
