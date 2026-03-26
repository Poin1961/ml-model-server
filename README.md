# Machine Learning Model Server (Rust)

A high-performance, low-latency machine learning model serving framework built in Rust. This project focuses on efficient model inference, concurrent request handling, and robust deployment for production AI applications.

## Features

*   **Blazing Fast Inference**: Leverages Rust's performance characteristics for minimal latency.
*   **Concurrent Request Handling**: Asynchronous architecture for handling multiple inference requests simultaneously.
*   **Model Agnostic**: Designed to serve models from various frameworks (e.g., ONNX, TensorFlow Lite, custom Rust models).
*   **RESTful API**: Provides a clean and well-documented REST API for model predictions.
*   **Metrics & Monitoring**: Integrated with Prometheus for real-time performance monitoring.
*   **Hot Reloading (Planned)**: Future support for updating models without service interruption.

## Installation

```bash
git clone https://github.com/Poin1961/ml-model-server.git
cd ml-model-server
cargo build --release
```

## Usage

First, place your trained model (e.g., an ONNX file) in the `models/` directory.

```bash
./target/release/ml-model-server
```

Then, send inference requests to the API:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"inputs": [[0.1, 0.2, 0.3]]}' http://127.0.0.1:8000/predict
```

## Project Structure

```
ml-model-server/
├── src/
│   ├── main.rs
│   ├── model_loader.rs
│   └── server.rs
├── models/
│   └── my_model.onnx
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
