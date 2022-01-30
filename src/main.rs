use std::process::Command;

fn main() {
    println!("Patio Pi");

    let mut pixleds = Command::new("../../rpi_pixleds");
    pixleds.arg("-n 90");
    pixleds.arg("ffffff");
    pixleds.spawn().expect("Failed to invoke rpi_pixleds");
}
