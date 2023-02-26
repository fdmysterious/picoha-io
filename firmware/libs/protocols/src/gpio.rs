/// HA Protocol implementation for Gpio control

use crate::ha;
use heapless::Vec;

/// Enums for Gpio stuff

#[derive(Debug)]
pub enum GpioDir {
    PullDownInput,
    PullUpInput,
    Output,
}

impl GpioDir {
    pub fn from_u8(x: u8) -> Option<Self> {
        match x {
            0 => Some(Self::PullDownInput),
            1 => Some(Self::PullUpInput),
            2 => Some(Self::Output),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::PullDownInput  => 0,
            Self::PullUpInput    => 1,
            Self::Output         => 2,
        }
    }
}

#[derive(Debug)]
pub enum GpioValue {
    Low,
    High,
}

impl GpioValue {
    pub fn from_u8(x: u8) -> Option<Self> {
        match x {
            0 => Some(Self::Low),
            1 => Some(Self::High),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Low  => 0,
            Self::High => 1,
        }
    }
}

//////////////////////////////////////

#[derive(Debug)]
pub enum Request {
    Ping,
    ItfType,
    Version,
    IdGet,

    GpioDirSet(u8, GpioDir),
    GpioDirGet(u8),
    GpioWrite(u8, GpioValue),
    GpioRead(u8),
}

impl Request {
    pub fn consume_frame<const BUFLEN: usize>(ff: ha::MsgFrame<BUFLEN>) -> Result<Self, ha::MsgError> {
        match ff.code {
            ha::Code::Ping       => Ok(Self::Ping),
            ha::Code::ItfType    => Ok(Self::ItfType),
            ha::Code::Version    => Ok(Self::Version),
            ha::Code::IdGet      => Ok(Self::IdGet),

            ////////////////////////////////////////

            ha::Code::GpioDirSet => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let pin_idx = match argp.consume_u8() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                let dir = match argp.consume_u8() {
                    Some(x) => match GpioDir::from_u8(x) {
                        Some(d) => d,
                        None    => {return Err(ha::MsgError::InvalidArg);},
                    }

                    None => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::GpioDirSet(pin_idx, dir))
            },

            ha::Code::GpioDirGet => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let pin_idx = match argp.consume_u8() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::GpioDirGet(pin_idx))
            },

            ha::Code::GpioWrite => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let pin_idx = match argp.consume_u8() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                let value = match argp.consume_u8() {
                    Some(x) => match GpioValue::from_u8(x) {
                        Some(v) => v,
                        None    => {return Err(ha::MsgError::InvalidArg);}
                    },
                    None => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::GpioWrite(pin_idx, value))
            },

            ha::Code::GpioRead => {
                let mut argp = ha::ArgParser::new(&ff.data.as_slice());

                let pin_idx = match argp.consume_u8() {
                    Some(x) => x,
                    None    => {return Err(ha::MsgError::InvalidArg);}
                };

                Ok(Self::GpioRead(pin_idx))
            }

            _ => Err(ha::MsgError::NotARequest(ff.code))
        }
    }
}

//////////////////////////////////////

#[derive(Debug)]
pub enum Response {
    Good,
    GpioValue(u8, GpioValue),
}

impl Response {
    pub fn to_frame(&self) -> ha::MsgFrame<2> {
        match self {
            Self::Good => {
                ha::MsgFrame {
                    code: ha::Code::Good,
                    data: Vec::new(),
                }
            }

            Self::GpioValue(idx, value) => {
                ha::MsgFrame::<2> {
                    code: ha::Code::GpioValue,
                    data: Vec::from_slice(&[*idx, value.to_u8()]).unwrap()
                }
            }
        }
    }
}
