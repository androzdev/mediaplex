// use symphonium::{DecodedAudio, SymphoniumLoader};

// fn create_media_source() {}

// struct Transcoder {
//   audio_data: DecodedAudio,
//   frames_elapsed: usize,
//   temp_buf_l: Vec<f32>,
//   temp_buf_r: Vec<f32>,
// }

// impl Transcoder {
//   fn create(audio_data: DecodedAudio, max_buffer_size: usize) -> Self {
//     Transcoder {
//       audio_data,
//       frames_elapsed: 0,
//       temp_buf_l: vec![0.0; max_buffer_size],
//       temp_buf_r: vec![0.0; max_buffer_size],
//     }
//   }

//   fn process(&mut self, output: &mut [f32]) {
//     let frames = output.len() / 2;

//     self.audio_data.fill_stereo(
//       self.frames_elapsed,
//       &mut self.temp_buf_l[..frames],
//       &mut self.temp_buf_r[..frames],
//     );

//     for (out, (&in1, &in2)) in output
//       .chunks_exact_mut(2)
//       .zip(self.temp_buf_l.iter().zip(self.temp_buf_r.iter()))
//     {
//       out[0] = in1;
//       out[1] = in2;
//     }

//     self.frames_elapsed += frames;
//   }
// }
