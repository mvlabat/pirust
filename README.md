# pirust

## Building client
```bash
cargo build --bin pirust_client --target armv7-unknown-linux-musleabihf
```

## Running server
```bash
cargo run --bin pirust_server
```

## Installing rust on MacOS
```bash
curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path 
```

## Adding a new target (to compile program for that target)
```bash
xcode-select --install
brew install arm-linux-gnueabihf-binutils llvm rsync
rustup target add armv7-unknown-linux-musleabihf
```

## MQTT stuff
We are using wide used set of tools [https://mosquitto.org](mosquitto)

**Install mosquitto**
```bash
brew install mosquitto
```

**Run MQTT broker in docker container**
```bash
 docker run -ti -p 1883:1883 -p 91:9001 toke/mosquitto
```
**Publish message example**
```bash
mosquitto_pub  -h ip_or_hostname -p 1883 -m "test message" -t roomId/topicId
```
**Listen for specific topic**
```bash
mosquitto_sub  -h ip_or_hostname -p 1883 -t roomId/topicId
```
