crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/AudioLiveScrollingDisplay.h]

/**
  | This component scrolls a continuous
  | waveform showing the audio that's coming
  | into whatever audio inputs this object
  | is connected to.
  |
  */
#[no_copy]
#[leak_detector]
pub struct LiveScrollingAudioDisplay<'a> {
    base:  AudioVisualiserComponent<'a>,
}

impl<'a> Default for LiveScrollingAudioDisplay<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : audio_visualiser_component(1),

            setSamplesPerBlock (256);
            setBufferSize (1024)
        */
    }
}

impl<'a> AudioIODeviceCallback for LiveScrollingAudioDisplay<'a> {

    fn audio_device_about_to_start(&mut self, _0: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            clear();
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            clear();
        */
    }
    
    fn audio_device_io_callback(
        &mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        number_of_samples:   i32
    ) {
        
        todo!();
        /*
            for (int i = 0; i < numberOfSamples; ++i)
            {
                float inputSample = 0;

                for (int chan = 0; chan < numInputChannels; ++chan)
                    if (const float* inputChannel = inputChannelData[chan])
                        inputSample += inputChannel[i];  // find the sum of all the channels

                inputSample *= 10.0f; // boost the level to make it more easily visible.

                pushSample (&inputSample, 1);
            }

            // We need to clear the output buffers before returning, in case they're full of junk..
            for (int j = 0; j < numOutputChannels; ++j)
                if (float* outputChannel = outputChannelData[j])
                    zeromem (outputChannel, (size_t) numberOfSamples * sizeof (float));
        */
    }
}
