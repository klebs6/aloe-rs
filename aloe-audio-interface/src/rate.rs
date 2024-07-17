crate::ix!();

pub trait SetRateAndBufferSizeDetails {

    /**
      | This is called by the processor to specify
      | its details before being played. You
      | should call this function after having
      | informed the processor about the channel
      | and bus layouts via setBusesLayout.
      | 
      | @see setBusesLayout
      |
      */
    fn set_rate_and_buffer_size_details(
        &mut self, 
        sample_rate: f64,
        block_size:  i32
    );
}
