#![feature(proc_macro_hygiene, decl_macro)]
#![feature(mutex_unlock)]

#[macro_use]
extern crate rocket;

use std::process::Command;
use std::format;
use image::{ImageBuffer, Rgb, RgbImage};
use std::{thread, time};
use std::sync::{Mutex, Arc};
use rocket::{Data, State};
use rocket::config::{Config, Environment};
use rocket::http::{RawStr, Status};

// RHS: 61 LEDs from right end
// LHS: Unknown, not working well (electrical issue)

#[get("/")]
fn world() -> &'static str {
    "Patio Pi server up and running"
}

#[get("/off")]
fn off(animation: State<Arc<Mutex<Animation>>>) -> &'static str {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 1, |_x, _y| {
        image::Rgb([0u8, 0u8, 0u8])
    });
    update_animation(animation, img);
    "Off"
}

#[get("/on")]
fn on(animation: State<Arc<Mutex<Animation>>>) -> &'static str {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 1, |_x, _y| {
        image::Rgb([0xffu8, 0xffu8, 0xffu8])
    });
    update_animation(animation, img);
    "On"
}

#[get("/solid?<rgb>")]
fn solid(rgb: &RawStr, animation: State<Arc<Mutex<Animation>>>) -> Status {
    let r = u8::from_str_radix(&rgb[0..2], 16).unwrap();
    let g = u8::from_str_radix(&rgb[2..4], 16).unwrap();
    let b = u8::from_str_radix(&rgb[4..6], 16).unwrap();
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 1, |_x, _y| {
        image::Rgb([r, g, b])
    });
    update_animation(animation, img);
    Status::Ok
}

#[get("/leds")]
fn leds(animation: State<Arc<Mutex<Animation>>>) -> &'static str {
    let img = image::open("test.png").unwrap().to_rgb8();

    update_animation(animation, img);
    "Flash!"
}

#[post("/image", format = "any", data = "<data>")]
fn image(data: Data, animation: State<Arc<Mutex<Animation>>>) -> &'static str {
    data.stream_to_file("/tmp/upload.png").map(|n| n.to_string()).unwrap();

    let img = image::open("/tmp/upload.png").unwrap().to_rgb8();
    update_animation(animation, img);
    "Uploaded"
}

fn update_animation(animation: State<Arc<Mutex<Animation>>>, img: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut guard = animation.lock().unwrap();
    guard.img = img;
    guard.updated = true;
}

const INTENSITY_LIMIT: u32 = 61 * 255 * 3 / 4;

fn animate(animation: Arc<Mutex<Animation>>) {
    'animator: loop {
        let image = check_new_image(&animation);
        let (width, height) = image.dimensions();
        for y in 0..height {
            let guard = animation.lock().expect("Failed to acquire lock on animation");
            if guard.updated { continue 'animator; };
            Mutex::unlock(guard);

            let mut pixleds = Command::new("./rpi_pixleds");
            pixleds.arg("-n");
            pixleds.arg("61");
            let mut pixels = String::new();
            for x in (0..width).rev() {
                let pixel = image.get_pixel(x, y);
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
}

fn check_new_image(animation: &Arc<Mutex<Animation>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut guard = animation.lock().expect("Failed to acquire lock on animation");
    let mut new_frame = Default::default();
    if guard.updated {
        let mut peak_intensity: u32 = 0;
        let frame = &guard.img;
        let (width, height) = frame.dimensions();
        for y in 0..height {
            let mut intensity: u32 = 0;
            for x in 0..width {
                let pixel = frame.get_pixel(x, y);
                intensity = intensity + pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32;
            }
            if intensity > peak_intensity { peak_intensity = intensity }
        }
        new_frame =
            if peak_intensity > INTENSITY_LIMIT {
                let ratio = (INTENSITY_LIMIT as f64) / (peak_intensity as f64);
                println!("Scaling intensity down to {}", ratio);
                ImageBuffer::from_fn(width, height, |x, y| {
                    let p = frame.get_pixel(x, y);
                    image::Rgb([(p[0] as f64 * ratio) as u8, (p[1] as f64 * ratio) as u8, (p[2] as f64 * ratio) as u8])
                })
            } else {
                frame.clone()
            };
        guard.updated = false;
    }
    new_frame
}

#[derive(Clone)]
// struct Animation(Arc<Mutex<RgbImage>>);
struct Animation {
    img: RgbImage,
    updated: bool,
}

fn main() {
    println!("Patio Pi");

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(61, 1, |_x, _y| {
        image::Rgb([0u8, 0u8, 0u8])
    });
    // let animation = Animation(Arc::new(Mutex::new(img)));
    let animation = Arc::new(Mutex::new(Animation { img, updated: false }));

    let animation1 = animation.clone();
    std::thread::spawn(move || {
        animate(animation1)
    });

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .finalize().unwrap();

    rocket::custom(config)
        .mount("/", routes![world,on,off,solid,leds,image])
        .manage(animation)
        .launch();
}

// TODO: Fade between states
// TODO: scan a pixel, animated

/*
TODO move patterns to Neo:
Dawn sunrise
Opening up 8am
Weather during day
Sunset animation
Random patterns until midnight
Meater mode
*/