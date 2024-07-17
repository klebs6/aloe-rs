crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_android_Audio.cpp]

pub const CHANNEL_OUT_STEREO:  usize = 12;
pub const CHANNEL_IN_STEREO:   usize = 12;
pub const CHANNEL_IN_MONO:     usize = 16;
pub const ENCODING_PCM_16BIT:  usize = 2;
pub const STREAM_MUSIC:        usize = 3;
pub const MODE_STREAM:         usize = 1;
pub const STATE_UNINITIALIZED: usize = 0;

pub const JAVA_AUDIO_TYPE_NAME: &'static str = "Android Audio";

#[cfg(target_os="android")]
#[no_copy]
pub struct AndroidAudioIODevice {
    base:                                AudioIODevice,
    base2:                               Thread,
    min_buffer_size_out:                 i32,
    min_buffer_size_in:                  i32,
    callback_lock:                       CriticalSection,
    callback:                            *mut dyn AudioIODeviceCallback,
    sample_rate:                         i32,
    num_client_input_channels:           i32,
    num_device_input_channels:           i32,
    num_device_input_channels_available: i32,
    num_client_output_channels:          i32,
    num_device_output_channels:          i32,
    actual_buffer_size:                  i32,
    is_running:                          bool,
    last_error:                          String,
    active_output_chans:                 BigInteger,
    active_input_chans:                  BigInteger,
    output_device:                       GlobalRef,
    input_device:                        GlobalRef,
    input_channel_buffer:                AudioBuffer<f32>,
    output_channel_buffer:               AudioBuffer<f32>,
    get_underrun_count:                  jmethodID, // default = nullptr
}

#[cfg(target_os="android")]
impl Drop for AndroidAudioIODevice {

    fn drop(&mut self) {
        todo!();
        /* 
            close();
         */
    }
}

#[cfg(target_os="android")]
impl AndroidAudioIODevice {

    pub fn new(device_name: &String) -> Self {
    
        todo!();
        /*
        : audio_io_device(deviceName, javaAudioTypeName),
        : thread("audio"),
        : min_buffer_size_out(0),
        : min_buffer_size_in(0),
        : callback(nullptr),
        : sample_rate(0),
        : num_client_input_channels(0),
        : num_device_input_channels(0),
        : num_device_input_channels_available(2),
        : num_client_output_channels(0),
        : num_device_output_channels(0),
        : actual_buffer_size(0),
        : is_running(false),
        : input_channel_buffer(1, 1),
        : output_channel_buffer(1, 1),

            JNIEnv* env = getEnv();
            sampleRate = env->CallStaticIntMethod (AudioTrack, AudioTrack.getNativeOutputSampleRate, MODE_STREAM);

            minBufferSizeOut = (int) env->CallStaticIntMethod (AudioTrack,  AudioTrack.getMinBufferSize,  sampleRate, CHANNEL_OUT_STEREO, ENCODING_PCM_16BIT);
            minBufferSizeIn  = (int) env->CallStaticIntMethod (AudioRecord, AudioRecord.getMinBufferSize, sampleRate, CHANNEL_IN_STEREO,  ENCODING_PCM_16BIT);

            if (minBufferSizeIn <= 0)
            {
                minBufferSizeIn = env->CallStaticIntMethod (AudioRecord, AudioRecord.getMinBufferSize, sampleRate, CHANNEL_IN_MONO, ENCODING_PCM_16BIT);

                if (minBufferSizeIn > 0)
                    numDeviceInputChannelsAvailable = 1;
                else
                    numDeviceInputChannelsAvailable = 0;
            }

            DBG ("Audio device - min buffers: " << minBufferSizeOut << ", " << minBufferSizeIn << "; "
                  << sampleRate << " Hz; input chans: " << numDeviceInputChannelsAvailable);
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> s;
            s.add ("Left");
            s.add ("Right");
            return s;
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> s;

            if (numDeviceInputChannelsAvailable == 2)
            {
                s.add ("Left");
                s.add ("Right");
            }
            else if (numDeviceInputChannelsAvailable == 1)
            {
                s.add ("Audio Input");
            }

            return s;
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            Vec<double> r;
            r.add ((double) sampleRate);
            return r;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> b;
            int n = 16;

            for (int i = 0; i < 50; ++i)
            {
                b.add (n);
                n += n < 64 ? 16
                            : (n < 512 ? 32
                                       : (n < 1024 ? 64
                                                   : (n < 2048 ? 128 : 256)));
            }

            return b;
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return 2048;
        */
    }
    
