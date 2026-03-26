use actix_web::{web, App, HttpServer, Responder};
use std::io;

mod server;
mod model_loader;

async fn health_check() -> impl Responder {
    "ML Model Server is healthy!"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting ML Model Server...");

    // Load models (example: a dummy model)
    let model_registry = model_loader::ModelRegistry::new();
    model_registry.load_model("dummy_model", "./models/dummy_model.onnx").expect("Failed to load dummy model");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(model_registry.clone())) // Pass model registry to handlers
            .route("/health", web::get().to(health_check))
            .route("/predict/{model_name}", web::post().to(server::predict_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
