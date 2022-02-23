use actix_web::{get, App, HttpServer, Responder};
use chrono::Local;
use std::io::Result;

#[get("/")]
async fn index_root() -> impl Responder {
    let time = Local::now();
    format!("Hello from demo-actix_web, time:{}", time.to_rfc3339())
}

#[actix_web::main]
async fn main() -> Result<()> {
    let bind_addr = "0.0.0.0:2001";

    let r = HttpServer::new(|| App::new().service(index_root))
        .bind(bind_addr)?
        .run();

    println!("bind to {}", bind_addr);
    r.await
}
