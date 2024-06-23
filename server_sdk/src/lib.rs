//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
extern crate alloc;
struct ServerState {
    router: pavex_matchit::Router<u32>,
    #[allow(dead_code)]
    application_state: ApplicationState,
}
pub struct ApplicationState {}
pub async fn build_application_state() -> crate::ApplicationState {
    crate::ApplicationState {}
}
pub fn run(
    server_builder: pavex::server::Server,
    application_state: ApplicationState,
) -> pavex::server::ServerHandle {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router(),
        application_state,
    });
    server_builder.serve(route_request, server_state)
}
fn build_router() -> pavex_matchit::Router<u32> {
    let mut router = pavex_matchit::Router::new();
    router.insert("/api/greet/:name", 0u32).unwrap();
    router.insert("/api/login", 1u32).unwrap();
    router.insert("/api/ping", 2u32).unwrap();
    router
}
async fn route_request(
    request: http::Request<hyper::body::Incoming>,
    _connection_info: Option<pavex::connection::ConnectionInfo>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex::response::Response {
    let (request_head, request_body) = request.into_parts();
    #[allow(unused)]
    let request_body = pavex::request::body::RawIncomingBody::from(request_body);
    let request_head: pavex::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.target.path()) {
        Ok(m) => m,
        Err(_) => {
            let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter(
                    vec![],
                )
                .into();
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "*",
            );
            return route_3::entrypoint(
                    &request_head,
                    matched_route_template,
                    &allowed_methods,
                )
                .await;
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex::request::path::RawPathParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/api/greet/:name",
            );
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_1::entrypoint(
                            url_params,
                            matched_route_template,
                            &request_head,
                        )
                        .await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_3::entrypoint(
                            &request_head,
                            matched_route_template,
                            &allowed_methods,
                        )
                        .await
                }
            }
        }
        1u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/api/login",
            );
            match &request_head.method {
                &pavex::http::Method::POST => {
                    route_2::entrypoint(
                            request_body,
                            matched_route_template,
                            &request_head,
                        )
                        .await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::POST,
                        ])
                        .into();
                    route_3::entrypoint(
                            &request_head,
                            matched_route_template,
                            &allowed_methods,
                        )
                        .await
                }
            }
        }
        2u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/api/ping",
            );
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_0::entrypoint(matched_route_template, &request_head).await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_3::entrypoint(
                            &request_head,
                            matched_route_template,
                            &allowed_methods,
                        )
                        .await
                }
            }
        }
        i => unreachable!("Unknown route id: {}", i),
    }
}
pub mod route_0 {
    pub async fn entrypoint<'a>(
        s_0: pavex::request::path::MatchedPathPattern,
        s_1: &'a pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_0(s_0, s_1).await;
        response
    }
    async fn stage_1<'a>(
        s_0: pavex::request::path::MatchedPathPattern,
        s_1: &'a pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_1(s_0, s_1).await;
        response
    }
    async fn stage_2<'a>(s_0: &'a pavex_tracing::RootSpan) -> pavex::response::Response {
        let response = handler().await;
        let response = post_processing_0(response, s_0).await;
        response
    }
    async fn wrapping_0(
        v0: pavex::request::path::MatchedPathPattern,
        v1: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = crate::route_0::Next0 {
            s_0: v0,
            s_1: v1,
            next: stage_1,
        };
        let v3 = pavex::middleware::Next::new(v2);
        let v4 = pavex::middleware::wrap_noop(v3).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v4)
    }
    async fn wrapping_1(
        v0: pavex::request::path::MatchedPathPattern,
        v1: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = pavex::telemetry::ServerRequestId::generate();
        let v3 = app::telemetry::root_span(v1, v0, v2);
        let v4 = crate::route_0::Next1 {
            s_0: &v3,
            next: stage_2,
        };
        let v5 = pavex::middleware::Next::new(v4);
        let v6 = <pavex_tracing::RootSpan as core::clone::Clone>::clone(&v3);
        let v7 = pavex_tracing::logger(v6, v5).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v7)
    }
    async fn handler() -> pavex::response::Response {
        let v0 = app::routes::ping::get();
        <http::StatusCode as pavex::response::IntoResponse>::into_response(v0)
    }
    async fn post_processing_0(
        v0: pavex::response::Response,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = app::telemetry::response_logger(v0, v1).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: pavex::request::path::MatchedPathPattern,
        s_1: &'a pavex::request::RequestHead,
        next: fn(
            pavex::request::path::MatchedPathPattern,
            &'a pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1)
        }
    }
    struct Next1<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex_tracing::RootSpan,
        next: fn(&'a pavex_tracing::RootSpan) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next1<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}
pub mod route_1 {
    pub async fn entrypoint<'a, 'b, 'c>(
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'c pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_0(s_0, s_1, s_2).await;
        response
    }
    async fn stage_1<'a, 'b, 'c>(
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: &'c pavex::request::RequestHead,
        s_2: pavex::request::path::MatchedPathPattern,
    ) -> pavex::response::Response {
        let response = wrapping_1(s_0, s_2, s_1).await;
        response
    }
    async fn stage_2<'a, 'b, 'c, 'd>(
        s_0: &'a pavex_tracing::RootSpan,
        s_1: pavex::request::path::RawPathParams<'b, 'c>,
        s_2: &'d pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = handler(s_0, s_1, s_2).await;
        let response = post_processing_0(response, s_0).await;
        response
    }
    async fn wrapping_0(
        v0: pavex::request::path::RawPathParams<'_, '_>,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = crate::route_1::Next0 {
            s_0: v0,
            s_1: v2,
            s_2: v1,
            next: stage_1,
        };
        let v4 = pavex::middleware::Next::new(v3);
        let v5 = pavex::middleware::wrap_noop(v4).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v5)
    }
    async fn wrapping_1(
        v0: pavex::request::path::RawPathParams<'_, '_>,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = pavex::telemetry::ServerRequestId::generate();
        let v4 = app::telemetry::root_span(v2, v1, v3);
        let v5 = crate::route_1::Next1 {
            s_0: &v4,
            s_1: v0,
            s_2: v2,
            next: stage_2,
        };
        let v6 = pavex::middleware::Next::new(v5);
        let v7 = <pavex_tracing::RootSpan as core::clone::Clone>::clone(&v4);
        let v8 = pavex_tracing::logger(v7, v6).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v8)
    }
    async fn handler(
        v0: &pavex_tracing::RootSpan,
        v1: pavex::request::path::RawPathParams<'_, '_>,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = app::user_agent::UserAgent::extract(v2);
        let v4 = match v3 {
            Ok(ok) => ok,
            Err(v4) => {
                return {
                    let v5 = app::user_agent::invalid_user_agent(&v4);
                    let v6 = pavex::Error::new(v4);
                    app::telemetry::error_logger(&v6, v0).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v5,
                    )
                };
            }
        };
        let v5 = pavex::request::path::PathParams::extract(v1);
        let v6 = match v5 {
            Ok(ok) => ok,
            Err(v6) => {
                return {
                    let v7 = pavex::request::path::errors::ExtractPathParamsError::into_response(
                        &v6,
                    );
                    let v8 = pavex::Error::new(v6);
                    app::telemetry::error_logger(&v8, v0).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v7,
                    )
                };
            }
        };
        let v7 = app::routes::greet::get(v6, v4);
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v7)
    }
    async fn post_processing_0(
        v0: pavex::response::Response,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = app::telemetry::response_logger(v0, v1).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    struct Next0<'a, 'b, 'c, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: &'c pavex::request::RequestHead,
        s_2: pavex::request::path::MatchedPathPattern,
        next: fn(
            pavex::request::path::RawPathParams<'a, 'b>,
            &'c pavex::request::RequestHead,
            pavex::request::path::MatchedPathPattern,
        ) -> T,
    }
    impl<'a, 'b, 'c, T> std::future::IntoFuture for Next0<'a, 'b, 'c, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2)
        }
    }
    struct Next1<'a, 'b, 'c, 'd, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex_tracing::RootSpan,
        s_1: pavex::request::path::RawPathParams<'b, 'c>,
        s_2: &'d pavex::request::RequestHead,
        next: fn(
            &'a pavex_tracing::RootSpan,
            pavex::request::path::RawPathParams<'b, 'c>,
            &'d pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, 'b, 'c, 'd, T> std::future::IntoFuture for Next1<'a, 'b, 'c, 'd, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2)
        }
    }
}
pub mod route_2 {
    pub async fn entrypoint<'a>(
        s_0: pavex::request::body::RawIncomingBody,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'a pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_0(s_0, s_1, s_2).await;
        response
    }
    async fn stage_1<'a>(
        s_0: pavex::request::body::RawIncomingBody,
        s_1: &'a pavex::request::RequestHead,
        s_2: pavex::request::path::MatchedPathPattern,
    ) -> pavex::response::Response {
        let response = wrapping_1(s_0, s_2, s_1).await;
        response
    }
    async fn stage_2<'a, 'b>(
        s_0: &'a pavex_tracing::RootSpan,
        s_1: pavex::request::body::RawIncomingBody,
        s_2: &'b pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = handler(s_0, s_1, s_2).await;
        let response = post_processing_0(response, s_0).await;
        response
    }
    async fn wrapping_0(
        v0: pavex::request::body::RawIncomingBody,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = crate::route_2::Next0 {
            s_0: v0,
            s_1: v2,
            s_2: v1,
            next: stage_1,
        };
        let v4 = pavex::middleware::Next::new(v3);
        let v5 = pavex::middleware::wrap_noop(v4).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v5)
    }
    async fn wrapping_1(
        v0: pavex::request::body::RawIncomingBody,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = pavex::telemetry::ServerRequestId::generate();
        let v4 = app::telemetry::root_span(v2, v1, v3);
        let v5 = crate::route_2::Next1 {
            s_0: &v4,
            s_1: v0,
            s_2: v2,
            next: stage_2,
        };
        let v6 = pavex::middleware::Next::new(v5);
        let v7 = <pavex_tracing::RootSpan as core::clone::Clone>::clone(&v4);
        let v8 = pavex_tracing::logger(v7, v6).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v8)
    }
    async fn handler(
        v0: &pavex_tracing::RootSpan,
        v1: pavex::request::body::RawIncomingBody,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = <pavex::request::body::BodySizeLimit as std::default::Default>::default();
        let v4 = pavex::request::body::BufferedBody::extract(v2, v1, v3).await;
        let v5 = match v4 {
            Ok(ok) => ok,
            Err(v5) => {
                return {
                    let v6 = pavex::request::body::errors::ExtractBufferedBodyError::into_response(
                        &v5,
                    );
                    let v7 = pavex::Error::new(v5);
                    app::telemetry::error_logger(&v7, v0).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v6,
                    )
                };
            }
        };
        let v6 = app::login_payload::AuthStatus::extract(v2, &v5);
        let v7 = match v6 {
            Ok(ok) => ok,
            Err(v7) => {
                return {
                    let v8 = app::login_payload::invalid_credentials(&v7);
                    let v9 = pavex::Error::new(v7);
                    app::telemetry::error_logger(&v9, v0).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v8,
                    )
                };
            }
        };
        let v8 = pavex::request::body::JsonBody::extract(v2, &v5);
        let v9 = match v8 {
            Ok(ok) => ok,
            Err(v9) => {
                return {
                    let v10 = pavex::request::body::errors::ExtractJsonBodyError::into_response(
                        &v9,
                    );
                    let v11 = pavex::Error::new(v9);
                    app::telemetry::error_logger(&v11, v0).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v10,
                    )
                };
            }
        };
        let v10 = app::routes::login::post(&v9, v7).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v10)
    }
    async fn post_processing_0(
        v0: pavex::response::Response,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = app::telemetry::response_logger(v0, v1).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: pavex::request::body::RawIncomingBody,
        s_1: &'a pavex::request::RequestHead,
        s_2: pavex::request::path::MatchedPathPattern,
        next: fn(
            pavex::request::body::RawIncomingBody,
            &'a pavex::request::RequestHead,
            pavex::request::path::MatchedPathPattern,
        ) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2)
        }
    }
    struct Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex_tracing::RootSpan,
        s_1: pavex::request::body::RawIncomingBody,
        s_2: &'b pavex::request::RequestHead,
        next: fn(
            &'a pavex_tracing::RootSpan,
            pavex::request::body::RawIncomingBody,
            &'b pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, 'b, T> std::future::IntoFuture for Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2)
        }
    }
}
pub mod route_3 {
    pub async fn entrypoint<'a, 'b>(
        s_0: &'a pavex::request::RequestHead,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'b pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let response = wrapping_0(s_0, s_1, s_2).await;
        response
    }
    async fn stage_1<'a, 'b>(
        s_0: &'a pavex::router::AllowedMethods,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'b pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_1(s_2, s_1, s_0).await;
        response
    }
    async fn stage_2<'a, 'b>(
        s_0: &'a pavex::router::AllowedMethods,
        s_1: &'b pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let response = handler(s_0).await;
        let response = post_processing_0(response, s_1).await;
        response
    }
    async fn wrapping_0(
        v0: &pavex::request::RequestHead,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v3 = crate::route_3::Next0 {
            s_0: v2,
            s_1: v1,
            s_2: v0,
            next: stage_1,
        };
        let v4 = pavex::middleware::Next::new(v3);
        let v5 = pavex::middleware::wrap_noop(v4).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v5)
    }
    async fn wrapping_1(
        v0: &pavex::request::RequestHead,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v3 = pavex::telemetry::ServerRequestId::generate();
        let v4 = app::telemetry::root_span(v0, v1, v3);
        let v5 = crate::route_3::Next1 {
            s_0: v2,
            s_1: &v4,
            next: stage_2,
        };
        let v6 = pavex::middleware::Next::new(v5);
        let v7 = <pavex_tracing::RootSpan as core::clone::Clone>::clone(&v4);
        let v8 = pavex_tracing::logger(v7, v6).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v8)
    }
    async fn handler(v0: &pavex::router::AllowedMethods) -> pavex::response::Response {
        let v1 = pavex::router::default_fallback(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    async fn post_processing_0(
        v0: pavex::response::Response,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = app::telemetry::response_logger(v0, v1).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    struct Next0<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::router::AllowedMethods,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'b pavex::request::RequestHead,
        next: fn(
            &'a pavex::router::AllowedMethods,
            pavex::request::path::MatchedPathPattern,
            &'b pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, 'b, T> std::future::IntoFuture for Next0<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2)
        }
    }
    struct Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::router::AllowedMethods,
        s_1: &'b pavex_tracing::RootSpan,
        next: fn(&'a pavex::router::AllowedMethods, &'b pavex_tracing::RootSpan) -> T,
    }
    impl<'a, 'b, T> std::future::IntoFuture for Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1)
        }
    }
}