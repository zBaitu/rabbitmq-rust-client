use conf::Conf;
use connection::Connection;
use result::*;

pub struct Session {
    conf: Conf,
    conn: Connection,
}

impl Session {
    pub fn new() -> Session {
        let conf = Conf::default();
        Session {
            conf: conf.clone(),
            conn: Connection::new(conf),
        }
    }

    pub fn with_conf(conf: Conf) -> Session {
        let conn = Connection::new(conf.clone());
        Session {
            conf: conf,
            conn: conn,
        }
    }

    pub fn start(&mut self) -> AmqpResult<()> {
        self.conn.start()
    }

    pub fn close(&mut self) -> AmqpResult<()> {
        self.conn.close()
    }
}
