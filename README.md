# Lights

This program aims at controlling the lights in an house, by talking
to the [lights-firmware](https://github.com/SoftengPoliTo/lights-firmware)
program which runs on `ESP32-C3`

## Building

```console
cargo build --release
```

## Running

```console
cargo run
```

## Usage

Use `-h`/`--help` to get help:

```
Controls the lights in an house.

Usage: lights-controller [OPTIONS] --config-path <CONFIG_PATH>

Options:
  -a, --address <ADDRESS>
          Address of the lights firmware on the local network e.g. `192.168.1.42:23`. This option overwrites the value read from the configuration file
  -s, --light-location <LIGHT_LOCATION>
          Light to control [default: LivingRoom] [possible values: Laundry, Bathroom, Hall, LivingRoom, SittingRoom, DiningTable, KitchenIsland, Kitchen, ParentBathroom, ParentBedroom, ParentBed]
  -x, --action <ACTION>
          Type of signal/event to send on the light [default: Pulse] [possible values: Pulse]
  -c, --config-path <CONFIG_PATH>
          Configuration path
  -p, --thing-port <THING_PORT>
          Port of the Thing. This option overwrites the value read from the configuration file
  -h, --help
          Print help
  -V, --version
          Print version
```

Use the `--address` option to specify the address, and the `--light-location`
option to specify the group of lights to control. The `--action`
option defaults to `pulse`, which is also the only possible value.

A configuration file can be used to read the value of the `--address`
option and the `--thing-port` value.

## Basic usage

To turn the group of lights in a room:

```console
lights-controller -a 192.168.1.125:23 -s livingroom
```

## Web of Things

[Web of Things](https://www.w3.org/TR/wot-architecture/) standard turns each
light present in a Smart Home into a Thing: an abstraction of a physical or
virtual entity (e.g., a device or a room) and is described by standardized
metadata. `lights-controller` is a WoT Server to control and manage the
lights. The `--thing-port` option allows to set the server port used to listen
on the incoming connections. The default server port is `8888`.


Below a series of examples to perform actions and retrieve data from Things.

To visualize **Things Descriptions** from browser, run the server

```console
lights-controller
```

and go to the `127.0.0.1:8888` address. From there, you can download the `JSON`
**Things Descriptions**.

To download **Things Descriptions** with `Curl` and format the output `JSON` 
through `jq`, run:

```console
curl -X GET 127.0.0.1:8888 | jq
```

To download the **Laundry** lights `properties`, represented by the `0`
identifier, run:

```console
curl -X GET 127.0.0.1:8888/0/properties | jq
```

To turn on the **Laundry** lights, run the following `PUT` REST API to change
the `pulse` property. You must set the
`Accept: application/json` and `Content-Type: application/json` headers.

```console
curl -X PUT http://127.0.0.1:8888/0/properties/pulse -H "Accept: application/json" -H 'Content-Type: application/json' -d '{"pulse":true}'
```

This command runs on the server the `ValueForwarder` method which prints:

```
Starting the Things server (port 8888)…
Sending a Pulse to Laundry (value `true`, address `127.0.0.1:23`)…
```

### WebThings Gateway

Once the WebThings Server is running, use the [WebThings
Gateway](https://webthings.io/gateway/) to easily interact with your
lights through a simple and straightforward Web interface.

Follow these [instructions](./webthings-gateway.md) to setup, run, and use the
gateway.
