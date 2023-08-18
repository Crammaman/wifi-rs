use crate::platforms::{Config, WifiError, WifiInterface};
use std::process::Command;
use regex::Regex;

#[derive(Debug)]
pub struct Connection {
    pub(crate) ssid: String,
}

/// Wireless network interface for linux operating system.
#[derive(Debug)]
pub struct Linux {
    pub(crate) connection: Option<Connection>,
    pub(crate) interface: String,
}

impl Linux {
    pub fn new(config: Option<Config>) -> Self {
        Linux {
            connection: None,
            interface: config.map_or("wlan0".to_string(), |cfg| {
                cfg.interface.unwrap_or("wlan0").to_string()
            }),
        }
    }
    pub fn available_ssids() -> Result<Vec<String>, WifiError> {
        let o = Command::new("nmcli")
            .args(&["-t","dev","wifi","list"])
            .output()
            .map_err(|err| WifiError::IoError(err));
        
        let text = String::from_utf8_lossy(&o.unwrap().stdout).to_string();
        let lines = text.split("\n");
        let mut configs: Vec<String> = vec![];
        
        let re = Regex::new(r"^([^:]*):(.*):([^:]*):([^:]*):([^:]*):([^:]*):([^:]*):([^:]*):([^:]*):([^:]*)$").unwrap();
        for line in lines {
            let result = re.captures(line);
            // Skip tailing line and any other un expected lines
            if result.is_none() { continue }
            let fields = result.unwrap();

            configs.push(fields[4].to_string());
        }
        Ok(configs)
    }
}

/// Wifi interface for linux operating system.
/// This provides basic functionalities for wifi interface.
impl WifiInterface for Linux {
    /// Check if wireless network adapter is enabled.
    fn is_wifi_enabled() -> Result<bool, WifiError> {
        let output = Command::new("nmcli")
            .args(&["radio", "wifi"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .replace(" ", "")
            .replace("\n", "")
            .contains("enabled"))
    }

    /// Turn on the wireless network adapter.
    fn turn_on() -> Result<(), WifiError> {
        Command::new("nmcli")
            .args(&["radio", "wifi", "on"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }

    /// Turn off the wireless network adapter.
    fn turn_off() -> Result<(), WifiError> {
        Command::new("nmcli")
            .args(&["radio", "wifi", "off"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }
}
