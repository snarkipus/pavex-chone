use pavex::{
    request::path::PathParams,
    response::{body::Json, Response},
};

use crate::configuration::ModelController;
use crate::tickets::TicketForCreate;

// region:    -- REST Handlers --
pub async fn post(mc: &ModelController, ticket_fc: TicketForCreate) -> Response {
    let ticket = mc.create_ticket(ticket_fc).await;

    match ticket {
        Ok(ticket) => {
            let json = Json::new(ticket).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        Err(_) => Response::internal_server_error().set_typed_body("Failed to create the ticket"),
    }
}

pub async fn get(mc: &ModelController) -> Response {
    let tickets = mc.list_tickets().await;

    match tickets {
        Ok(tickets) => {
            let json = Json::new(tickets).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        Err(_) => Response::internal_server_error().set_typed_body("Failed to list the tickets"),
    }
}

#[PathParams]
pub struct TicketId {
    pub id: u64,
}

pub async fn delete(mc: &ModelController, id: PathParams<TicketId>) -> Response {
    let ticket = mc.delete_ticket(id.0.id).await;

    match ticket {
        Ok(ticket) => {
            let json = Json::new(ticket).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        Err(_) => Response::internal_server_error().set_typed_body("Failed to delete the ticket"),
    }
}
// endregion: -- REST Handlers --
