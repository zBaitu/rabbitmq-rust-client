use conf::Conf;
use frame::Frame;
use protocol::{self, connection};
use result::*;
use transport::Transport;
use types::*;
use types::FieldValue::*;

pub struct Connection {
    conf: Conf,
    tx: Transport,

    pub server_props: Table,
}

impl Connection {
    pub fn new(conf: Conf) -> Connection {
        let tx = Transport::new(&conf);
        Connection {
            conf: conf,
            tx: tx,

            server_props: Table::new(),
        }
    }

    pub fn start(&mut self) -> AmqpResult<()> {
        try!(self.tx.send_header());

        let frame = try!(self.tx.recv());
        let start: connection::Start = try!(frame.to_method());
        self.server_props = start.server_properties.clone();

        let mut start_ok = connection::StartOk::default();
        start_ok.client_properties = self.client_props();
        start_ok.mechanism = "PLAIN".to_string();
        start_ok.response = self.start_ok_response();
        start_ok.locale = "en_US".to_string();
        try!(self.tx.send(Frame::from_method(0, &start_ok).unwrap()));

        let frame = try!(self.tx.recv());
        let tune: connection::Tune = try!(frame.to_method());

        let mut tune_ok = connection::TuneOk::default();
        tune_ok.channel_max = tune.channel_max;
        tune_ok.frame_max = tune.frame_max;
        tune_ok.heartbeat = tune.heartbeat;
        try!(self.tx.send(Frame::from_method(0, &tune_ok).unwrap()));

        let open = connection::Open::default();
        try!(self.tx.send(Frame::from_method(0, &open).unwrap()));

        let frame = try!(self.tx.recv());
        let open_ok: connection::OpenOk = try!(frame.to_method());

        Ok(())
    }

    pub fn close(&mut self) -> AmqpResult<()> {
        let mut close = connection::Close::default();
        close.reply_code = protocol::REPLY_SUCCESS as u16;
        close.reply_text = "OK".to_string();
        try!(self.tx.send(Frame::from_method(0, &close).unwrap()));

        let frame = try!(self.tx.recv());
        let close_ok: connection::CloseOk = try!(frame.to_method());

        self.tx.close()
    }

    fn client_props(&self) -> Table {
        let mut client_props = Table::new();
        client_props.insert("product".to_string(), LongString(b"rust-amqp".to_vec()));
        client_props.insert("platform".to_string(), LongString(b"rust".to_vec()));
        client_props.insert("version".to_string(), LongString(b"0.0.1".to_vec()));
        client_props.insert("information".to_string(),
                            LongString(b"https://github.com/zbaitu/rust-amqp-client".to_vec()));

        let mut caps = Table::new();
        caps.insert("publisher_confirms".to_string(), Bool(true));
        caps.insert("exchange_exchange_bindings".to_string(), Bool(true));
        caps.insert("basic.nack".to_string(), Bool(true));
        caps.insert("consumer_cancel_notify".to_string(), Bool(true));
        caps.insert("connection.blocked".to_string(), Bool(true));
        caps.insert("authentication_failure_close".to_string(), Bool(true));
        client_props.insert("capabilities".to_string(), FieldTable(caps));

        client_props
    }

    fn start_ok_response(&self) -> Longstr {
        format!("\0{}\0{}", self.conf.user, self.conf.password).into_bytes()
    }
}
