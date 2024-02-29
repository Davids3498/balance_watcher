use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main] // Marks the entry point of the application
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // Defines a route
            // Add more routes here
    })
    .bind("127.0.0.1:8080")? // Specifies the address and port to serve on
    .run()
    .await
}