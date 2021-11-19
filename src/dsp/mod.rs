//! The plugin's digital signal processing is fully implemented within this module.
//!
//! All updates to input parameters are received through message passing to avoid thread locking
//! during audio processing. In particular, note that parameter smoothing is considered within the
//! scope of audio processing rather than state management.

use crate::plugin_state::StateUpdate;
use std::sync::mpsc::Receiver;

use vst::buffer::AudioBuffer;
use std::collections::VecDeque;

pub mod filter_kernal;
pub mod convolution;
use filter_kernal::FILTER_KERNAL;
use convolution::convolve;
use convolution::windowed_sinc_filter;

/// Handles all audio processing algorithms for the plugin.
pub(super) struct PluginDsp {
  amplitude: f32,
  impulse_response: &'static[f32],
  history_buffer: VecDeque<f32>,
  messages_from_params: Receiver<StateUpdate>,
}

impl PluginDsp {
  pub fn new(incoming_messages: Receiver<StateUpdate>) -> Self {
    let mut history_buffer: VecDeque<f32> = VecDeque::new();
    let impulse_response = &FILTER_KERNAL;
    for _ in 0..impulse_response.len() {
      history_buffer.push_front(0.0);
    }
    Self {
      amplitude: 1.,
      messages_from_params: incoming_messages,
      filter_kernal: impulse_response,
      history_buffer: history_buffer,
    }
  }

  /// Applies any incoming state update events to the audio generation algorithm, and then writes
  /// processed audio into the output buffer.
  pub fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    // First, get any new changes to parameter ranges.
    while let Ok(message) = self.messages_from_params.try_recv() {
      match message {
        StateUpdate::SetKnob(v) => self.amplitude = v,
      }
    }

    // the length of the history buffer should be impulse_response + buffer_length
    while self.history_buffer.len() < self.impulse_response.len() + buffer.samples() {
      self.history_buffer.push_back(0.0);
    }

    // do some convolving
    for (input_buffer, output_buffer) in buffer.zip() {
      for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
        *output_sample = convolve(*input_sample, &self.impulse_response, &mut self.history_buffer);
      }
    }
  }
}
