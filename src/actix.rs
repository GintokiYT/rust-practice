use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    message: String,
}

async fn json_response() -> impl Responder {
    let response = JsonResponse {
        message: "Hello world!".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// Cambiamos la función para que pueda ser llamada desde main.rs
pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(json_response))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
