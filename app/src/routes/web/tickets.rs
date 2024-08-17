use pavex::{
    request::path::PathParams,
    response::{body::Json, Response},
};

use crate::tickets::TicketForCreate;
use crate::{configuration::ModelController, ctx::Ctx};

// region:    -- REST Handlers --
pub async fn post(mc: &ModelController, ctx: Ctx, ticket_fc: TicketForCreate) -> Response {
    let ticket = mc.create_ticket(ctx, ticket_fc).await;

    match ticket {
        Ok(ticket) => {
            let json = Json::new(ticket).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        Err(_) => Response::internal_server_error().set_typed_body("Failed to create the ticket"),
    }
}

pub async fn get(mc: &ModelController, ctx: Ctx) -> Response {
    let tickets = mc.list_tickets(ctx).await;

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

pub async fn delete(mc: &ModelController, ctx: Ctx, id: PathParams<TicketId>) -> Response {
    let ticket = mc.delete_ticket(ctx, id.0.id).await;

    match ticket {
        Ok(ticket) => {
            let json = Json::new(ticket).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        Err(_) => Response::internal_server_error().set_typed_body("Failed to delete the ticket"),
    }
}
// endregion: -- REST Handlers --
