# WebThings Gateway

 It provides a Web UI interface to better interact with the **Things** present
 in your smart home. Through this gateway, you can change **Thing
 Properties** states, run its **Actions**, and intercept its **Events**.
 Both Things and WebThings Gateway should be connected to the same network in
 order to communicate with each other.

# Running the WebThings Gateway

This guide assumes you are on a Unix-like system.

It would be recommended to use the `Docker` image generated from the official
source code in order to be in line with the latest changes. Below the steps to
run the gateway:

```console
git clone https://github.com/WebThingsIO/gateway
cd gateway
docker build -t gateway .
docker run \
    -d \
    -e TZ=America/Los_Angeles \
    -v /path/to/shared/data:/home/node/.webthings \
    --network="host" \
    --log-opt max-size=1m \
    --log-opt max-file=10 \
    --name webthings-gateway \
    gateway
```

If you want to use a stable gateway version instead, you can directly run the
`Docker` image present on [DockerHub](https://hub.docker.com/r/webthingsio/gateway).
In this case, the only step is:

```console
docker run \
    -d \
    -e TZ=America/Los_Angeles \
    -v /path/to/shared/data:/home/node/.webthings \
    --network="host" \
    --log-opt max-size=1m \
    --log-opt max-file=10 \
    --name webthings-gateway \
    webthingsio/gateway:latest
```

To understand the meaning of each parameter, have a look at the **Parameter**
section on this [page](https://hub.docker.com/r/webthingsio/gateway).

Change the `TZ` parameter according to your timezone.

The `path/to/shared/data` directory can be created anywhere on your system and
it will contain every artifact shared with the image volume, such as your
personal gateway account data, the logging file presenting a list of every
operation you have performed, and many other information.

Unfortunately, running the gateway results in an immediate failure caused by
creation of the `sqlite3` internal database. The workaround for this issue
consists of adding a `.node_version` file to the `path/to/shared/data`
containing just the number `12`. Once this operation has been done, the
`docker run` command works as expected.

To stop and remove the `Docker` container, run the
`docker container remove --force CONTAINER_ID` command.

# Usage

A simple use case which shows how system components cooperate with each other.

On one terminal, run the `lights-controller` WebThings Server with:

```console
cargo run
```

To visualize the `Json` file containing all Things Descriptions from your
browser, go to `localhost:8888` address.

On another terminal, run the `gateway` using the `docker run` command explained
before. You can access to the gateway from your browser going to the
`localhost:8080` address.

Follow these [instructions](https://webthings.io/docs/gateway/setup/) to setup
the gateway.

To scan all available Things present in your network, you must add the
`Web Thing` add-on as explained
[here](https://webthings.io/docs/gateway/settings/#add-ons).

Once the `Web Thing` add-on has been enabled, you can manage your Things
following this [guide](https://webthings.io/docs/gateway/things/).
