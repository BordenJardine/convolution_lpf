use std::collections::VecDeque;
pub fn convolve(
  input_sample: f32,
  impulse_response: &[f32],
  history_buffer: &mut VecDeque<f32>
) -> f32 {
  for (i, kernal_sample) in impulse_response.iter().enumerate() {
    match history_buffer.get_mut(i) {
      Some(history_sample) => *history_sample += (input_sample * kernal_sample) / 2.0,
      None => return 0.0
    }
  }

  match history_buffer.pop_front() {
    Some(output) => {
      history_buffer.push_back(0.0);
      return output;
    },
    None => return 0.0
  }
}

