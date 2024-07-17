crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_linux_Bela.cpp]
//TODO
pub struct MissingSymbol {}
pub type BelaInitSettings = MissingSymbol;
pub type BelaContext      = MissingSymbol;

#[no_copy]
#[leak_detector]
pub struct BelaAudioIODevice {
    base:                           AudioIODevice,
    analog_channel_start:           i32, // default = 2
    expected_elapsed_audio_samples: u64, // default = 0
    underruns:                      i32, // default = 0
    first_callback:                 bool, // default = false
    default_settings:               BelaInitSettings,
    settings:                       BelaInitSettings,
    is_bela_open:                   bool, // default = false
    is_running:                     bool, // default = false
    callback_lock:                  CriticalSection,
    callback:                       *mut dyn AudioIODeviceCallback, // default = nullptr
    last_error:                     String,
    actual_buffer_size:             u32, // default = 0
    actual_number_of_inputs:        i32, // default = 0
    actual_number_of_outputs:       i32, // default = 0
    channel_in_buffer:              HeapBlock<*const f32>,
    channel_out_buffer:             HeapBlock<*mut f32>,
    include_analog_support:         bool,
}

pub mod bela_audio_io_device {
    use super::*;

    lazy_static!{
        /*
        static const char* const belaTypeName;
        const char* const BelaAudioIODevice::belaTypeName = "Bela Analog";
        */
    }
}

impl Default for BelaAudioIODevice {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioIODevice (BelaAudioIODevice::belaTypeName,
                                              BelaAudioIODevice::belaTypeName)
            Bela_defaultSettings (&defaultSettings)
        */
    }
}

impl Drop for BelaAudioIODevice {
    fn drop(&mut self) {
        todo!();
        /* 
            close();
         */
    }
}

impl BelaAudioIODevice {

    pub fn get_output_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> result;

            for (int i = 1; i <= actualNumberOfOutputs; i++)
                result.add ("Out #" + std::to_string (i));

            return result;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> result;

            for (int i = 1; i <= actualNumberOfInputs; i++)
                result.add ("In #" + std::to_string (i));

