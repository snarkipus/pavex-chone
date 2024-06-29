use pavex::{
    blueprint::{
        router::{DELETE, GET, POST},
        Blueprint,
    },
    f,
};

pub mod mw_auth;
pub mod tickets;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn tickets_bp() -> Blueprint {
    let mut bp = Blueprint::new();

    bp.request_scoped(f!(crate::tickets::TicketForCreate::extract))
        .error_handler(f!(crate::tickets::invalid_ticket));
    
    bp.request_scoped(f!(crate::ctx::Ctx::new));

    bp.pre_process(f!(super::web::mw_auth::mw_require_auth))
        .error_handler(f!(super::web::mw_auth::mw_auth_error));

    bp.route(POST, "/tickets", f!(self::tickets::post));
    bp.route(GET, "/tickets", f!(self::tickets::get));
    bp.route(DELETE, "/tickets/:id", f!(self::tickets::delete));
    bp
}
