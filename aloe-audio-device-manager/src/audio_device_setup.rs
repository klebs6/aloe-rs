crate::ix!();

/**
  | This structure holds a set of properties
  | describing the current audio setup.
  | 
  | An AudioDeviceManager uses this class
  | to save/load its current settings,
  | and to specify your preferred options
  | when opening a device.
  | 
  | @see AudioDeviceManager::setAudioDeviceSetup(),
  | AudioDeviceManager::initialise()
  |
  */
pub struct AudioDeviceSetup {

    /**
      | The name of the audio device used for output. The
      | name has to be one of the ones listed by the AudioDeviceManager's
      | currently selected device type. This may be the
      | same as the input device. An empty string indicates
      | the default device.
      */
    output_device_name:          String,

    /**
      | The name of the audio device used for input. This
      | may be the same as the output device. An empty
      | string indicates the default device.
      */
    input_device_name:           String,

    /**
      | The current sample rate. This rate is used for
      | both the input and output devices. A value of 0
      | indicates that you don't care what rate is used,
      | and the device will choose a sensible rate for
      | you.
      */
    sample_rate:                 f64, // default = 0

    /**
      | The buffer size, in samples. This buffer size is
      | used for both the input and output devices. A value
      | of 0 indicates the default buffer size.
      */
    buffer_size:                 i32, // default = 0

    /**
      | The set of active input channels. The bits that
      | are set in this array indicate the channels of
      | the input device that are active. If useDefaultInputChannels
      | is true, this value is ignored.
      */
    input_channels:              BigInteger,

    /**
      | If this is true, it indicates that the inputChannels
      | array should be ignored, and instead, the device's
      | default channels should be used.
      */
    use_default_input_channels:  bool, // default = true

    /**
      | The set of active output channels. The bits that
      | are set in this array indicate the channels of
      | the input device that are active. If useDefaultOutputChannels
      | is true, this value is ignored.
      */
    output_channels:             BigInteger,

    /**
      | If this is true, it indicates that the outputChannels
      | array should be ignored, and instead, the device's
      | default channels should be used.
      */
    use_default_output_channels: bool, // default = true
}

impl PartialEq<AudioDeviceSetup> for AudioDeviceSetup {
    
    #[inline] fn eq(&self, other: &AudioDeviceSetup) -> bool {
        todo!();
        /*
            return outputDeviceName == other.outputDeviceName
                    && inputDeviceName == other.inputDeviceName
                    && sampleRate == other.sampleRate
                    && bufferSize == other.bufferSize
                    && inputChannels == other.inputChannels
                    && useDefaultInputChannels == other.useDefaultInputChannels
                    && outputChannels == other.outputChannels
                    && useDefaultOutputChannels == other.useDefaultOutputChannels;
        */
    }
}

impl Eq for AudioDeviceSetup {}
