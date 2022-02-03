use std::process::Command;
use std::format;
use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::{thread, time};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Patio Pi");

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 60, |x, y| {
        if (x+y) % 3 == 0 {
            image::Rgb([85u8,0u8,85u8])
        } else {
            image::Rgb([0u8,0u8,0u8])
        }
    });

    let data = web::Data::new(img);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(off)
            .service(on)
            .service(leds)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/leds")]
async fn leds(data: web::Data<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> impl Responder {
    for r in 0..59 {
        let mut pixleds = Command::new("./rpi_pixleds");
        pixleds.arg("-n");
        pixleds.arg("61");
        let mut pixels = String::new();
        for n in 0..60 {
            let pixel = *data.get_pixel(n, r);
            pixels = pixels + &*format!("{:02X}", pixel[0]);
            pixels = pixels + &*format!("{:02X}", pixel[1]);
            pixels = pixels + &*format!("{:02X}", pixel[2]);
            pixels = pixels + ",";
        }
        pixleds.arg(&pixels);
        pixleds.arg(&pixels);
        pixleds.output().expect("Failed to invoke rpi_pixleds");

        let delay = time::Duration::from_millis(20);
        thread::sleep(delay);
    }

    HttpResponse::Ok()
}

#[get("/off")]
async fn off(_req_body: String) -> impl Responder {
    let mut pixleds = Command::new("./rpi_pixleds");
    pixleds.arg("-n");
    pixleds.arg("61");
    let pixels = std::iter::repeat("000000,").take(80).collect::<String>();
    pixleds.arg(&pixels);
    pixleds.arg(&pixels);

    pixleds.output().expect("Failed to invoke rpi_pixleds");

    HttpResponse::Ok().body(format!("{:#?}", pixleds))
}

#[get("/on")]
async fn on(_req_body: String) -> impl Responder {
    let mut pixleds = Command::new("./rpi_pixleds");
    pixleds.arg("-n");
    pixleds.arg("61");
    let pixels = std::iter::repeat("555555,").take(80).collect::<String>();
    pixleds.arg(&pixels);
    pixleds.arg(&pixels);

    pixleds.output().expect("Failed to invoke rpi_pixleds");

    HttpResponse::Ok().body(format!("{:#?}", pixleds))
}

// RHS: 61 LEDs from right end
// LHS: Unknown, not working well

// TODO: Playback from buffer, in a thread
// TODO: scan a pixel, animated
// TODO: read a PNG