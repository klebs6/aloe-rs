crate::ix!();

pub trait OpenDevice {

    /**
      | Tries to open the device ready to play.
      | 
      | -----------
      | @param inputChannels
      | 
      | a BigInteger in which a set bit indicates
      | that the corresponding input channel
      | should be enabled
      | ----------
      | @param outputChannels
      | 
      | a BigInteger in which a set bit indicates
      | that the corresponding output channel
      | should be enabled
      | ----------
      | @param sampleRate
      | 
      | the sample rate to try to use - to find
      | out which rates are available, see getAvailableSampleRates()
      | ----------
      | @param bufferSizeSamples
      | 
      | the size of i/o buffer to use - to find
      | out the available buffer sizes, see
      | getAvailableBufferSizes()
      | 
      | -----------
      | @return
      | 
      | an error description if there's a problem,
      | or an empty string if it succeeds in opening
      | the device @see close
      |
      */
    fn open(&mut self, 
        input_channels:      &BigInteger,
        output_channels:     &BigInteger,
        sample_rate:         f64,
        buffer_size_samples: i32) -> String;
}

pub trait Close {

    /**
      | Closes and releases the device if it's
      | open.
      |
      */
    fn close(&mut self);
}

pub trait DeviceIsOpen {

    /**
      | Returns true if the device is still open.
      | 
      | A device might spontaneously close
      | itself if something goes wrong, so this
      | checks if it's still open.
      |
      */
    fn is_open(&mut self) -> bool;
}
