use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/status", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
