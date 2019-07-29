use bincode;
use failure::Error;
use futures::stream::Stream;
use mqtt::{control::variable_header::ConnectReturnCode, packet::*, Decodable, Encodable};
use tokio::{runtime, timer::Interval};
use uuid::Uuid;
use clap::{Arg, App as CliApp};

use std::{io::Write, net::TcpStream, time::Duration};

use pirust_common::Weight;

fn generate_client_id() -> String {
    format!("/MQTT/rust/{}", Uuid::new_v4())
}

fn main() -> Result<(), Error> {
    let matches = CliApp::new("Pirust")
        .version("0.1.0")
        .arg(Arg::with_name("host")
          .required(true)
          .takes_value(true)
          .index(1)
          .help("Host name or IP of the MQTT Server (without port)")
        )
        .arg(Arg::with_name("port")
          .required(false)
          .takes_value(true)
          .index(2)
          .help("Port of the MQTT Server")
        )
        .get_matches();

    let server_addr = matches.value_of("host").unwrap().to_string();
    let server_port = matches.value_of("port").unwrap_or("1883").to_string();
    let server_host = [server_addr, ":".to_string(), server_port].concat();

    println!("Connecting to {:?} ... ", server_host);
    let mut stream = TcpStream::connect(server_host).unwrap();
    println!("Connected!");

    let client_id = generate_client_id();
    println!("Client identifier {:?}", client_id);
    let mut conn = ConnectPacket::new("MQTT", client_id);
    conn.set_clean_session(true);
    let mut buf = Vec::new();
    conn.encode(&mut buf).unwrap();
    stream.write_all(&buf[..]).unwrap();

    let connack = ConnackPacket::decode(&mut stream).unwrap();
    println!("CONNACK {:?}", connack);

    if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
        panic!(
            "Failed to connect to server, return code {:?}",
            connack.connect_return_code()
        );
    }

    let topic_name = mqtt::TopicName::new("pirust_weight").expect("Expected a new topic name");

    let future = Interval::new_interval(Duration::new(1, 0))
        .map_err(std::convert::Into::<Error>::into)
        .for_each(move |_| {
            let weight = Weight(11.2);
            println!("Publishing weight to server: {:?}", weight);

            let publish_packet = PublishPacket::new(
                topic_name.clone(),
                QoSWithPacketIdentifier::Level0,
                bincode::serialize(&weight)?,
            );
            let mut buf = Vec::new();
            publish_packet.encode(&mut buf).unwrap();
            stream.write_all(&buf[..]).unwrap();
            Ok(())
        });

    let mut runtime = runtime::Builder::new().build()?;
    runtime.block_on(future)
}
