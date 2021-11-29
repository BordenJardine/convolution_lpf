//! The plugin's digital signal processing is fully implemented within this module.
//!
//! All updates to input parameters are received through message passing to avoid thread locking
//! during audio processing. In particular, note that parameter smoothing is considered within the
//! scope of audio processing rather than state management.

use std::sync::mpsc::Receiver;
use std::collections::VecDeque;
use vst::buffer::AudioBuffer;
use crate::plugin_state::StateUpdate;

pub mod convolution;
use convolution::convolve;
use convolution::windowed_sinc_filter;

const KERNAL_LEN: usize = 127;
const DEFAULT_CUTOFF: f32 = 0.25;

/// Handles all audio processing algorithms for the plugin.
pub(super) struct PluginDsp {
  cutoff: f32,
  messages_from_params: Receiver<StateUpdate>,
  filter_kernal: [f32; KERNAL_LEN],
  history_buffers: Vec<VecDeque<f32>>,
}

impl PluginDsp {
  pub fn new(incoming_messages: Receiver<StateUpdate>) -> Self {

    // init the filter kernal
    let mut filter_kernal: [f32; KERNAL_LEN] = [0.; KERNAL_LEN];
    windowed_sinc_filter(DEFAULT_CUTOFF, &mut filter_kernal);

    // init a buffer to hold on to the still-relevant samples during convolution
    let mut history_buffers = Vec::new();
    for _ in 0..2 {
      let mut history_buffer: VecDeque<f32> = VecDeque::new();
      for _ in 0..filter_kernal.len() {
        history_buffer.push_front(0.);
      }
      history_buffers.push(history_buffer);
    }

    Self {
      cutoff: DEFAULT_CUTOFF,
      messages_from_params: incoming_messages,
      filter_kernal,
      history_buffers
    }
  }

  /// Applies any incoming state update events to the audio generation algorithm, and then writes
  /// processed audio into the output buffer.
  pub fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    // First, get any new changes to parameter ranges.
    while let Ok(message) = self.messages_from_params.try_recv() {
      match message {
        StateUpdate::SetKnob(v) => {
          // v will be a number between 0. and 1.
          // cutoff should be a number between 0. and 0.5
          // see comment of windowed_sinc_filter as to why
          self.cutoff = (v * v) / 2.;
          windowed_sinc_filter(self.cutoff, &mut self.filter_kernal);
        },
      }
    }

    // verify length of the history buffer is the impulse_response + buffer_length
    //for mut history_buffer in self.history_buffers.iter() {
    for i in 0..self.history_buffers.len() {
      let history_buffer = &mut self.history_buffers[i];
      while history_buffer.len() < self.filter_kernal.len() + buffer.samples() {
        history_buffer.push_back(0.);
      }
    }

    // do some convolving
    let mut i = 0;
    for (input_buffer, output_buffer) in buffer.zip() {
      // for history_sample in self.history_buffer.iter_mut() {
      //   *history_sample = 0.0;
      // }
      let history_buffer = &mut self.history_buffers[i];
      i += 1;
      for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
        *output_sample = convolve(*input_sample, &self.filter_kernal, history_buffer);
      }
    }
  }
}
