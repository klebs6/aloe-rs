crate::ix!();

pub trait GetLatencySamples {

    /**
      | This returns the number of samples delay
      | that the processor imposes on the audio
      | passing through it.
      | 
      | The host will call this to find the latency
      | - the processor itself should set this
      | value by calling setLatencySamples()
      | as soon as it can during its initialisation.
      |
      */
    fn get_latency_samples(&self) -> i32;
}

pub trait SetLatencySamples {

    /**
      | Your processor subclass should call
      | this to set the number of samples delay
      | that it introduces.
      | 
      | The processor should call this as soon
      | as it can during initialisation, and
      | can call it later if the value changes.
      |
      */
    fn set_latency_samples(&mut self, new_latency: i32);
}

pub trait GetOutputLatencyInSamples {

    /**
      | Returns the device's output latency.
      | 
      | This is the delay in samples between
      | a callback getting a block of data, and
      | that data actually getting played.
      |
      */
    fn get_output_latency_in_samples(&mut self) -> i32;
}

pub trait GetInputLatencyInSamples {

    /**
      | Returns the device's input latency.
      | 
      | This is the delay in samples between
      | some audio actually arriving at the
      | soundcard, and the callback getting
      | passed this block of data.
      |
      */
    fn get_input_latency_in_samples(&mut self) -> i32;
}
