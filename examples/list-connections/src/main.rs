use std::io;
use wifi_rs::{prelude::*, WiFi};

fn main() -> Result<(), io::Error> {
    let configs = WiFi::available_ssids();
    println!("Result {:?}",configs);
    Ok(())
}
