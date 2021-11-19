use std::collections::VecDeque;
pub fn convolve(
  input_sample: f32,
  impulse_response: &[f32],
  history_buffer: &mut VecDeque<f32>
) -> f32 {
  for (i, kernal_sample) in impulse_response.iter().enumerate() {
    match history_buffer.get_mut(i) {
      Some(history_sample) => *history_sample += (input_sample  * kernal_sample),
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

pub fn windowed_sinc_filter(cutoff: &f32, filter_kernal: mut &[f32]) {
}

/*
void lp_windowed_sinc_ftr(
    double *sig_src_arr,
    double *sig_dest_arr,
    double *fltr_kernel_dest_arr,
    double cutoff_freq,
    int filter_length,
    int input_sig_length
)
{
    for(int i = 0; i < filter_length; i++)
    {
        double offset = i - (filter_length / 2);
        if(offset == 0)
        {
            // mid point
            fltr_kernel_dest_arr[i] = 2 * M_PI * cutoff_freq;
        }
        if(offset != 0)
        {
            // sinc function
            fltr_kernel_dest_arr[i] = sin(2 * M_PI * cutoff_freq * offset) / offset;
            // hamming
            fltr_kernel_dest_arr[i] = fltr_kernel_dest_arr[i] * (0.54-0.46*cos(2*M_PI*i/filter_length));
        }
    }

    //convolve that shit
    for(int i = filter_length; i < input_sig_length; i++)
    {
        sig_dest_arr[i] = 0;
        for(int j = 0; j < filter_length; j++)
        {
            sig_dest_arr[i] = sig_dest_arr[i] + sig_src_arr[i-j] * fltr_kernel_dest_arr[j];
        }
    }
}
 */
