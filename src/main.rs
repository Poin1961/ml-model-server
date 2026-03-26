use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod model_loader;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(server::predict)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
