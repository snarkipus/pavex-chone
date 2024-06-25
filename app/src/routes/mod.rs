pub mod greet;
pub mod login;
pub mod ping;
pub mod tickets;

use pavex::blueprint::router::{DELETE, POST};
use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/api/ping", f!(self::ping::get));
    bp.route(GET, "/api/greet/:name", f!(self::greet::get));
    bp.route(POST, "/web/login", f!(self::login::post));
    bp.route(POST, "/web/tickets", f!(self::tickets::post));
    bp.route(GET, "/web/tickets", f!(self::tickets::get));
    bp.route(DELETE, "/web/tickets/:id", f!(self::tickets::delete));
}
