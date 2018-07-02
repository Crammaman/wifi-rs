mod handlers;
pub mod profile_network;
mod providers;
mod stubs;

use std::{fmt, io};

pub trait Network: fmt::Debug {
    /// Makes an attempt to connect to a selected wireless network with password specified.
    fn connect(&self, password: &str) -> Result<bool, NetworkError>;
    fn disconnect(&self) -> Result<bool, NetworkError>;
}

// #[derive(Debug)]
// pub enum NetworkType {
//     WEP,
//     WPA,
//     WPA2,
// }

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub interface: Option<&'a str>,
}

#[derive(Debug)]
pub enum NetworkError {
    // FromUtf8Error(FromUtf8Error),
    SsidNotFound,
    OsNotSupported,
    IpAssignFailed,
    AddNetworkProfileFailed,
    IoError(io::Error),
    FailedToConnect(String),
    FailedToDisconnect(String),
}

impl From<io::Error> for NetworkError {
    fn from(error: io::Error) -> Self {
        NetworkError::IoError(error)
    }
}
