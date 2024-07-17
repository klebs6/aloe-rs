crate::ix!();

pub struct IOChannelData {
    inputs:     Box<IOSAudioIODevicePimplIOChannelDataIOChannelConfig>,
    outputs:    Box<IOSAudioIODevicePimplIOChannelDataIOChannelConfig>,
    audio_data: AudioBuffer<f32>, // { 0, 0 };
}

impl IOChannelData {

    pub fn reconfigure(&mut self, 
        required_input_channels:  BigInteger,
        required_output_channels: BigInteger)  {
        
        todo!();
        /*
            inputs .reset (new IOSAudioIODevicePimplIOChannelDataIOChannelConfig (true,  requiredInputChannels));
                outputs.reset (new IOSAudioIODevicePimplIOChannelDataIOChannelConfig (false, requiredOutputChannels));

                audioData.setSize (inputs->numActiveChannels + outputs->numActiveChannels,
                                   audioData.getNumSamples());
        */
    }
    
    pub fn get_float_buffer_size(&self) -> i32 {
        
        todo!();
        /*
            return audioData.getNumSamples();
        */
    }
    
    pub fn set_float_buffer_size(&mut self, new_size: i32)  {
        
        todo!();
        /*
            audioData.setSize (audioData.getNumChannels(), newSize);
        */
    }
    
    pub fn are_input_channels_available(&self) -> bool {
        
        todo!();
        /*
            return inputs->areChannelsAccessible && inputs->numActiveChannels > 0;
        */
    }
}
