use crate::{configuration, routes, telemetry};
use pavex::blueprint::Blueprint;
use pavex::f;
use pavex::kit::ApiKit;

/// The main blueprint, containing all the routes, middlewares, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);
    telemetry::register(&mut bp);
    configuration::register(&mut bp);

    bp.request_scoped(f!(crate::user_agent::UserAgent::extract))
        .error_handler(f!(crate::user_agent::invalid_user_agent));

    bp.request_scoped(f!(crate::login_payload::AuthStatus::extract))
        .error_handler(f!(crate::login_payload::invalid_credentials));

    routes::register(&mut bp);
    bp
}
