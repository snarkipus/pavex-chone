pub mod greet;
pub mod login;
pub mod ping;
pub mod web;

use pavex::blueprint::router::POST;
use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/api/ping", f!(self::ping::get));
    bp.route(GET, "/api/greet/:name", f!(self::greet::get));
    bp.route(POST, "/web/login", f!(self::login::post));
}
