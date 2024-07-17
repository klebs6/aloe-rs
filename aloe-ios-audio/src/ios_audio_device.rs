crate::ix!();

pub const iOSAudioDeviceName: &'static str = "iOS Audio";

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_ios_Audio.cpp]

#[no_copy]
pub struct iOSAudioIODevice<'a> {
    base:  AudioIODevice,
    pimpl: Box<iOSAudioIODevicePimpl<'a>>,
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
          pimpl (new Pimpl (ioDeviceType, *this))
        */
    }
    
    pub fn open(&mut self, 
        in_chans:              &BigInteger,
        out_chans:             &BigInteger,
        requested_sample_rate: f64,
        requested_buffer_size: i32) -> String {
        
        todo!();
        /*
            return pimpl->open (inChans, outChans, requestedSampleRate, requestedBufferSize);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            pimpl->close();
        */
    }
    
    pub fn start(&mut self, callback_to_use: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            pimpl->start (callbackToUse);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            pimpl->stop();
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            return pimpl->availableSampleRates;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            return pimpl->availableBufferSizes;
        */
    }
    
    pub fn set_audio_preprocessing_enabled(&mut self, enabled: bool) -> bool {
        
        todo!();
        /*
            return pimpl->setAudioPreprocessingEnabled (enabled);
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return pimpl->isRunning && pimpl->callback != nullptr;
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return pimpl->isRunning;
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return pimpl->lastError;
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return pimpl->channelData.outputs->hardwareChannelNames;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return pimpl->channelData.inputs->areChannelsAccessible ? pimpl->channelData.inputs->hardwareChannelNames : Vec<String>();
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return pimpl->defaultBufferSize;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return pimpl->bufferSize;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return pimpl->sampleRate;
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
            return pimpl->channelData.inputs->activeChannels;
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return pimpl->channelData.outputs->activeChannels;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return roundToInt (pimpl->sampleRate * [AVAudioSession sharedInstance].inputLatency);
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return roundToInt (pimpl->sampleRate * [AVAudioSession sharedInstance].outputLatency);
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->xrun;
        */
    }
    
    pub fn set_midi_message_collector(&mut self, collector: *mut MidiMessageCollector)  {
        
        todo!();
        /*
            pimpl->messageCollector = collector;
        */
    }
    
    pub fn get_audio_play_head(&self) -> *mut dyn AudioPlayHeadInterface {
        
        todo!();
        /*
            return pimpl.get();
        */
    }
    
    pub fn is_inter_app_audio_connected(&self) -> bool {
        
        todo!();
        /*
            return pimpl->interAppAudioConnected;
        */
    }

    #[cfg(ALOE_MODULE_AVAILABLE_aloe_graphics)]
    pub fn get_icon(&mut self, size: i32) -> Image {
        
        todo!();
        /*
            return pimpl->getIcon (size);
        */
    }
    
    pub fn switch_application(&mut self)  {
        
        todo!();
        /*
            return pimpl->switchApplication();
        */
    }
    
}