    pub fn open(&mut self, 
        input_channels:        &BigInteger,
        output_channels:       &BigInteger,
        requested_sample_rate: f64,
        buffer_size:           i32) -> String {
        
        todo!();
        /*
            close();

            if (sampleRate != (int) requestedSampleRate)
                return "Sample rate not allowed";

            lastError.clear();
            int preferredBufferSize = (bufferSize <= 0) ? getDefaultBufferSize() : bufferSize;

            numDeviceInputChannels = 0;
            numDeviceOutputChannels = 0;

            activeOutputChans = outputChannels;
            activeOutputChans.setRange (2, activeOutputChans.getHighestBit(), false);
            numClientOutputChannels = activeOutputChans.countNumberOfSetBits();

            activeInputChans = inputChannels;
            activeInputChans.setRange (2, activeInputChans.getHighestBit(), false);
            numClientInputChannels = activeInputChans.countNumberOfSetBits();

            actualBufferSize = preferredBufferSize;
            inputChannelBuffer.setSize (2, actualBufferSize);
            inputChannelBuffer.clear();
            outputChannelBuffer.setSize (2, actualBufferSize);
            outputChannelBuffer.clear();

            JNIEnv* env = getEnv();

            if (numClientOutputChannels > 0)
            {
                numDeviceOutputChannels = 2;
                outputDevice = GlobalRef (LocalRef<jobject>(env->NewObject (AudioTrack, AudioTrack.constructor,
                                                                            STREAM_MUSIC, sampleRate, CHANNEL_OUT_STEREO, ENCODING_PCM_16BIT,
                                                                            (jint) (minBufferSizeOut * numDeviceOutputChannels * static_cast<int> (sizeof (int16))), MODE_STREAM)));

                const bool supportsUnderrunCount = (getAndroidSDKVersion() >= 24);
                getUnderrunCount = supportsUnderrunCount ? env->GetMethodID (AudioTrack, "getUnderrunCount", "()I") : nullptr;

                int outputDeviceState = env->CallIntMethod (outputDevice, AudioTrack.getState);
                if (outputDeviceState > 0)
                {
                    isRunning = true;
                }
                else
                {
                     // failed to open the device
                    outputDevice.clear();
                    lastError = "Error opening audio output device: android.media.AudioTrack failed with state = " + String (outputDeviceState);
                }
            }

            if (numClientInputChannels > 0 && numDeviceInputChannelsAvailable > 0)
            {
                if (! RuntimePermissions::isGranted (RuntimePermissions::recordAudio))
                {
                    // If you hit this assert, you probably forgot to get RuntimePermissions::recordAudio
                    // before trying to open an audio input device. This is not going to work!
                    jassertfalse;

                    inputDevice.clear();
                    lastError = "Error opening audio input device: the app was not granted android.permission.RECORD_AUDIO";
                }
                else
                {
                    numDeviceInputChannels = jmin (numClientInputChannels, numDeviceInputChannelsAvailable);
                    inputDevice = GlobalRef (LocalRef<jobject>(env->NewObject (AudioRecord, AudioRecord.constructor,
                                                                               0 /* (default audio source) */, sampleRate,
                                                                               numDeviceInputChannelsAvailable > 1 ? CHANNEL_IN_STEREO : CHANNEL_IN_MONO,
                                                                               ENCODING_PCM_16BIT,
                                                                               (jint) (minBufferSizeIn * numDeviceInputChannels * static_cast<int> (sizeof (int16))))));

                    int inputDeviceState = env->CallIntMethod (inputDevice, AudioRecord.getState);
                    if (inputDeviceState > 0)
                    {
                        isRunning = true;
                    }
                    else
                    {
                         // failed to open the device
                        inputDevice.clear();
                        lastError = "Error opening audio input device: android.media.AudioRecord failed with state = " + String (inputDeviceState);
                    }
                }
            }

            if (isRunning)
            {
                if (outputDevice != nullptr)
                    env->CallVoidMethod (outputDevice, AudioTrack.play);

                if (inputDevice != nullptr)
                    env->CallVoidMethod (inputDevice, AudioRecord.startRecording);

                startThread (8);
            }
            else
            {
                closeDevices();
            }

            return lastError;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (isRunning)
            {
                stopThread (2000);
                isRunning = false;
                closeDevices();
            }
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return (minBufferSizeOut * 3) / 4;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return (minBufferSizeIn * 3) / 4;
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return isRunning;
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return actualBufferSize;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return 16;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return sampleRate;
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return activeOutputChans;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return activeInputChans;
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return lastError;
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return isRunning && callback != nullptr;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            if (outputDevice != nullptr && getUnderrunCount != nullptr)
                return getEnv()->CallIntMethod (outputDevice, getUnderrunCount);

            return -1;
        */
    }
    
    pub fn start(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (isRunning && callback != newCallback)
            {
                if (newCallback != nullptr)
                    newCallback->audioDeviceAboutToStart (this);

                const ScopedLock sl (callbackLock);
                callback = newCallback;
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            if (isRunning)
            {
                AudioIODeviceCallback* lastCallback;

                {
                    const ScopedLock sl (callbackLock);
                    lastCallback = callback;
                    callback = nullptr;
                }

                if (lastCallback != nullptr)
                    lastCallback->audioDeviceStopped();
            }
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            JNIEnv* env = getEnv();
            jshortArray audioBuffer = env->NewShortArray (actualBufferSize * jmax (numDeviceOutputChannels, numDeviceInputChannels));

            while (! threadShouldExit())
            {
                if (inputDevice != nullptr)
                {
                    jint numRead = env->CallIntMethod (inputDevice, AudioRecord.read, audioBuffer, 0, actualBufferSize * numDeviceInputChannels);

                    if (numRead < actualBufferSize * numDeviceInputChannels)
                    {
                        DBG ("Audio read under-run! " << numRead);
                    }

                    jshort* const src = env->GetShortArrayElements (audioBuffer, nullptr);

                    for (int chan = 0; chan < inputChannelBuffer.getNumChannels(); ++chan)
                    {
                        AudioData::Pointer <AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::NonConst> d (inputChannelBuffer.getWritePointer (chan));

                        if (chan < numDeviceInputChannels)
                        {
                            AudioData::Pointer <AudioData::Int16, AudioData::NativeEndian, AudioData::Interleaved, AudioData::Const> s (src + chan, numDeviceInputChannels);
                            d.convertSamples (s, actualBufferSize);
                        }
                        else
                        {
                            d.clearSamples (actualBufferSize);
                        }
                    }

                    env->ReleaseShortArrayElements (audioBuffer, src, 0);
                }

                if (threadShouldExit())
                    break;

                {
                    const ScopedLock sl (callbackLock);

                    if (callback != nullptr)
                    {
                        callback->audioDeviceIOCallback (inputChannelBuffer.getArrayOfReadPointers(), numClientInputChannels,
                                                         outputChannelBuffer.getArrayOfWritePointers(), numClientOutputChannels,
                                                         actualBufferSize);
                    }
                    else
                    {
                        outputChannelBuffer.clear();
                    }
                }

                if (outputDevice != nullptr)
                {
                    if (threadShouldExit())
                        break;

                    jshort* const dest = env->GetShortArrayElements (audioBuffer, nullptr);

                    for (int chan = 0; chan < numDeviceOutputChannels; ++chan)
                    {
                        AudioData::Pointer <AudioData::Int16, AudioData::NativeEndian, AudioData::Interleaved, AudioData::NonConst> d (dest + chan, numDeviceOutputChannels);

                        const float* const sourceChanData = outputChannelBuffer.getReadPointer (jmin (chan, outputChannelBuffer.getNumChannels() - 1));
                        AudioData::Pointer <AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::Const> s (sourceChanData);
                        d.convertSamples (s, actualBufferSize);
                    }

                    env->ReleaseShortArrayElements (audioBuffer, dest, 0);
                    jint numWritten = env->CallIntMethod (outputDevice, AudioTrack.write, audioBuffer, 0, actualBufferSize * numDeviceOutputChannels);

                    if (numWritten < actualBufferSize * numDeviceOutputChannels)
                    {
                        DBG ("Audio write underrun! " << numWritten);
                    }
                }
            }
        */
    }
    
    pub fn close_devices(&mut self)  {
        
        todo!();
        /*
            if (outputDevice != nullptr)
            {
                outputDevice.callVoidMethod (AudioTrack.stop);
                outputDevice.callVoidMethod (AudioTrack.release);
                outputDevice.clear();
            }

            if (inputDevice != nullptr)
            {
                inputDevice.callVoidMethod (AudioRecord.stop);
                inputDevice.callVoidMethod (AudioRecord.release);
                inputDevice.clear();
            }
        */
    }
}
