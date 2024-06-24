use pavex::response::{body::Json, Response};

use crate::model::{ModelController, TicketForCreate};

// region:    -- REST Handlers --
pub async fn post(mc: ModelController, ticket_fc: TicketForCreate) -> Response {
    let ticket = mc.create_ticket(ticket_fc).await;

    match ticket {
        Ok(ticket) => {
            let json = Json::new(ticket).expect("Failed to serialize the response body");
            Response::ok().set_typed_body(json)
        }
        Err(_) => Response::internal_server_error().set_typed_body("Failed to create the ticket"),
    }
}
// endregion: -- REST Handlers --
