crate::ix!();

pub struct ALSAAudioIODevice {
    base:       AudioIODevice,
    input_id:   String,
    output_id:  String,
    is_open:    bool, // default = false
    is_started: bool, // default = false
    internal:   ALSAThread,
}

impl Drop for ALSAAudioIODevice {

    fn drop(&mut self) {
        todo!();
        /* 
            close();
         */
    }
}

impl ALSAAudioIODevice {

    pub fn new(
        device_name:      &String,
        device_type_name: &String,
        input_deviceid:   &String,
        output_deviceid:  &String) -> Self {
    
        todo!();
        /*
        : audio_io_device(deviceName, deviceTypeName),
        : input_id(inputDeviceID),
        : output_id(outputDeviceID),
        : internal(inputDeviceID, outputDeviceID),

        
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return internal.channelNamesOut;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return internal.channelNamesIn;
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            return internal.sampleRates;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> r;
            int n = 16;

            for (int i = 0; i < 50; ++i)
            {
                r.add (n);
                n += n < 64 ? 16
                            : (n < 512 ? 32
                                       : (n < 1024 ? 64
                                                   : (n < 2048 ? 128 : 256)));
            }

            return r;
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return 512;
        */
    }
    
    pub fn open(&mut self, 
        input_channels:      &BigInteger,
        output_channels:     &BigInteger,
        sample_rate:         f64,
        buffer_size_samples: i32) -> String {
        
        todo!();
        /*
            close();

            if (bufferSizeSamples <= 0)
                bufferSizeSamples = getDefaultBufferSize();

            if (sampleRate <= 0)
            {
                for (int i = 0; i < internal.sampleRates.size(); ++i)
                {
                    double rate = internal.sampleRates[i];

                    if (rate >= 44100)
                    {
                        sampleRate = rate;
                        break;
                    }
                }
            }

            internal.open (inputChannels, outputChannels,
                           sampleRate, bufferSizeSamples);

            isOpen_ = internal.error.isEmpty();
            return internal.error;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();
            internal.close();
            isOpen_ = false;
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return isOpen_;
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return isStarted && internal.error.isEmpty();
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return internal.error;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return internal.bufferSize;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return internal.sampleRate;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return internal.getBitDepth();
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return internal.currentOutputChans;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return internal.currentInputChans;
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return internal.outputLatency;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return internal.inputLatency;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return internal.getXRunCount();
        */
    }
    
    pub fn start(&mut self, callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (! isOpen_)
                callback = nullptr;

            if (callback != nullptr)
                callback->audioDeviceAboutToStart (this);

            internal.setCallback (callback);

            isStarted = (callback != nullptr);
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            auto oldCallback = internal.callback;

            start (nullptr);

            if (oldCallback != nullptr)
                oldCallback->audioDeviceStopped();
        */
    }
}

