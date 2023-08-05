use audiopus::coder::{Decoder, Encoder};
use audiopus::{Application, Bitrate, Channels, SampleRate};
use napi::bindgen_prelude::*;
use napi::Error;
use napi::Result;

const FRAME_SIZE: usize = 960;
const MAX_FRAME_SIZE: usize = 6 * FRAME_SIZE;

#[napi]
pub struct OpusEncoder {
  encoder: Option<Encoder>,
  decoder: Option<Decoder>,
  sample_rate: i32,
  channels: i32,
  out_buffer: Vec<u8>,
}

#[napi]
impl OpusEncoder {
  #[napi(constructor)]
  pub fn new(sample_rate: i32, channels: i32) -> Result<Self> {
    if channels != 1 && channels != 2 {
      return Err(Error::new(Status::InvalidArg, "Invalid channels"));
    }

    Ok(OpusEncoder {
      encoder: None,
      decoder: None,
      sample_rate,
      channels,
      out_buffer: vec![0u8; MAX_FRAME_SIZE],
    })
  }

  fn ensure_encoder(&mut self) {
    if self.encoder.is_none() {
      let rate = self.resolve_sample_rate().unwrap();
      let channels = self.resolve_channels().unwrap();
      self.encoder = Some(Encoder::new(rate, channels, Application::Audio).unwrap());
    }
  }

  fn ensure_decoder(&mut self) {
    if self.decoder.is_none() {
      let rate = self.resolve_sample_rate().unwrap();
      let channels = self.resolve_channels().unwrap();
      self.decoder = Some(Decoder::new(rate, channels).unwrap());
    }
  }

  fn resolve_sample_rate(&self) -> std::result::Result<SampleRate, Error> {
    match self.sample_rate {
      8000 => Ok(SampleRate::Hz8000),
      12000 => Ok(SampleRate::Hz12000),
      16000 => Ok(SampleRate::Hz16000),
      24000 => Ok(SampleRate::Hz24000),
      48000 => Ok(SampleRate::Hz48000),
      _ => return Err(Error::new(Status::InvalidArg, "Invalid sample rate")),
    }
  }

  fn resolve_channels(&self) -> std::result::Result<Channels, Error> {
    match self.channels {
      1 => Ok(Channels::Mono),
      2 => Ok(Channels::Stereo),
      _ => return Err(Error::new(Status::InvalidArg, "Invalid channels")),
    }
  }

  #[napi(catch_unwind)]
  pub fn encode(&mut self, data: Buffer) -> Result<Buffer> {
    self.ensure_encoder();

    let encoder = self.encoder.as_mut().unwrap();
    let data_ref: &[u8] = &data;

    let data_i16 = unsafe {
      let ptr = data_ref.as_ptr() as *const i16;
      let len = data_ref.len() / 2;
      std::slice::from_raw_parts(ptr, len)
    };

    let len = encoder
      .encode(data_i16, &mut self.out_buffer)
      .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to encode: {}", e)))?;

    Ok(self.out_buffer[..len].to_vec().into())
  }

  #[napi(catch_unwind)]
  pub fn decode(&mut self, data: Buffer) -> Result<Buffer> {
    self.ensure_decoder();

    let decoder = self.decoder.as_mut().unwrap();
    let data_ref: &[u8] = &data;

    let mut out = vec![0i16; MAX_FRAME_SIZE * self.channels as usize];

    let len = decoder
      .decode(Some(data_ref), &mut out, false)
      .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to decode: {}", e)))?;

    if len == 0 {
      return Err(Error::new(Status::GenericFailure, "Failed to decode"));
    }

    let decoded_length = len * 2 * self.channels as usize;

    let out = unsafe {
      let ptr = out.as_ptr() as *const u8;
      std::slice::from_raw_parts(ptr, decoded_length)
    };

    Ok(out.to_vec().into())
  }

  #[napi(catch_unwind)]
  pub fn set_bitrate(&mut self, bitrate: i32) -> Result<()> {
    self.ensure_encoder();

    let encoder = self.encoder.as_mut().unwrap();

    encoder
      .set_bitrate(Bitrate::BitsPerSecond(bitrate))
      .map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to set bitrate: {}", e),
        )
      })?;

    Ok(())
  }

  #[napi(catch_unwind)]
  pub fn get_bitrate(&mut self) -> Result<i32> {
    self.ensure_encoder();

    let encoder = self.encoder.as_mut().unwrap();

    let bitrate = encoder.bitrate().map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to get bitrate: {}", e),
      )
    })?;

    match bitrate {
      Bitrate::BitsPerSecond(bitrate) => Ok(bitrate),
      _ => Err(Error::new(
        Status::GenericFailure,
        "Failed to get bitrate: Invalid bitrate",
      )),
    }
  }

  #[napi(catch_unwind)]
  pub fn apply_encoder_ctl(&mut self, request: i32, value: i32) -> Result<()> {
    self.ensure_encoder();

    let encoder = self.encoder.as_mut().unwrap();

    encoder
      .set_encoder_ctl_request(request, value)
      .map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to apply encoder ctl: {}", e),
        )
      })?;

    Ok(())
  }

  #[napi]
  pub fn apply_decoder_ctl(&mut self, _request: i32, _value: i32) -> Result<()> {
    // Currently, this function is not implemented due to being private in the library.
    // It will be implemented if it becomes available in the future. This currently exists
    // for compatibility reasons.
    Ok(())
  }

  #[napi]
  pub fn destroy(&mut self) {
    self.encoder = None;
    self.decoder = None;
  }
}
