use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model_loader::ModelRegistry;

#[derive(Deserialize)]
pub struct PredictRequest {
    pub input_data: Vec<f32>,
}

#[derive(Serialize)]
pub struct PredictResponse {
    pub model_name: String,
    pub prediction: Vec<f32>,
}

pub async fn predict_handler(path: web::Path<String>, req: web::Json<PredictRequest>, model_registry: web::Data<ModelRegistry>) -> impl Responder {
    let model_name = path.into_inner();
    let input_data = req.input_data.clone();

    let registry = model_registry.get_ref();

    match registry.get_model(&model_name) {
        Some(model) => {
            // In a real scenario, you would pass input_data to the loaded model
            // and get a real prediction. For this example, we'll return a dummy prediction.
            println!("Received prediction request for model: {} with input: {:?}", model_name, input_data);
            let dummy_prediction = vec![0.1, 0.9]; // Example dummy prediction

            let response = PredictResponse {
                model_name,
                prediction: dummy_prediction,
            };
            HttpResponse::Ok().json(response)
        }
        None => {
            HttpResponse::NotFound().body(format!("Model {} not found", model_name))
        }
    }
}
