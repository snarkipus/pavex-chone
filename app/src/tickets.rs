use pavex::{
    request::{
        body::{errors::ExtractJsonBodyError, BufferedBody, JsonBody},
        RequestHead,
    },
    response::Response,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// region:    -- Ticket Types --
pub type TicketResult<T> = std::result::Result<T, TicketError>;

// Ticket Model Errors
#[derive(Debug, Error)]
pub enum TicketError {
    #[error("Ticket not found")]
    NotFound { id: u64 },
    #[error("Failed to extract the JSON payload")]
    ExtractJsonBody(#[from] ExtractJsonBodyError),
}

// Ticket error handler
pub fn invalid_ticket(e: &TicketError) -> Response {
    match e {
        TicketError::ExtractJsonBody(_) => Response::bad_request().set_typed_body("Invalid JSON"),
        TicketError::NotFound { id } => {
            Response::not_found().set_typed_body(format!("Ticket not found: {}", id))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TicketForCreate {
    pub title: String,
}

impl TicketForCreate {
    #[tracing::instrument(
        name = "extract ticket for creation",
        skip(request_head, buffered_body),
        fields(
            ticket_title = tracing::field::Empty,
            error = tracing::field::Empty,
        )
    )]
    pub fn extract(request_head: &RequestHead, buffered_body: &BufferedBody) -> TicketResult<Self> {
        let ticket_fc =
            JsonBody::<TicketForCreate>::extract(request_head, buffered_body).map_err(|e| {
                tracing::Span::current().record("error", e.to_string());
                TicketError::ExtractJsonBody(e)
            })?;

        tracing::Span::current().record("ticket_title", &ticket_fc.0.title);
        Ok(ticket_fc.0)
    }
}
// endregion: -- Ticket Types --
