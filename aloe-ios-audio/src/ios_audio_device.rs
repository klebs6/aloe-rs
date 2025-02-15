crate::ix!();

pub const iOSAudioDeviceName: &'static str = "iOS Audio";

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_ios_Audio.cpp]

#[no_copy]
pub struct iOSAudioIODevice<'a> {
    base:  AudioIODevice,
    impl_: Box<iOSAudioIODeviceImpl<'a>>,
}

impl<'a> Default for iOSAudioIODevice<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        : audio_io_device_type(iOSAudioDeviceName),

            sessionHolder->activeDeviceTypes.add (this);
        */
    }
}

impl<'a> iOSAudioIODevice<'a> {

    pub fn new(
        io_device_type: *mut iOSAudioIODeviceType,
        _1:             &str,
        _2:             &str) -> Self {
    
        todo!();
        /*


            : AudioIODevice (iOSAudioDeviceName, iOSAudioDeviceName),
          impl (new Impl (ioDeviceType, *this))
        */
    }
    
    pub fn open(&mut self, 
        in_chans:              &BigInteger,
        out_chans:             &BigInteger,
        requested_sample_rate: f64,
        requested_buffer_size: i32) -> String {
        
        todo!();
        /*
            return impl->open (inChans, outChans, requestedSampleRate, requestedBufferSize);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            impl->close();
        */
    }
    
    pub fn start(&mut self, callback_to_use: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            impl->start (callbackToUse);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            impl->stop();
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            return impl->availableSampleRates;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return impl->availableBufferSizes;
        */
    }
    
    pub fn set_audio_preprocessing_enabled(&mut self, enabled: bool) -> bool {
        
        todo!();
        /*
            return impl->setAudioPreprocessingEnabled (enabled);
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return impl->isRunning && impl->callback != nullptr;
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return impl->isRunning;
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return impl->lastError;
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return impl->channelData.outputs->hardwareChannelNames;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return impl->channelData.inputs->areChannelsAccessible ? impl->channelData.inputs->hardwareChannelNames : Vec<String>();
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return impl->defaultBufferSize;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return impl->bufferSize;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return impl->sampleRate;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return 16;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return impl->channelData.inputs->activeChannels;
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return impl->channelData.outputs->activeChannels;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return roundToInt (impl->sampleRate * [AVAudioSession sharedInstance].inputLatency);
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return roundToInt (impl->sampleRate * [AVAudioSession sharedInstance].outputLatency);
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return impl->xrun;
        */
    }
    
    pub fn set_midi_message_collector(&mut self, collector: *mut MidiMessageCollector)  {
        
        todo!();
        /*
            impl->messageCollector = collector;
        */
    }
    
    pub fn get_audio_play_head(&self) -> *mut dyn AudioPlayHeadInterface {
        
        todo!();
        /*
            return impl.get();
        */
    }
    
    pub fn is_inter_app_audio_connected(&self) -> bool {
        
        todo!();
        /*
            return impl->interAppAudioConnected;
        */
    }

    #[cfg(ALOE_MODULE_AVAILABLE_aloe_graphics)]
    pub fn get_icon(&mut self, size: i32) -> Image {
        
        todo!();
        /*
            return impl->getIcon (size);
        */
    }
    
    pub fn switch_application(&mut self)  {
        
        todo!();
        /*
            return impl->switchApplication();
        */
    }
    
}

