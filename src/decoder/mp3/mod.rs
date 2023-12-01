use minimp3_sys::{mp3d_sample_t, mp3dec_frame_info_t};
use minimp3_sys::{mp3dec_decode_frame, mp3dec_init, mp3dec_t, MINIMP3_MAX_SAMPLES_PER_FRAME};
use napi::bindgen_prelude::*;
use napi::Error;
use napi::Result;
use std::mem;

#[napi(object)]
pub struct Mp3DecodedFrame {
  pub frame_bytes: i32,
  pub frame_offset: i32,
  pub channels: i32,
  pub hz: i32,
  pub layer: i32,
  pub bitrate_kbps: i32,
  pub data: Buffer,
}

#[napi]
pub struct Mp3Decoder {
  decoder: mp3dec_t,
}

#[napi]
impl Mp3Decoder {
  #[napi(constructor)]
  pub fn new() -> Self {
    let mut context = unsafe { mem::zeroed() };
    unsafe { mp3dec_init(&mut context) };

    Self { decoder: context }
  }

  #[napi]
  pub fn decode(&mut self, data: Buffer) -> Result<Mp3DecodedFrame> {
    let mut frame_info: mp3dec_frame_info_t = unsafe { mem::zeroed() };
    let buffer = data.as_ref();
    let input_len = buffer.len();
    let mut pcm = Vec::with_capacity(MINIMP3_MAX_SAMPLES_PER_FRAME as usize);
    let samples: usize = unsafe {
      mp3dec_decode_frame(
        &mut self.decoder,
        buffer.as_ptr(),
        input_len as _,
        pcm.as_mut_ptr(),
        &mut frame_info,
      ) as _
    };

    if samples > 0 {
      unsafe {
        pcm.set_len(samples * frame_info.channels as usize);
      }
    }

    let out_vec = unsafe { std::mem::transmute::<&mut [i16], &mut [u8]>(&mut pcm.as_mut()) };

    Ok(Mp3DecodedFrame {
      frame_bytes: frame_info.frame_bytes,
      frame_offset: frame_info.frame_offset,
      channels: frame_info.channels,
      hz: frame_info.hz,
      layer: frame_info.layer,
      bitrate_kbps: frame_info.bitrate_kbps,
      data: out_vec.to_vec().into(),
    })
  }
}
