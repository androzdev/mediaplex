use napi::bindgen_prelude::*;
use napi::{Error, Result};

use unsafe_libopus::{
  opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy, opus_encode,
  opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy, opus_get_version_string,
  OPUS_ALLOC_FAIL, OPUS_APPLICATION_AUDIO, OPUS_BAD_ARG, OPUS_BUFFER_TOO_SMALL,
  OPUS_GET_BITRATE_REQUEST, OPUS_INTERNAL_ERROR, OPUS_INVALID_PACKET, OPUS_INVALID_STATE, OPUS_OK,
  OPUS_SET_BITRATE_REQUEST, OPUS_UNIMPLEMENTED,
};

const FRAME_SIZE: usize = 960;
const MAX_FRAME_SIZE: usize = 6 * FRAME_SIZE;
const MAX_PACKET_SIZE: usize = 3 * 1276;

fn get_decode_error(decoded_samples: i32) -> &'static str {
  match decoded_samples {
    OPUS_BAD_ARG => "One or more invalid/out of range arguments",
    OPUS_BUFFER_TOO_SMALL => "The mode struct passed is invalid",
    OPUS_INTERNAL_ERROR => "An internal error was detected",
    OPUS_INVALID_PACKET => "The compressed data passed is corrupted",
    OPUS_UNIMPLEMENTED => "Invalid/unsupported request number",
    OPUS_INVALID_STATE => "An encoder or decoder structure is invalid or already freed.",
    OPUS_ALLOC_FAIL => "Memory allocation has failed",
    _ => "Unknown OPUS error",
  }
}

#[napi]
pub fn get_opus_version() -> String {
  let version_string = opus_get_version_string();
  version_string.to_string()
}

#[napi]
pub struct OpusEncoder {
  encoder: *mut unsafe_libopus::OpusEncoder,
  decoder: *mut unsafe_libopus::OpusDecoder,
  sample_rate: i32,
  channels: i32,
}

#[napi]
impl OpusEncoder {
  #[napi(ts_return_type = "OpusEncoder")]
  pub fn create(sample_rate: i32, channels: i32) -> Result<Self> {
    Ok(Self {
      encoder: std::ptr::null_mut(),
      decoder: std::ptr::null_mut(),
      sample_rate,
      channels,
    })
  }

  fn ensure_encoder(&mut self) -> i32 {
    if self.encoder.is_null() {
      let mut opus_code = 0;

      let pointer = unsafe {
        opus_encoder_create(
          self.sample_rate,
          self.channels,
          OPUS_APPLICATION_AUDIO,
          &mut opus_code,
        )
      };

      if opus_code == OPUS_OK || !pointer.is_null() {
        self.encoder = pointer;
      }

      return opus_code;
    }

    OPUS_OK
  }

  fn ensure_decoder(&mut self) -> i32 {
    if self.decoder.is_null() {
      let mut opus_code = 0;

      let pointer = unsafe { opus_decoder_create(self.sample_rate, self.channels, &mut opus_code) };

      if opus_code == OPUS_OK || !pointer.is_null() {
        self.decoder = pointer;
      }

      return opus_code;
    }

    OPUS_OK
  }

  #[napi]
  pub fn has_encoder(&self) -> bool {
    !self.encoder.is_null()
  }

  #[napi]
  pub fn has_decoder(&self) -> bool {
    !self.decoder.is_null()
  }

  #[napi(catch_unwind)]
  pub fn encode(&mut self, data: Buffer) -> Result<Buffer> {
    let status = self.ensure_encoder();

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to create encoder: {}", get_decode_error(status)),
      ));
    }

    let mut out_buffer = vec![0u8; MAX_PACKET_SIZE];

    let pcm = unsafe { std::mem::transmute::<&[u8], &[i16]>(&data) };

    let frame_size = data.len() / 2 / (self.channels as usize);

    let compressed_len = unsafe {
      opus_encode(
        self.encoder,
        pcm.as_ptr(),
        frame_size as i32,
        out_buffer.as_mut_ptr(),
        MAX_PACKET_SIZE as i32,
      )
    };

    out_buffer.truncate(compressed_len as usize);
    Ok(out_buffer.into())
  }

  #[napi(catch_unwind)]
  pub fn decode(&mut self, data: Buffer) -> Result<Buffer> {
    let status = self.ensure_decoder();

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to create decoder: {}", get_decode_error(status)),
      ));
    }

    let mut out = vec![0i16; MAX_FRAME_SIZE * self.channels as usize];

    let decoded_samples = unsafe {
      opus_decode(
        self.decoder,
        data.as_ptr(),
        data.len() as i32,
        out.as_mut_ptr(),
        MAX_FRAME_SIZE as i32,
        0,
      )
    };

    if decoded_samples < 0 {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to decode: {}", get_decode_error(decoded_samples)),
      ));
    }

    let out = unsafe {
      let ptr = out.as_ptr() as *const u8;
      std::slice::from_raw_parts(ptr, decoded_samples as usize * 4)
    };

    Ok(out.to_vec().into())
  }

  #[napi(catch_unwind)]
  pub fn set_bitrate(&mut self, bitrate: i32) -> Result<()> {
    let status = self.ensure_encoder();

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to create encoder: {}", get_decode_error(status)),
      ));
    }

    let status = unsafe { opus_encoder_ctl!(self.encoder, OPUS_SET_BITRATE_REQUEST, bitrate) };

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to apply encoder ctl: {}", get_decode_error(status)),
      ));
    }

    Ok(())
  }

  #[napi(catch_unwind)]
  pub fn get_bitrate(&mut self) -> Result<i32> {
    let status = self.ensure_encoder();

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to create encoder: {}", get_decode_error(status)),
      ));
    }

    let mut value = 0;

    let status = unsafe { opus_encoder_ctl!(self.encoder, OPUS_GET_BITRATE_REQUEST, &mut value) };

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to apply encoder ctl: {}", get_decode_error(status)),
      ));
    }

    Ok(value)
  }

  #[napi(catch_unwind)]
  pub fn apply_encoder_ctl(&mut self, request: i32, value: i32) -> Result<()> {
    let status = self.ensure_encoder();

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to create encoder: {}", get_decode_error(status)),
      ));
    }

    let status = unsafe { opus_encoder_ctl!(self.encoder, request, value) };

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to apply encoder ctl: {}", get_decode_error(status)),
      ));
    }

    Ok(())
  }

  #[napi]
  pub fn apply_decoder_ctl(&mut self, request: i32, value: i32) -> Result<()> {
    let status = self.ensure_decoder();

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to create decoder: {}", get_decode_error(status)),
      ));
    }

    let status = unsafe { opus_decoder_ctl!(self.decoder, request, value) };

    if status != OPUS_OK {
      return Err(Error::new(
        Status::GenericFailure,
        format!("Failed to apply decoder ctl: {}", get_decode_error(status)),
      ));
    }

    Ok(())
  }

  #[napi(getter)]
  pub fn get_version(&self) -> String {
    get_opus_version()
  }
}

impl Drop for OpusEncoder {
  fn drop(&mut self) {
    if !self.encoder.is_null() {
      unsafe {
        opus_encoder_destroy(self.encoder);
      }
    }

    if !self.decoder.is_null() {
      unsafe {
        opus_decoder_destroy(self.decoder);
      }
    }
  }
}
