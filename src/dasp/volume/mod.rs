use super::*;

#[napi]
pub struct VolumeTransformer {
  volume: i32,
  format: PCMFormat,
  channels: AudioChannel,
}

// TODO: investigate SIMD optimizations

#[napi]
impl VolumeTransformer {
  #[napi(constructor)]
  pub fn new(volume: i32, format: PCMFormat, channels: AudioChannel) -> Self {
    Self {
      volume,
      format,
      channels,
    }
  }

  #[napi(setter)]
  pub fn set_volume(&mut self, volume: i32) {
    if volume < 0 {
      self.volume = 0;
    } else {
      self.volume = volume;
    }
  }

  #[napi(getter)]
  pub fn get_volume(&self) -> i32 {
    self.volume
  }

  #[napi(setter)]
  pub fn set_format(&mut self, format: PCMFormat) {
    self.format = format;
  }

  #[napi(getter)]
  pub fn get_format(&self) -> PCMFormat {
    self.format
  }

  #[napi(setter)]
  pub fn set_channels(&mut self, channels: AudioChannel) {
    self.channels = channels;
  }

  #[napi(getter)]
  pub fn get_channels(&self) -> AudioChannel {
    self.channels
  }

  #[napi]
  pub fn process(&self, input: Buffer) -> Buffer {
    // avoid computation if volume is 1, aka default volume
    if self.volume == 1 {
      return input;
    }

    match self.channels {
      AudioChannel::Mono => self.process_mono_inner(input),
      AudioChannel::Stereo => self.process_stereo_inner(input),
    }
  }

  fn process_stereo_inner(&self, input: Buffer) -> Buffer {
    match self.format {
      PCMFormat::S16LE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(4) {
          let mut sample = i16::from_le_bytes([input[i], input[i + 1]]);
          sample = (sample as i32 * self.volume) as i16;
          output.extend_from_slice(&sample.to_le_bytes());
          let mut sample = i16::from_le_bytes([input[i + 2], input[i + 3]]);
          sample = (sample as i32 * self.volume) as i16;
          output.extend_from_slice(&sample.to_le_bytes());
        }
        output.into()
      }
      PCMFormat::S16BE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(4) {
          let mut sample = i16::from_be_bytes([input[i], input[i + 1]]);
          sample = (sample as i32 * self.volume) as i16;
          output.extend_from_slice(&sample.to_be_bytes());
          let mut sample = i16::from_be_bytes([input[i + 2], input[i + 3]]);
          sample = (sample as i32 * self.volume) as i16;
          output.extend_from_slice(&sample.to_be_bytes());
        }
        output.into()
      }
      PCMFormat::S32LE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(8) {
          let mut sample = i32::from_le_bytes([input[i], input[i + 1], input[i + 2], input[i + 3]]);
          sample = (sample as i32 * self.volume) as i32;
          output.extend_from_slice(&sample.to_le_bytes());
          let mut sample =
            i32::from_le_bytes([input[i + 4], input[i + 5], input[i + 6], input[i + 7]]);
          sample = (sample as i32 * self.volume) as i32;
          output.extend_from_slice(&sample.to_le_bytes());
        }
        output.into()
      }
      PCMFormat::S32BE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(8) {
          let mut sample = i32::from_be_bytes([input[i], input[i + 1], input[i + 2], input[i + 3]]);
          sample = (sample as i32 * self.volume) as i32;
          output.extend_from_slice(&sample.to_be_bytes());
          let mut sample =
            i32::from_be_bytes([input[i + 4], input[i + 5], input[i + 6], input[i + 7]]);
          sample = (sample as i32 * self.volume) as i32;
          output.extend_from_slice(&sample.to_be_bytes());
        }
        output.into()
      }
    }
  }

  fn process_mono_inner(&self, input: Buffer) -> Buffer {
    match self.format {
      PCMFormat::S16LE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(2) {
          let mut sample = i16::from_le_bytes([input[i], input[i + 1]]);
          sample = (sample as i32 * self.volume) as i16;
          output.extend_from_slice(&sample.to_le_bytes());
        }
        output.into()
      }
      PCMFormat::S16BE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(2) {
          let mut sample = i16::from_be_bytes([input[i], input[i + 1]]);
          sample = (sample as i32 * self.volume) as i16;
          output.extend_from_slice(&sample.to_be_bytes());
        }
        output.into()
      }
      PCMFormat::S32LE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(4) {
          let mut sample = i32::from_le_bytes([input[i], input[i + 1], input[i + 2], input[i + 3]]);
          sample = (sample as i32 * self.volume) as i32;
          output.extend_from_slice(&sample.to_le_bytes());
        }
        output.into()
      }
      PCMFormat::S32BE => {
        let mut output = Vec::with_capacity(input.len());
        for i in (0..input.len()).step_by(4) {
          let mut sample = i32::from_be_bytes([input[i], input[i + 1], input[i + 2], input[i + 3]]);
          sample = (sample as i32 * self.volume) as i32;
          output.extend_from_slice(&sample.to_be_bytes());
        }
        output.into()
      }
    }
  }
}
