// Module imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/error", web::get().to(error_handler));
}

//Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Gugus, greatings from ACTIX") 
}

pub async fn error_handler() -> impl Responder {
    HttpResponse::Ok().json("Gugus, error from ACTIX") 
}

// Instantiate and run the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Construct app and configure routes
    let app = move || App::new().configure(general_routes);

    // Start HTTP 
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
