use actix_web::{http, test};
use tracker::api::handlers;

async fn assert_handler_ok(endpoint: &str) {
    let app = test::init_service(handlers::url_dispatcher()).await;
    let req = test::TestRequest::get().uri(endpoint).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
    let body: handlers::index::ResponseOk = test::read_body_json(resp).await;
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
