extern crate amqp;

use std::io::{self, Read};
use std::thread;
use std::time::Duration;

use amqp::conf::Conf;
use amqp::frame::Frame;
use amqp::protocol::*;
use amqp::transport::Transport;
use amqp::types;
use amqp::types::*;
use amqp::types::FieldValue::*;

fn main() {
    let mut tx = Transport::new(&Conf::default());
    tx.send_header().unwrap();

    let frame = tx.recv().unwrap();
    let start: connection::Start = frame.to_method().unwrap();
    println!("{:#?}", start);
    if let FieldValue::LongString(ref s) = start.server_properties["copyright"] {
        println!("{}", types::to_string(s.clone()));
    }
    if let FieldValue::LongString(ref s) = start.server_properties["version"] {
        println!("{}", types::to_string(s.clone()));
    }
    if let FieldValue::LongString(ref s) = start.server_properties["cluster_name"] {
        println!("{}", types::to_string(s.clone()));
    }
    if let FieldValue::LongString(ref s) = start.server_properties["product"] {
        println!("{}", types::to_string(s.clone()));
    }
    if let FieldValue::LongString(ref s) = start.server_properties["information"] {
        println!("{}", types::to_string(s.clone()));
    }
    if let FieldValue::LongString(ref s) = start.server_properties["platform"] {
        println!("{}", types::to_string(s.clone()));
    }
    println!("{}", types::to_string(start.mechanisms));
    println!("{}", types::to_string(start.locales));

    let mut client_properties = Table::new();
    client_properties.insert("product".to_string(), LongString(b"rust-amqp".to_vec()));
    client_properties.insert("platform".to_string(), LongString(b"rust".to_vec()));
    client_properties.insert("version".to_string(), LongString(b"0.0.1".to_vec()));
    client_properties.insert("information".to_string(),
                             LongString(b"https://github.com/Antti/rust-amqp".to_vec()));

    let mut capabilities = Table::new();
    capabilities.insert("publisher_confirms".to_string(), Bool(true));
    capabilities.insert("consumer_cancel_notify".to_string(), Bool(true));
    capabilities.insert("exchange_exchange_bindings".to_string(), Bool(true));
    capabilities.insert("basic.nack".to_string(), Bool(true));
    capabilities.insert("connection.blocked".to_string(), Bool(true));
    capabilities.insert("authentication_failure_close".to_string(), Bool(true));
    client_properties.insert("capabilities".to_string(), FieldTable(capabilities));

    let mut start_ok = connection::StartOk::default();
    start_ok.client_properties = client_properties;
    start_ok.mechanism = "PLAIN".to_string();
    start_ok.response = b"\0mq\0mq".to_vec();
    start_ok.locale = "en_US".to_string();
    tx.send(Frame::from_method(0, &start_ok).unwrap()).unwrap();

    let frame = tx.recv().unwrap();
    let tune: connection::Tune = frame.to_method().unwrap();
    println!("{:#?}", tune);

    let mut tune_ok = connection::TuneOk::default();
    tune_ok.channel_max = tune.channel_max;
    tune_ok.frame_max = tune.frame_max;
    tune_ok.heartbeat = tune.heartbeat;
    tx.send(Frame::from_method(0, &tune_ok).unwrap()).unwrap();

    let open = connection::Open::default();
    tx.send(Frame::from_method(0, &open).unwrap()).unwrap();

    let frame = tx.recv().unwrap();
    let open_ok: connection::OpenOk = frame.to_method().unwrap();
    println!("{:#?}", open_ok);

    //thread::sleep(Duration::from_secs(3));
    io::stdin().read_to_string(&mut String::new());

    let mut close = connection::Close::default();
    close.reply_code = REPLY_SUCCESS as u16;
    close.reply_text = "OK".to_string();
    tx.send(Frame::from_method(0, &close).unwrap()).unwrap();

    let frame = tx.recv().unwrap();
    let close_ok: connection::CloseOk = frame.to_method().unwrap();
    println!("{:#?}", close_ok);

    tx.close().unwrap();
}
