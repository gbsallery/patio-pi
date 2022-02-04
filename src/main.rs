#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::process::Command;
use std::format;
use image::{ImageBuffer, Rgb, RgbImage};
use std::{thread, time};
use rocket::Data;
use rocket::config::{Config, Environment};
use rocket::http::RawStr;


#[get("/")]
fn world() -> &'static str {
    "Patio Pi server up and running"
}

#[get("/off")]
fn off() -> &'static str {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 1, |_x, _y| {
            image::Rgb([0u8,0u8,0u8])
    });
    animate(img);
    "Off"
}

#[get("/on")]
fn on() -> &'static str {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 1, |_x, _y| {
            image::Rgb([0x55u8,0x55u8,0u8])
    });
    animate(img);
    "On"
}

#[get("/solid?<rgb>")]
fn solid(rgb: &RawStr) -> &str {
    let r= u8::from_str_radix(&rgb[0..2], 16).unwrap();
    let g = u8::from_str_radix(&rgb[2..4], 16).unwrap();
    let b = u8::from_str_radix(&rgb[4..6], 16).unwrap();
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 1, |_x, _y| {
            image::Rgb([r,g,b])
    });
    animate(img);
    rgb
}

#[get("/leds")]
fn leds() -> &'static str {
    let img = image::open("test.png").unwrap().to_rgb8();

    animate(img);
    "Flash!"
}

#[post("/image", format = "any", data = "<data>")]
fn image(data: Data) -> &'static str {
    data.stream_to_file("/tmp/upload.png").map(|n| n.to_string()).unwrap();

    let img = image::open("/tmp/upload.png").unwrap().to_rgb8();
    animate(img);
    "Uploaded"
}

fn animate(img: RgbImage) {
    let (width, height) = img.dimensions();
    for r in 0..height {
        let mut pixleds = Command::new("./rpi_pixleds");
        pixleds.arg("-n");
        pixleds.arg("61");
        let mut pixels = String::new();
        for n in 0..width {
            let pixel = img.get_pixel(n, r);
            pixels = pixels + &*format!("{:02X}", pixel[0]);
            pixels = pixels + &*format!("{:02X}", pixel[1]);
            pixels = pixels + &*format!("{:02X}", pixel[2]);
            pixels = pixels + ",";
        }
        pixleds.arg(&pixels);
        pixleds.output().expect("Failed to invoke rpi_pixleds");

        let delay = time::Duration::from_millis(20);
        thread::sleep(delay);
    }
}

fn main() {
    println!("Patio Pi");

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .finalize().unwrap();

    rocket::custom(config).mount("/", routes![world,on,off,solid,leds,image]).launch();
}

// RHS: 61 LEDs from right end
// LHS: Unknown, not working well

// TODO: Playback from buffer, in a thread
// TODO: scan a pixel, animated
// TODO: Intensity clamp
// TODO: Weather pattern
// TODO: Progress bar API
// TODO: Meater mode