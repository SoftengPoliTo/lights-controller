use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Configuration {
    pub(crate) address: SocketAddr,
    pub(crate) thing_port: u16,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 23),
            thing_port: 8888,
        }
    }
}

impl Configuration {
    pub(crate) fn load(path: impl AsRef<Path>) -> Result<Configuration, confy::ConfyError> {
        confy::load_path(path)
    }
}
