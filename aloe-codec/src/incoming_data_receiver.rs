crate::ix!();

/**
  | Receiver for incoming data.
  |
  */
pub trait IncomingDataReceiver {

    fn reset(
        &mut self, 
        num_channels:            i32,
        sample_rate:             f64,
        total_samples_in_source: Option<i64>
    );

    fn add_block(
        &mut self, 
        sample_number_in_source: i64,
        new_data:                &AudioBuffer<f32>,
        start_offset_in_buffer:  i32,
        num_samples:             i32
    );
}
