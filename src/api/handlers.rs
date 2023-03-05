use actix_web::{App, Error, HttpServer};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::get;
use crate::api::health::ping;


pub fn register_handlers() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    return App::new().
        route("/ping/", get().to(ping)).
        route("/", get().to(ping))
}

pub async fn start_server() -> std::io::Result<()> {
   HttpServer::new(|| { register_handlers()})
        .bind("127.0.0.1:8000")?
        .run()
        .await
}