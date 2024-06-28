use crate::routes::web::tickets_bp;
use crate::{configuration, routes, telemetry};
use pavex::blueprint::Blueprint;
use pavex::cookie::CookieKit;
use pavex::f;
use pavex::kit::ApiKit;

/// The main blueprint, containing all the routes, middlewares, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    CookieKit::new()
        .with_default_processor_config()
        .register(&mut bp);
    ApiKit::new().register(&mut bp);
    telemetry::register(&mut bp);
    configuration::register(&mut bp);

    bp.request_scoped(f!(crate::user_agent::UserAgent::extract))
        .error_handler(f!(crate::user_agent::invalid_user_agent));

    bp.request_scoped(f!(crate::credentials::AuthStatus::extract))
        .error_handler(f!(crate::credentials::invalid_credentials));

    bp.singleton(f!(crate::model::ModelController::new));

    bp.request_scoped(f!(crate::model::TicketForCreate::extract))
        .error_handler(f!(crate::model::invalid_ticket));

    routes::register(&mut bp);
    bp.nest_at("/web", tickets_bp());
    bp
}
