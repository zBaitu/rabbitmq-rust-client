extern crate amqp;

use std::io::{self, Read};

use amqp::conf::Conf;
use amqp::session::Session;

fn main() {
    let mut sess = Session::new();
    sess.start().unwrap();
    io::stdin().read_to_string(&mut String::new());
    sess.close().unwrap();
}
