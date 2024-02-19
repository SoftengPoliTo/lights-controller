use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use clap::builder::TypedValueParser as _;
use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum LightLocation {
    Laundry = 0,
    Bathroom,
    Hall,
    LivingRoom,
    SittingRoom,
    DiningTable,
    KitchenIsland,
    Kitchen,
    ParentBathroom,
    ParentBedroom,
    ParentBed,
}

impl FromStr for LightLocation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Laundry" => Ok(Self::Laundry),
            "Bathroom" => Ok(Self::Bathroom),
            "Hall" => Ok(Self::Hall),
            "LivingRoom" => Ok(Self::LivingRoom),
            "SittingRoom" => Ok(Self::SittingRoom),
            "DiningTable" => Ok(Self::DiningTable),
            "KitchenIsland" => Ok(Self::KitchenIsland),
            "Kitchen" => Ok(Self::Kitchen),
            "ParentBathroom" => Ok(Self::ParentBathroom),
            "ParentBedroom" => Ok(Self::ParentBedroom),
            "ParentBed" => Ok(Self::ParentBed),
            _ => Err(format!("Unknown room: {s}")),
        }
    }
}

impl fmt::Display for LightLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Laundry => "Laundry",
            Self::Bathroom => "Bathroom",
            Self::Hall => "Hall",
            Self::LivingRoom => "LivingRoom",
            Self::SittingRoom => "SittingRoom",
            Self::DiningTable => "DiningTable",
            Self::KitchenIsland => "KitchenIsland",
            Self::Kitchen => "Kitchen",
            Self::ParentBathroom => "ParentBathroom",
            Self::ParentBedroom => "ParentBedroom",
            Self::ParentBed => "ParentBed",
        };
        s.fmt(f)
    }
}

impl LightLocation {
    const fn all() -> &'static [&'static str] {
        &[
            "Laundry",
            "Bathroom",
            "Hall",
            "LivingRoom",
            "SittingRoom",
            "DiningTable",
            "KitchenIsland",
            "Kitchen",
            "ParentBathroom",
            "ParentBedroom",
            "ParentBed",
        ]
    }
}

#[derive(Parser, Debug, Clone)]
#[repr(u8)]
pub(crate) enum Action {
    Pulse = 0,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Pulse" => Ok(Self::Pulse),
            _ => Err(format!("Unknown light action: {s}")),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Pulse => "Pulse",
        };
        s.fmt(f)
    }
}

impl Action {
    const fn all() -> &'static [&'static str] {
        &["Pulse"]
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "lights-controller",
    version,
    author,
    about = "Controls the lights in an house."
)]
pub(crate) struct Options {
    /// Address of the lights firmware on the local network
    /// e.g. `192.168.1.42:23`.
    /// This option overwrites the value read from the configuration file.
    #[arg(short, long)]
    pub(crate) address: Option<SocketAddr>,

    /// Light to control.
    #[arg(long, short = 's', default_value_t = LightLocation::LivingRoom,
        value_parser = clap::builder::PossibleValuesParser::new(LightLocation::all())
            .map(|s| s.parse::<LightLocation>().unwrap()),)]
    pub(crate) light_location: LightLocation,

    /// Type of signal/event to send on the light.
    #[arg(long, short = 'x', default_value_t = Action::Pulse,
        value_parser = clap::builder::PossibleValuesParser::new(Action::all())
            .map(|s| s.parse::<Action>().unwrap()),)]
    pub(crate) action: Action,

    /// Configuration path.
    #[arg(short = 'c', long)]
    pub(crate) config_path: Option<PathBuf>,

    /// Port of the Thing.
    /// This option overwrites the value read from the configuration file.
    #[arg(short = 'p', long)]
    pub(crate) thing_port: Option<u16>,
}
