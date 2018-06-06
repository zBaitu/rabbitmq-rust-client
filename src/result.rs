use std::convert;
use std::error::Error;
use std::fmt;
use std::io;
use std::result;
use std::string;
use std::u8;

use protocol;

#[derive(Debug)]
pub enum AmqpError {
    ShortStrTooLong(usize),
    FrameEndErr(u8),
    IoErr(io::Error),
    Utf8Err(string::FromUtf8Error),
}

impl convert::From<io::Error> for AmqpError {
    fn from(err: io::Error) -> AmqpError {
        AmqpError::IoErr(err)
    }
}

impl convert::From<string::FromUtf8Error> for AmqpError {
    fn from(err: string::FromUtf8Error) -> AmqpError {
        AmqpError::Utf8Err(err)
    }
}

impl Error for AmqpError {
    fn description(&self) -> &str {
        match *self {
            AmqpError::ShortStrTooLong(_) => "short string length is too long",
            AmqpError::FrameEndErr(_) => "frame end error",
            _ => Error::description(self),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AmqpError::ShortStrTooLong(_) | AmqpError::FrameEndErr(_) => None,
            _ => Some(self as &Error),
        }
    }
}

impl fmt::Display for AmqpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AmqpError::ShortStrTooLong(len) => {
                write!(f, "short string length[{}] > MAX[{}]", len, u8::MAX)
            },
            AmqpError::FrameEndErr(end) => {
                write!(f, "frame end[{}] != [{}]", end, protocol::FRAME_END)
            },
            _ => write!(f, "{}", Error::description(self)),
        }
    }
}

pub type AmqpResult<T> = result::Result<T, AmqpError>;
