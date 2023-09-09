use super::*;

#[napi(object)]
pub struct EqualizerCoefficient {
  pub beta: i32,
  pub alpha: i32,
  pub gamma: i32,
}

#[napi]
pub struct Equalizer {
  coefficients: Vec<EqualizerCoefficient>,
  format: PCMFormat,
  channels: AudioChannel,
}

#[napi]
impl Equalizer {
    #[napi(constructor)]
    pub fn new() -> Self {
        
    }
}