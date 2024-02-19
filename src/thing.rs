use std::{
    marker::{Send, Sync},
    net::ToSocketAddrs,
    sync::{Arc, RwLock, Weak},
};

use serde_json::{json, Value};
use webthing::{
    property::ValueForwarder, server::ActionGenerator, Action as ThingAction, BaseProperty,
    BaseThing, Thing, ThingsType, WebThingServer,
};

use crate::command::{Action, LightLocation};

// Structure used to generate new Actions
struct Generator;

// Trait to generate a new Action, based on its name
impl ActionGenerator for Generator {
    fn generate(
        &self,
        _thing: Weak<RwLock<Box<dyn Thing>>>, // Thing associated to this Action
        _name: String,                        // Name of the requested Action
        _input: Option<&Value>,               // Input for the Action
    ) -> Option<Box<dyn ThingAction>> {
        None
    }
}

struct OnPulseValueForwarder<A>
where
    A: ToSocketAddrs + std::fmt::Debug + Copy + Clone + Send + Sync,
{
    address: A,
    light_location: LightLocation,
}

// Forward a new property value to the physical/virtual device.
impl<A> ValueForwarder for OnPulseValueForwarder<A>
where
    A: ToSocketAddrs + std::fmt::Debug + Copy + Clone + Send + Sync,
{
    fn set_value(&mut self, value: Value) -> Result<Value, &'static str> {
        println!(
            "Sending a {:?} to {:?} (value `{}`, address `{:?}`)…",
            Action::Pulse,
            self.light_location,
            value,
            self.address,
        );

        Ok(value)
    }
}

fn make_light<A>(address: A, light_location: LightLocation) -> Arc<RwLock<Box<dyn Thing + 'static>>>
where
    A: 'static + ToSocketAddrs + std::fmt::Debug + Copy + Clone + Send + Sync,
{
    // Create a new Thing.
    let mut thing = BaseThing::new(
        format!("urn:dev:ops:light-{}", light_location as u8), // Thing unique ID, it must be a URI
        light_location.to_string(),                            // Thing title
        Some(vec!["Light".to_owned()]),                        // Thing type
        None,                                                  // Thing description
    );

    // Property metadata pulse property
    let pulse_description = json!({
        "@type": "OnOffProperty",
        "title": "On/Off",
        "type": "boolean",
        "description": "Whether the lamp is turned on"
    })
    .as_object()
    .unwrap()
    .clone();

    // Add a new property to the Thing.
    // This represents an individual state value of a Thing.
    thing.add_property(Box::new(BaseProperty::new(
        "pulse".to_owned(), // Property name
        json!(false),       // Initial property value
        Some(Box::new(OnPulseValueForwarder {
            address,
            light_location,
        })), // Optional value forwarder. Property is read-only if and only if this value is None
        Some(pulse_description),
    )));

    Arc::new(RwLock::new(Box::new(thing)))
}

pub(crate) async fn run<A>(address: A, port: u16) -> Result<(), std::io::Error>
where
    A: 'static + ToSocketAddrs + std::fmt::Debug + Copy + Clone + Send + Sync,
{
    // Create Things which represent the lights present in each house location
    let things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = vec![
        make_light(address, LightLocation::Laundry),
        make_light(address, LightLocation::Bathroom),
        make_light(address, LightLocation::Hall),
        make_light(address, LightLocation::LivingRoom),
        make_light(address, LightLocation::SittingRoom),
        make_light(address, LightLocation::DiningTable),
        make_light(address, LightLocation::KitchenIsland),
        make_light(address, LightLocation::Kitchen),
        make_light(address, LightLocation::ParentBathroom),
        make_light(address, LightLocation::ParentBedroom),
        make_light(address, LightLocation::ParentBed),
    ];

    println!("Starting the Things server (port {port})…",);

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Lights".to_string()), //  List of Things managed by this
        //  server with the relative group name
        Some(port),          // Port to listen on incoming connections
        None,                // Optional hostname
        None,                // Tuple of SSL options to pass to the actix web server
        Box::new(Generator), // Structure which generates an Action
        None,                // Base URL to use, in this case is `/`
        None,                // Enable host validation and use default configuration
    );
    // Start listening for incoming connections.
    // No configuration has been provided.
    server.start(None).await
}
