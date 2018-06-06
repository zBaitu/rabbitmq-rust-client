use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use std::io::{Read, Write};
use std::net::Shutdown;
use std::net::TcpStream;

use conf::Conf;
use frame::Frame;
use protocol;
use result::*;

pub struct Transport {
    stream: TcpStream,
}

impl Transport {
    pub fn new(conf: &Conf) -> Transport {
        Transport {
            stream: TcpStream::connect((&*conf.host, conf.port)).unwrap(),
        }
    }

    pub fn send_header(&mut self) -> AmqpResult<()> {
        try!(self.stream.write_all(&protocol::PROT_HEADER));
        Ok(())
    }

    pub fn send(&mut self, frame: Frame) -> AmqpResult<()> {
        try!(self.stream.write_u8(frame.ty));
        try!(self.stream.write_u16::<BigEndian>(frame.channel));
        try!(self.stream.write_u32::<BigEndian>(frame.payload.len() as u32));
        try!(self.stream.write_all(&frame.payload));
        try!(self.stream.write_u8(protocol::FRAME_END));
        Ok(())
    }

    pub fn recv(&mut self) -> AmqpResult<Frame> {
        let mut header = &mut [0u8; 7] as &mut [u8];
        try!(self.stream.read_exact(&mut header));
        let header = &mut (header as &[u8]) as &mut &[u8];

        let ty = try!(header.read_u8());
        let channel = try!(header.read_u16::<BigEndian>());
        let size = try!(header.read_u32::<BigEndian>()) as usize;

        let mut payload = vec![0u8; size];
        try!(self.stream.read_exact(&mut payload));

        let frame_end = try!(self.stream.read_u8());
        if frame_end != protocol::FRAME_END {
            return Err(AmqpError::FrameEndErr(frame_end));
        }

        let frame = Frame {
            ty: ty,
            channel: channel,
            payload: payload,
        };
        Ok(frame)
    }

    pub fn close(&self) -> AmqpResult<()> {
        try!(self.stream.shutdown(Shutdown::Both));
        Ok(())
    }
}
