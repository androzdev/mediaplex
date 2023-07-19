use std::ops::Deref;
use thiserror::Error;

pub mod symphonia_decoder;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("Symphonia Decoder Error: {0}")]
    SymphoniaDecoder(String),
}

pub type DecoderResult<T> = Result<T, DecoderError>;

#[derive(Error, Debug)]
pub enum AudioPacketError {
    #[error("Decoder Raw Error: Can't return Raw on Samples")]
    Raw,
    #[error("Decoder Samples Error: Can't return Samples on Raw")]
    Samples,
}

pub type AudioPacketResult<T> = Result<T, AudioPacketError>;

pub enum AudioPacket {
    Samples(Vec<f64>),
    Raw(Vec<u8>),
}

impl AudioPacket {
    pub fn samples(&self) -> AudioPacketResult<&[f64]> {
        match self {
            AudioPacket::Samples(s) => Ok(s),
            AudioPacket::Raw(_) => Err(AudioPacketError::Raw),
        }
    }

    pub fn raw(&self) -> AudioPacketResult<&[u8]> {
        match self {
            AudioPacket::Raw(d) => Ok(d),
            AudioPacket::Samples(_) => Err(AudioPacketError::Samples),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            AudioPacket::Samples(s) => s.is_empty(),
            AudioPacket::Raw(d) => d.is_empty(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioPacketPosition {
    pub position_ms: u32,
    pub skipped: bool,
}

impl Deref for AudioPacketPosition {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.position_ms
    }
}

pub trait AudioDecoder {
    fn seek(&mut self, position_ms: u32) -> Result<u32, DecoderError>;
    fn next_packet(
        &mut self,
    ) -> DecoderResult<Option<(AudioPacketPosition, AudioPacket, u16, u32)>>;
}

impl From<symphonia::core::errors::Error> for DecoderError {
    fn from(err: symphonia::core::errors::Error) -> Self {
        Self::SymphoniaDecoder(err.to_string())
    }
}