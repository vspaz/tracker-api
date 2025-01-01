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
