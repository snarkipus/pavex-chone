use crate::user_agent::UserAgent;
use pavex::request::path::PathParams;
use pavex::response::Response;

#[PathParams]
pub struct GreetParams {
    pub name: String,
}

pub fn get(params: PathParams<GreetParams>, user_agent: UserAgent) -> Response {
    if let UserAgent::Unknown = user_agent {
        return Response::unauthorized().set_typed_body("You must provide a `User-Agent` header.");
    }
    let GreetParams { name } = params.0;
    Response::ok().set_typed_body(format!("Hello, {name}!"))
}
