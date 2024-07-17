crate::ix!();

/**
  | One of these is passed to an AudioIODevice
  | object to stream the audio data in and
  | out.
  | 
  | The AudioIODevice will repeatedly
  | call this class's audioDeviceIOCallback()
  | method on its own high-priority audio
  | thread, when it needs to send or receive
  | the next block of data.
  | 
  | @see AudioIODevice, AudioDeviceManager
  | 
  | @tags{Audio}
  |
  */
pub trait AudioIODeviceCallback {

    /**
      | Processes a block of incoming and outgoing
      | audio data.
      | 
      | The subclass's implementation should
      | use the incoming audio for whatever
      | purposes it needs to, and must fill all
      | the output channels with the next block
      | of output data before returning.
      | 
      | The channel data is arranged with the
      | same array indices as the channel name
      | array returned by AudioIODevice::getOutputChannelNames(),
      | but those channels that aren't specified
      | in AudioIODevice::open() will have
      | a null pointer for their associated
      | channel, so remember to check for this.
      | 
      | -----------
      | @param inputChannelData
      | 
      | a set of arrays containing the audio
      | data for each incoming channel - this
      | data is valid until the function returns.
      | There will be one channel of data for
      | each input channel that was enabled
      | when the audio device was opened (see
      | AudioIODevice::open())
      | ----------
      | @param numInputChannels
      | 
      | the number of pointers to channel data
      | in the inputChannelData array.
      | ----------
      | @param outputChannelData
      | 
      | a set of arrays which need to be filled
      | with the data that should be sent to each
      | outgoing channel of the device. There
      | will be one channel of data for each output
      | channel that was enabled when the audio
      | device was opened (see AudioIODevice::open())
      | The initial contents of the array is
      | undefined, so the callback function
      | must fill all the channels with zeros
      | if its output is silence. Failing to
      | do this could cause quite an unpleasant
      | noise!
      | ----------
      | @param numOutputChannels
      | 
      | the number of pointers to channel data
      | in the outputChannelData array.
      | ----------
      | @param numSamples
      | 
      | the number of samples in each channel
      | of the input and output arrays. The number
      | of samples will depend on the audio device's
      | buffer size and will usually remain
      | constant, although this isn't guaranteed.
      | For example, on Android, on devices
      | which support it, Android will chop
      | up your audio processing into several
      | smaller callbacks to ensure higher
      | audio performance. So make sure your
      | code can cope with reasonable changes
      | in the buffer size from one callback
      | to the next.
      |
      */
    fn audio_device_io_callback(&mut self, 
            input_channel_data:  *const *const f32,
            num_input_channels:  i32,
            output_channel_data: *mut *mut f32,
            num_output_channels: i32,
            num_samples:         i32);

    /**
      | Called to indicate that the device is
      | about to start calling back.
      | 
      | This will be called just before the audio
      | callbacks begin, either when this callback
      | has just been added to an audio device,
      | or after the device has been restarted
      | because of a sample-rate or block-size
      | change.
      | 
      | You can use this opportunity to find
      | out the sample rate and block size that
      | the device is going to use by calling
      | the AudioIODevice::getCurrentSampleRate()
      | and AudioIODevice::getCurrentBufferSizeSamples()
      | on the supplied pointer.
      | 
      | -----------
      | @param device
      | 
      | the audio IO device that will be used
      | to drive the callback. Note that if you're
      | going to store this this pointer, it
      | is only valid until the next time that
      | audioDeviceStopped is called.
      |
      */
    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface);

    /**
      | Called to indicate that the device has
      | stopped.
      |
      */
    fn audio_device_stopped(&mut self);

    /**
      | This can be overridden to be told if the
      | device generates an error while operating.
      | Be aware that this could be called by
      | any thread! And not all devices perform
      | this callback.
      |
      */
    fn audio_device_error(&mut self, error_message: &String)  { }
}
