use actix_web::{http, test};
use std::collections::HashMap;
use tracker::api::handlers;

#[actix_web::test]
async fn test_ping_ok() {
    let app = test::init_service(handlers::url_dispatcher()).await;
    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
    let body: HashMap<String, String> = test::read_body_json(resp).await;
    assert_eq!(body.get("ping").unwrap(), "pong");
}
