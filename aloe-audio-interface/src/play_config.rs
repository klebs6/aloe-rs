crate::ix!();

pub trait SetPlayConfigDetails {

    /**
      | This is called by the processor to specify
      | its details before being played. Use
      | this version of the function if you are
      | not interested in any sidechain and/or
      | aux buses and do not care about the layout
      | of channels. Otherwise use setRateAndBufferSizeDetails.
      |
      */
    fn set_play_config_details(
        &mut self, 
        num_ins:     i32,
        num_outs:    i32,
        sample_rate: f64,
        block_size:  i32);

}
