use std::collections::VecDeque;
use std::f32::consts::PI;

// take in an impulse, an IR, and a buffer of the work done so far
// perform the convolution of the impulse and the IR onto the buffer
// pop and return the first value
pub fn convolve(
  input_sample: f32,
  impulse_response: &[f32],
  history_buffer: &mut VecDeque<f32>
) -> f32 {
  let in_sample = input_sample / 3.;
  for (i, kernal_sample) in impulse_response.iter().enumerate() {
    match history_buffer.get_mut(i) {
      Some(history_sample) => *history_sample = math_stuff(*history_sample, in_sample, *kernal_sample),
      None => ()
    }
  }

  match history_buffer.pop_front() {
    Some(output) => {
      history_buffer.push_back(0.0);
      return output;
    },
    None => return input_sample
  }
}

pub fn math_stuff( history_sample: f32, input_sample: f32, kernal_sample: f32) -> f32 {
    let val = history_sample + input_sample * kernal_sample;
    val
}

// cutoff_freq should be a number between 0 and 0.5
// it represents a ratio of the sample frequency (e.g. 44.1khz)
// and we don't want to let the filter go above the nyquist freq (sample rate * 0.5)
pub fn windowed_sinc_filter(cutoff_freq: f32, filter_kernal: &mut [f32]) {
  let cutoff = cutoff_freq.clamp(0.0, 0.5);
  let len = filter_kernal.len();
  let f_len = (filter_kernal.len() - 1) as f32;
  for (i, kernal_sample) in filter_kernal.iter_mut().enumerate() {
    let offset = i - len / 2;
    if offset == 0 {
      // mid point
      *kernal_sample = 2.0 * PI * cutoff;
    } else {
      let f_offset = offset as f32;
      // sinc function
      *kernal_sample = (2.0 * PI * cutoff * f_offset).sin() / f_offset;
      // hamming
      *kernal_sample *= 0.54-0.46*(2.0*PI*(i as f32)/f_len).cos();
    }
  }
}
