use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize)]
struct SubscribeFormData {
    name: String,
    email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: Option<web::Form<SubscribeFormData>>) -> HttpResponse {
    match form {
        Some(form) => {
            if !form.name.is_empty() && !form.email.is_empty() {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
