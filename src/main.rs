use std::process::Command;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Patio Pi");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(off)
            .service(on)
            .service(leds)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/leds")]
async fn leds(_req_body: String) -> impl Responder {
    let mut pixleds = Command::new("./rpi_pixleds");
    pixleds.arg("-n");
    pixleds.arg("80");
    pixleds.arg("ffffff");
    pixleds.output().expect("Failed to invoke rpi_pixleds");

    HttpResponse::Ok()
}

#[post("/off")]
async fn off(_req_body: String) -> impl Responder {
    let mut pixleds = Command::new("./rpi_pixleds");
    pixleds.arg("-n");
    pixleds.arg("80");
    for n in 1..80 {
        pixleds.arg("000000");
    }
    pixleds.output().expect("Failed to invoke rpi_pixleds");

    HttpResponse::Ok()
}

#[post("/on")]
async fn on(_req_body: String) -> impl Responder {
    let mut pixleds = Command::new("./rpi_pixleds");
    pixleds.arg("-n");
    pixleds.arg("80");
    for n in 1..80 {
        pixleds.arg("ffffff");
    }
    pixleds.output().expect("Failed to invoke rpi_pixleds");

    HttpResponse::Ok()
}