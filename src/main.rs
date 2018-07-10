mod connectivity;
pub use connectivity::{profile_network::ProfileNetwork as WiFi, Config, WifiConnectionError};

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let wifi = WiFi::new("AndroidAPSD22", config)?;

    match wifi.connect("belm4235") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successfull."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    Ok(())
}
