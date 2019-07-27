# pirust

## Prerequisites
- Rust (with armv7-unknown-linux-musleabihf target)
- LLVM
- arm-linux-gnueabihf-binutils

## Building commands

### Building publisher
```bash
cargo build --bin pirust_pub_weight --target armv7-unknown-linux-musleabihf
```

### Running publisher locally
```bash
cargo run --bin pirust_pub_weight
```

### Running subscriber locally
```bash
cargo run --bin pirust_sub_weight
```

## Setting up MacOS environment

### Installing rust on MacOS
```bash
curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path 
```

### Adding a new target (for compiling armv7 binaries)
```bash
xcode-select --install
brew install arm-linux-gnueabihf-binutils llvm rsync
rustup target add armv7-unknown-linux-musleabihf
```

## MQTT stuff
We use  [mosquitto](https://mosquitto.org) as a MQTT server.

### Install mosquitto

(Skip if you don't need mosquitto CLI tools and want to run it just in Docker.)

```bash
brew install mosquitto
```

### Run MQTT broker in docker container
```bash
 docker run -ti -p 1883:1883 -p 91:9001 toke/mosquitto
```

### Publish message example
```bash
mosquitto_pub  -h ip_or_hostname -p 1883 -m "test message" -t roomId/topicId
```

### Listen for specific topic
```bash
mosquitto_sub  -h ip_or_hostname -p 1883 -t roomId/topicId
```
