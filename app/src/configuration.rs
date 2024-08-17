use pavex::{blueprint::Blueprint, f, t};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    ctx::Ctx,
    tickets::{Ticket, TicketError, TicketForCreate, TicketResult},
};

#[derive(serde::Deserialize, Debug, Clone)]
/// The configuration object holding all the values required
/// to configure the application.
pub struct AppConfig {
    pub mc: ModelController,
}

impl AppConfig {
    pub fn register(bp: &mut Blueprint) {
        bp.prebuilt(t!(self::AppConfig));
        bp.singleton(f!(crate::configuration::ModelController::new));
    }
}

// region:    -- Model Controller --
#[derive(Clone, Debug)]
pub struct ModelController {
    pub tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// NOTE: This doesn't seem to be a grown up way to do this - assuming the AppConfig would just contain
//       the interface to the database and wouldn't attempt to deserialize the actual database
impl<'de> Deserialize<'de> for ModelController {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ModelController {
            tickets_store: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

// Constructor
impl ModelController {
    pub async fn new() -> Self {
        Self {
            tickets_store: Arc::default(),
        }
    }
}

// CRUD Implementation
impl ModelController {
    #[tracing::instrument(
        name = "create ticket",
        skip(self, ctx, ticket_fc),
        fields(
            ticket_id = tracing::field::Empty,
            ticket_title = tracing::field::Empty,
        )
    )]
    pub async fn create_ticket(
        &self,
        ctx: Ctx,
        ticket_fc: TicketForCreate,
    ) -> TicketResult<Ticket> {
        let mut store = self.tickets_store.lock().await;

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));

        tracing::Span::current().record("ticket_id", ticket.id);
        tracing::Span::current().record("ticket_title", &ticket.title);
        Ok(ticket)
    }

    #[tracing::instrument(name = "list tickets", skip(self, _ctx))]
    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>, TicketError> {
        let store = self.tickets_store.lock().await;
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    #[tracing::instrument(
        name = "delete ticket",
        skip(self, _ctx, id),
        fields(
            ticket_id = id,
            error = tracing::field::Empty,
        )
    )]
    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket, TicketError> {
        let mut store = self.tickets_store.lock().await;
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or({
            tracing::Span::current().record("Ticket {} not found error", id);
            TicketError::NotFound { id }
        })
    }
}
// endregion: -- Model Controller --
