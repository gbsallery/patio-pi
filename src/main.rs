use std::process::Command;

fn main() {
    println!("Patio Pi");

    let mut pixleds = Command::new("./rpi_pixleds");
    pixleds.arg("-n");
    pixleds.arg("80");
    pixleds.arg("ffffff");
    pixleds.output().expect("Failed to invoke rpi_pixleds");
}