            return result;
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            return { 44100.0 };
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            /* TODO: */ return { getDefaultBufferSize() };
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return defaultSettings.periodSize;
        */
    }
    
    pub fn open(&mut self, 
        input_channels:      &BigInteger,
        output_channels:     &BigInteger,
        sample_rate:         f64,
        buffer_size_samples: i32) -> String {
        
        todo!();
        /*
            if (sampleRate != 44100.0 && sampleRate != 0.0)
            {
                lastError = "Bela audio outputs only support 44.1 kHz sample rate";
                return lastError;
            }

            settings = defaultSettings;

            auto numIns = getNumContiguousSetBits (inputChannels);
            auto numOuts = getNumContiguousSetBits (outputChannels);

            // Input and Output channels are numbered as follows
            //
            // 0  .. 1  - audio
            // 2  .. 9  - analog

            if (numIns > 2 || numOuts > 2)
            {
                settings.useAnalog            = true;
                settings.numAnalogInChannels  = std::max (numIns - 2, 8);
                settings.numAnalogOutChannels = std::max (numOuts - 2, 8);
                settings.uniformSampleRate    = true;
            }

            settings.numAudioInChannels   = std::max (numIns, 2);
            settings.numAudioOutChannels  = std::max (numOuts, 2);

            settings.detectUnderruns      = 1;
            settings.setup                = setupCallback;
            settings.render               = renderCallback;
            settings.cleanup              = cleanupCallback;
            settings.interleave           = 0;

            if (bufferSizeSamples > 0)
                settings.periodSize = bufferSizeSamples;

            isBelaOpen = false;
            isRunning  = false;
            callback   = nullptr;
            underruns  = 0;

            if (Bela_initAudio (&settings, this) != 0 || ! isBelaOpen)
            {
                lastError = "Bela_initAutio failed";
                return lastError;
            }

            actualNumberOfInputs  = jmin (numIns, actualNumberOfInputs);
            actualNumberOfOutputs = jmin (numOuts, actualNumberOfOutputs);

            channelInBuffer.calloc (actualNumberOfInputs);
            channelOutBuffer.calloc (actualNumberOfOutputs);

            return {};
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();

            if (isBelaOpen)
            {
                Bela_cleanupAudio();

                isBelaOpen = false;
                callback = nullptr;
                underruns = 0;

                actualBufferSize = 0;
                actualNumberOfInputs = 0;
                actualNumberOfOutputs = 0;

                channelInBuffer.free();
                channelOutBuffer.free();
            }
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return isBelaOpen;
        */
    }
    
    pub fn start(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (! isBelaOpen)
                return;

            if (isRunning)
            {
                if (newCallback != callback)
                {
                    if (newCallback != nullptr)
                        newCallback->audioDeviceAboutToStart (this);

                    {
                        ScopedLock lock (callbackLock);
                        std::swap (callback, newCallback);
                    }

                    if (newCallback != nullptr)
                        newCallback->audioDeviceStopped();
                }
            }
            else
            {
                callback = newCallback;
                isRunning = (Bela_startAudio() == 0);

                if (callback != nullptr)
                {
                    if (isRunning)
                    {
                        callback->audioDeviceAboutToStart (this);
                    }
                    else
                    {
                        lastError = "Bela_StartAudio failed";
                        callback->audioDeviceError (lastError);
                    }
                }
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            AudioIODeviceCallback* oldCallback = nullptr;

            if (callback != nullptr)
            {
                ScopedLock lock (callbackLock);
                std::swap (callback, oldCallback);
            }

            isRunning = false;
            Bela_stopAudio();

            if (oldCallback != nullptr)
                oldCallback->audioDeviceStopped();
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return isRunning;
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return lastError;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return (int) actualBufferSize;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return 44100.0;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return 16;
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            BigInteger b; b.setRange (0, actualNumberOfOutputs, true); return b;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            BigInteger b; b.setRange (0, actualNumberOfInputs, true);  return b;
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            /* TODO */ return 0;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            /* TODO */ return 0;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return underruns;
        */
    }
    
    pub fn setup(&mut self, context: &mut BelaContext) -> bool {
        
        todo!();
        /*
            actualBufferSize      = context.audioFrames;
            actualNumberOfInputs  = (int) (context.audioInChannels + context.analogInChannels);
            actualNumberOfOutputs = (int) (context.audioOutChannels + context.analogOutChannels);
            isBelaOpen = true;
            firstCallback = true;

            ScopedLock lock (callbackLock);

            if (callback != nullptr)
                callback->audioDeviceAboutToStart (this);

            return true;
        */
    }
    
    pub fn render(&mut self, context: &mut BelaContext)  {
        
        todo!();
        /*
            // check for xruns
            calculateXruns (context.audioFramesElapsed, context.audioFrames);

            ScopedLock lock (callbackLock);

            // Check for and process and midi
            for (auto midiInput : MidiInput::Pimpl::midiInputs)
                midiInput->poll();

            if (callback != nullptr)
            {
                jassert (context.audioFrames <= actualBufferSize);
                jassert ((context.flags & BELA_FLAG_INTERLEAVED) == 0);

                using Frames = decltype (context.audioFrames);

                // Setup channelInBuffers
                for (int ch = 0; ch < actualNumberOfInputs; ++ch)
                {
                    if (ch < analogChannelStart)
                        channelInBuffer[ch] = &context.audioIn[(Frames) ch * context.audioFrames];
                    else
                        channelInBuffer[ch] = &context.analogIn[(Frames) (ch - analogChannelStart) * context.analogFrames];
                }

                // Setup channelOutBuffers
                for (int ch = 0; ch < actualNumberOfOutputs; ++ch)
                {
                    if (ch < analogChannelStart)
                        channelOutBuffer[ch] = &context.audioOut[(Frames) ch * context.audioFrames];
                    else
                        channelOutBuffer[ch] = &context.analogOut[(Frames) (ch - analogChannelStart) * context.audioFrames];
                }

                callback->audioDeviceIOCallback (channelInBuffer.getData(), actualNumberOfInputs,
                                                 channelOutBuffer.getData(), actualNumberOfOutputs,
                                                 (int) context.audioFrames);
            }
        */
    }
    
    pub fn cleanup(&mut self, _0: &mut BelaContext)  {
        
        todo!();
        /*
            ScopedLock lock (callbackLock);

            if (callback != nullptr)
                callback->audioDeviceStopped();
        */
    }
    
    pub fn calculate_xruns(&mut self, 
        audio_frames_elapsed: u64,
        num_samples:          u32)  {
        
        todo!();
        /*
            if (audioFramesElapsed > expectedElapsedAudioSamples && ! firstCallback)
                ++underruns;

            firstCallback = false;
            expectedElapsedAudioSamples = audioFramesElapsed + numSamples;
        */
    }
    
    pub fn get_num_contiguous_set_bits(value: &BigInteger) -> i32 {
        
        todo!();
        /*
            int bit = 0;

            while (value[bit])
                ++bit;

            return bit;
        */
    }
    
    pub fn setup_callback(
        context:   *mut BelaContext,
        user_data: *mut c_void) -> bool {
        
        todo!();
        /*
            return static_cast<BelaAudioIODevice*> (userData)->setup (*context);
        */
    }
    
    pub fn render_callback(
        context:   *mut BelaContext,
        user_data: *mut c_void)  {
        
        todo!();
        /*
            static_cast<BelaAudioIODevice*> (userData)->render (*context);
        */
    }
    
    pub fn cleanup_callback(
        context:   *mut BelaContext,
        user_data: *mut c_void)  {
        
        todo!();
        /*
            static_cast<BelaAudioIODevice*> (userData)->cleanup (*context);
        */
    }
}
