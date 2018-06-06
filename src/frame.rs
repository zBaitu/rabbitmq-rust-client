use method::{self, Method};
use protocol::*;
use result::*;

#[derive(Debug)]
pub struct Frame {
    pub ty: u8,
    pub channel: u16,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn from_method<M: Method>(channel: u16, method: &M) -> AmqpResult<Frame> {
        let payload = try!(method.se());
        let frame = Frame {
            ty: FRAME_METHOD,
            channel: channel,
            payload: payload,
        };
        Ok(frame)
    }

    pub fn to_method<M: Method>(&self) -> AmqpResult<M> {
        method::de(&self.payload)
    }
}

#[cfg(test)]
mod tests {
    use protocol::*;
    use super::*;

    #[test]
    fn test() {
        let a = connection::Start::default();
        println!("{:#?}", a);

        let frame = Frame::from_method(0, &a).unwrap();
        println!("{:?}", frame);

        let b: connection::Start = frame.to_method().unwrap();
        println!("{:#?}", b);

        assert_eq!(a, b);
    }
}
