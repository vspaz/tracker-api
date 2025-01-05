use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::{get, post, resource, scope};
use actix_web::{App, Error};
mod health;
mod index;
mod segment;

pub fn url_dispatcher() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .wrap(actix_web::middleware::Logger::default())
        .service(
            scope("/api/v1")
                .service(resource("/track").route(post().to(segment::track)))
                .service(resource("/t").route(post().to(segment::track)))
                .service(resource("/page").route(post().to(segment::page)))
                .service(resource("/p").route(post().to(segment::page)))
                .service(resource("/identify").route(post().to(segment::identify)))
                .service(resource("/i").route(post().to(segment::identify)))
                .service(resource("/alias").route(post().to(segment::alias)))
                .service(resource("/a").route(post().to(segment::alias)))
                .service(resource("/screen").route(post().to(segment::screen)))
                .service(resource("/s").route(post().to(segment::screen)))
                .service(resource("/batch").route(post().to(segment::batch)))
                .service(resource("/import").route(post().to(segment::batch))),
        )
        .service(resource("/index").route(get().to(index::index)))
        .service(resource("/").route(get().to(index::index)))
        .service(resource("/ping").route(get().to(health::ping)))
        .service(resource("/ping").route(post().to(health::ping)))
}

#[cfg(test)]
mod tests {
    use crate::api::handlers::index::ResponseOk;
    use crate::api::handlers::url_dispatcher;
    use actix_web::{http, test};
    use std::collections::HashMap;

    async fn assert_handler_ok(endpoint: &str) {
        let app = test::init_service(url_dispatcher()).await;
        let req = test::TestRequest::get().uri(endpoint).to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
        let body: ResponseOk = test::read_body_json(resp).await;
        assert_eq!(body.status, "200 OK");
        assert_eq!(body.message, "OK");
    }

    #[actix_web::test]
    async fn test_index_ok() {
        assert_handler_ok("/index").await;
    }

    #[actix_web::test]
    async fn test_root_ok() {
        assert_handler_ok("/").await;
    }

    #[actix_web::test]
    async fn test_ping_ok() {
        let app = test::init_service(url_dispatcher()).await;
        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
        let body: HashMap<String, String> = test::read_body_json(resp).await;
        assert_eq!(body.get("ping").unwrap(), "pong");
    }
}
