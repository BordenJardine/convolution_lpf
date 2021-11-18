//! The plugin's digital signal processing is fully implemented within this module.
//!
//! All updates to input parameters are received through message passing to avoid thread locking
//! during audio processing. In particular, note that parameter smoothing is considered within the
//! scope of audio processing rather than state management.

use crate::plugin_state::StateUpdate;
use std::sync::mpsc::Receiver;

use vst::buffer::AudioBuffer;

/// Handles all audio processing algorithms for the plugin.
pub(super) struct PluginDsp {
  amplitude: f32,
  messages_from_params: Receiver<StateUpdate>,
}

impl PluginDsp {
  pub fn new(incoming_messages: Receiver<StateUpdate>) -> Self {
    Self {
      amplitude: 1.,
      messages_from_params: incoming_messages,
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

    let num_channels = buffer.input_count();
    let num_samples = buffer.samples();
    let (inputs, mut outputs) = buffer.split();
    // Then, calculate each output sample by multiplying each input sample by its
    // corresponding amplitude value.
    for channel in 0..num_channels {
      for i in 0..num_samples {
        outputs[channel][i] = inputs[channel][i] * self.amplitude;
      }
    }
  }
}
