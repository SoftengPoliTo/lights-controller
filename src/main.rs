mod command;
mod config;
mod thing;

use clap::Parser;

use crate::command::Options;
use crate::config::Configuration;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let options = Options::parse();

    let configuration = if let Some(config_path) = options.config_path {
        Configuration::load(config_path).expect("Failed retrieving configuration file")
    } else {
        Configuration::default()
    };

    let address = options.address.unwrap_or(configuration.address);

    thing::run(
        address,
        options.thing_port.unwrap_or(configuration.thing_port),
    )
    .await
}
