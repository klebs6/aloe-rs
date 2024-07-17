crate::ix!();

pub trait GetSampleRate {

    /**
      | Returns the current sample rate.
      | 
      | This can be called from your processBlock()
      | method - it's not guaranteed to be valid
      | at any other time, and may return 0 if
      | it's unknown.
      |
      */
    fn get_sample_rate(&self) -> f64;
}

pub trait GetCurrentSampleRate {

    /**
      | Returns the sample rate that the device
      | is currently using.
      | 
      | If the device isn't actually open, this
      | value doesn't really mean much.
      |
      */
    fn get_current_sample_rate(&mut self) -> f64;
}

pub trait GetAvailableSampleRates {

    /**
      | Returns the set of sample-rates this
      | device supports. @see getCurrentSampleRate
      |
      */
    fn get_available_sample_rates(&mut self) -> Vec<f64>;
}
