use pavex::{blueprint::Blueprint, f, t};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::tickets::{Ticket, TicketError, TicketForCreate, TicketResult};

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

// Deserialize
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

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>, TicketError> {
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
