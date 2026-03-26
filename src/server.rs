use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use ndarray::{Array, ArrayD, IxDyn};
use ort::{Session, GraphOptimizationLevel};

#[derive(Deserialize)]
pub struct PredictRequest {
    pub inputs: Vec<Vec<f32>>,
}

#[derive(Serialize)]
pub struct PredictResponse {
    pub predictions: Vec<Vec<f32>>,
}

#[post("/predict")]
pub async fn predict(req: web::Json<PredictRequest>) -> impl Responder {
    // In a real application, the session would be loaded once and shared.
    // For simplicity, we load it here.
    let session = Session::builder()? // Use `?` for error propagation
        .with_optimization_level(GraphOptimizationLevel::All)?
        .with_model_from_file("models/my_model.onnx")?;

    let input_shape = session.inputs[0].dimensions.iter().map(|&d| d.unwrap()).collect::<Vec<usize>>();
    let output_shape = session.outputs[0].dimensions.iter().map(|&d| d.unwrap()).collect::<Vec<usize>>();

    let flat_inputs: Vec<f32> = req.inputs.iter().flatten().cloned().collect();
    let input_array = Array::from_shape_vec(IxDyn(&input_shape), flat_inputs).unwrap();

    let outputs: Vec<ArrayD<f32>> = session.run(ort::inputs![input_array].unwrap()).unwrap();
    let predictions = outputs[0].iter().map(|&x| vec![x]).collect();

    HttpResponse::Ok().json(PredictResponse { predictions })
}
