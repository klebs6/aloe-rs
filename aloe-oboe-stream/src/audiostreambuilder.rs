crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/AudioStreamBuilder.cpp]

lazy_static!{
    /*
    bool OboeOboeGlobals::mWorkaroundsEnabled = true;
    */
}

/**
  | The following default values are used
  | when oboe does not have any better way
  | of determining the optimal values for
  | an audio stream. This can happen when:
  | 
  | - Client is creating a stream on API <
  | 26 (OpenSLES) but has not supplied the
  | optimal sample rate and/or frames per
  | burst
  | 
  | - Client is creating a stream on API 16
  | (OpenSLES) where AudioManager.PROPERTY_OUTPUT_*
  | values are not available
  |
  */
lazy_static!{
    /*
    int32_t DefaultStreamValues::SampleRate = 48000; // Common rate for mobile audio and video
        int32_t DefaultStreamValues::FramesPerBurst = 192; // 4 msec at 48000 Hz
        int32_t DefaultStreamValues::ChannelCount = 2; // Stereo
    */
}

pub const kBufferSizeInBurstsForLowLatencyStreams: i32 = 2;

/**
  | Set OBOE_ENABLE_AAUDIO to 0 if you want to
  | disable the AAudio API.
  |
  | This might be useful if you want to force all
  | the unit tests to use OpenSL ES.
  */
#[cfg(not(OBOE_ENABLE_AAUDIO))]
pub const OBOE_ENABLE_AAUDIO: usize = 1;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/AudioStreamBuilder.h]

pub type ManagedStream = Box<AudioStream,StreamDeleterFunctor>;

/**
  | Factory class for an audio Stream.
  |
  */
pub struct AudioStreamBuilder {
    base:      AudioStreamBase,
    audio_api: OboeAudioApi, // default = AudioApi::Unspecified
}

impl Default for AudioStreamBuilder {
    
    fn default() -> Self {
        todo!();
        /*
        : audio_stream_base(),

        
        */
    }
}

impl AudioStreamBuilder {

    pub fn new(audio_stream_base: &AudioStreamBase) -> Self {
    
        todo!();
        /*
        : audio_stream_base(audioStreamBase),

        
        */
    }

    /**
      | Request a specific number of channels.
      | 
      | Default is kUnspecified. If the value
      | is unspecified then the application
      | should query for the actual value after
      | the stream is opened.
      |
      */
    pub fn set_channel_count(&mut self, channel_count: i32) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mChannelCount = channelCount;
            return this;
        */
    }

    /**
      | Request the direction for a stream.
      | The default is Direction::Output.
      | 
      | -----------
      | @param direction
      | 
      | Direction::Output or Direction::Input
      |
      */
    pub fn set_direction(&mut self, direction: OboeDirection) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mDirection = direction;
            return this;
        */
    }

    /**
      | Request a specific sample rate in Hz.
      | 
      | Default is kUnspecified. If the value
      | is unspecified then the application
      | should query for the actual value after
      | the stream is opened.
      | 
      | Technically, this should be called
      | the "frame rate" or "frames per second",
      | because it refers to the number of complete
      | frames transferred per second. But
      | it is traditionally called "sample
      | rate". Se we use that term.
      |
      */
    pub fn set_sample_rate(&mut self, sample_rate: i32) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mSampleRate = sampleRate;
            return this;
        */
    }

    /**
      | @deprecated use `setFramesPerDataCallback`
      | instead.
      |
      */
    pub fn set_frames_per_callback(&mut self, frames_per_callback: i32) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            return setFramesPerDataCallback(framesPerCallback);
        */
    }

    /**
      | Request a specific number of frames
      | for the data callback.
      | 
      | Default is kUnspecified. If the value
      | is unspecified then the actual number
      | may vary from callback to callback.
      | 
      | If an application can handle a varying
      | number of frames then we recommend leaving
      | this unspecified. This allow the underlying
      | API to optimize the callbacks. But if
      | your application is, for example, doing
      | FFTs or other block oriented operations,
      | then call this function to get the sizes
      | you need.
      | 
      | -----------
      | @param framesPerCallback
      | 
      | @return pointer to the builder so calls
      | can be chained
      |
      */
    pub fn set_frames_per_data_callback(&mut self, frames_per_callback: i32) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mFramesPerCallback = framesPerCallback;
            return this;
        */
    }

    /**
      | Request a sample data format, for example
      | Format::Float.
      | 
      | Default is Format::Unspecified. If
      | the value is unspecified then the application
      | should query for the actual value after
      | the stream is opened.
      |
      */
    pub fn set_format(&mut self, format: AudioFormat) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mFormat = format;
            return this;
        */
    }

    /**
      | Set the requested buffer capacity in
      | frames. BufferCapacityInFrames is
      | the maximum possible BufferSizeInFrames.
      | 
      | The final stream capacity may differ.
      | For AAudio it should be at least this
      | big. For OpenSL ES, it could be smaller.
      | 
      | Default is kUnspecified.
      | 
      | -----------
      | @param bufferCapacityInFrames
      | 
      | the desired buffer capacity in frames
      | or kUnspecified @return pointer to
      | the builder so calls can be chained
      |
      */
    pub fn set_buffer_capacity_in_frames(&mut self, buffer_capacity_in_frames: i32) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mBufferCapacityInFrames = bufferCapacityInFrames;
            return this;
        */
    }

    /**
      | Get the audio API which will be requested
      | when opening the stream. No guarantees
      | that this is the API which will actually
      | be used. Query the stream itself to find
      | out the API which is being used.
      | 
      | If you do not specify the API, then AAudio
      | will be used if isAAudioRecommended()
      | returns true. Otherwise OpenSL ES will
      | be used.
      | 
      | -----------
      | @return
      | 
      | the requested audio API
      |
      */
    pub fn get_audio_api(&self) -> OboeAudioApi {
        
        todo!();
        /*
            return mAudioApi;
        */
    }

    /**
      | If you leave this unspecified then Oboe
      | will choose the best API for the device
      | and SDK version at runtime.
      | 
      | This should almost always be left unspecified,
      | except for debugging purposes. Specifying
      | AAudio will force Oboe to use AAudio
      | on 8.0, which is extremely risky. Specifying
      | OpenSLES should mainly be used to test
      | legacy performance/functionality.
      | 
      | If the caller requests AAudio and it
      | is supported then AAudio will be used.
      | 
      | -----------
      | @param audioApi
      | 
      | Must be AudioApi::Unspecified, AudioApi::OpenSLES
      | or AudioApi::AAudio. @return pointer
      | to the builder so calls can be chained
      |
      */
    pub fn set_audio_api(&mut self, audio_api: OboeAudioApi) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mAudioApi = audioApi;
            return this;
        */
    }

    /**
      | Is the AAudio API recommended this device?
      | 
      | AAudio may be supported but not recommended
      | because of version specific issues.
      | AAudio is not recommended for Android
      | 8.0 or earlier versions.
      | 
      | -----------
      | @return
      | 
      | true if recommended
      |
      */
    pub fn isa_audio_recommended() -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Request a mode for sharing the device.
      | The requested sharing mode may not be
      | available. So the application should
      | query for the actual mode after the stream
      | is opened.
      | 
      | -----------
      | @param sharingMode
      | 
      | SharingMode::Shared or SharingMode::Exclusive
      | @return pointer to the builder so calls
      | can be chained
      |
      */
    pub fn set_sharing_mode(&mut self, sharing_mode: OboeSharingMode) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mSharingMode = sharingMode;
            return this;
        */
    }

    /**
      | Request a performance level for the
      | stream. This will determine the latency,
      | the power consumption, and the level
      | of protection from glitches.
      | 
      | -----------
      | @param performanceMode
      | 
      | for example, PerformanceMode::LowLatency
      | @return pointer to the builder so calls
      | can be chained
      |
      */
    pub fn set_performance_mode(&mut self, performance_mode: OboePerformanceMode) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mPerformanceMode = performanceMode;
            return this;
        */
    }

    /**
      | Set the intended use case for an output
      | stream.
      | 
      | The system will use this information
      | to optimize the behavior of the stream.
      | This could, for example, affect how
      | volume and focus is handled for the stream.
      | The usage is ignored for input streams.
      | 
      | The default, if you do not call this function,
      | is Usage::Media.
      | 
      | Added in API level 28.
      | 
      | -----------
      | @param usage
      | 
      | the desired usage, eg. Usage::Game
      |
      */
    pub fn set_usage(&mut self, usage: OboeUsage) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mUsage = usage;
            return this;
        */
    }

    /**
      | Set the type of audio data that an output
      | stream will carry.
      | 
      | The system will use this information
      | to optimize the behavior of the stream.
      | This could, for example, affect whether
      | a stream is paused when a notification
      | occurs. The contentType is ignored
      | for input streams.
      | 
      | The default, if you do not call this function,
      | is ContentType::Music.
      | 
      | Added in API level 28.
      | 
      | -----------
      | @param contentType
      | 
      | the type of audio data, eg. ContentType::Speech
      |
      */
    pub fn set_content_type(&mut self, content_type: OboeContentType) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mContentType = contentType;
            return this;
        */
    }

    /**
      | Set the input (capture) preset for the
      | stream.
      | 
      | The system will use this information
      | to optimize the behavior of the stream.
      | This could, for example, affect which
      | microphones are used and how the recorded
      | data is processed.
      | 
      | The default, if you do not call this function,
      | is InputPreset::VoiceRecognition.
      | That is because VoiceRecognition is
      | the preset with the lowest latency on
      | many platforms.
      | 
      | Added in API level 28.
      | 
      | -----------
      | @param inputPreset
      | 
      | the desired configuration for recording
      |
      */
    pub fn set_input_preset(&mut self, input_preset: OboeInputPreset) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mInputPreset = inputPreset;
            return this;
        */
    }

    /**
      | Set the requested session ID.
      | 
      | The session ID can be used to associate
      | a stream with effects processors. The
      | effects are controlled using the Android
      | AudioEffect Java API.
      | 
      | The default, if you do not call this function,
      | is SessionId::None.
      | 
      | If set to SessionId::Allocate then
      | a session ID will be allocated when the
      | stream is opened.
      | 
      | The allocated session ID can be obtained
      | by calling AudioStream::getSessionId()
      | and then used with this function when
      | opening another stream. This allows
      | effects to be shared between streams.
      | 
      | Session IDs from Oboe can be used the
      | Android Java APIs and vice versa. So
      | a session ID from an Oboe stream can be
      | passed to Java and effects applied using
      | the Java AudioEffect API.
      | 
      | Allocated session IDs will always be
      | positive and nonzero.
      | 
      | Added in API level 28.
      | 
      | -----------
      | @param sessionId
      | 
      | an allocated sessionID or SessionId::Allocate
      |
      */
    pub fn set_session_id(&mut self, session_id: OboeSessionId) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mSessionId = sessionId;
            return this;
        */
    }

    /**
      | Request a stream to a specific audio
      | input/output device given an audio
      | device ID.
      | 
      | In most cases, the primary device will
      | be the appropriate device to use, and
      | the deviceId can be left kUnspecified.
      | 
      | On Android, for example, the ID could
      | be obtained from the Java AudioManager.
      | AudioManager.getDevices() returns
      | an array of AudioDeviceInfo[], which
      | contains a getId() method (as well as
      | other type information), that should
      | be passed to this method.
      | 
      | Note that when using OpenSL ES, this
      | will be ignored and the created stream
      | will have deviceId kUnspecified.
      | 
      | -----------
      | @param deviceId
      | 
      | device identifier or kUnspecified
      | @return pointer to the builder so calls
      | can be chained
      |
      */
    pub fn set_device_id(&mut self, device_id: i32) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mDeviceId = deviceId;
            return this;
        */
    }

    /**
      | Specifies an object to handle data related
      | callbacks from the underlying API.
      | 
      | <strong>Important: See AudioStreamCallback
      | for restrictions on what may be called
      | from the callback methods.</strong>
      | 
      | -----------
      | @param dataCallback
      | 
      | @return pointer to the builder so calls
      | can be chained
      |
      */
    pub fn set_data_callback(&mut self, data_callback: *mut dyn AudioStreamDataCallback) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mDataCallback = dataCallback;
            return this;
        */
    }

    /**
      | Specifies an object to handle error
      | related callbacks from the underlying
      | API. This can occur when a stream is disconnected
      | because a headset is plugged in or unplugged.
      | It can also occur if the audio service
      | fails or if an exclusive stream is stolen
      | by another stream.
      | 
      | <strong>Important: See AudioStreamCallback
      | for restrictions on what may be called
      | from the callback methods.</strong>
      | 
      | <strong>When an error callback occurs,
      | the associated stream must be stopped
      | and closed in a separate thread.</strong>
      | 
      | -----------
      | @param errorCallback
      | 
      | @return pointer to the builder so calls
      | can be chained
      |
      */
    pub fn set_error_callback(&mut self, error_callback: *mut dyn AudioStreamErrorCallback) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mErrorCallback = errorCallback;
            return this;
        */
    }

    /**
      | Specifies an object to handle data or
      | error related callbacks from the underlying
      | API.
      | 
      | This is the equivalent of calling both
      | setDataCallback() and setErrorCallback().
      | 
      | <strong>Important: See AudioStreamCallback
      | for restrictions on what may be called
      | from the callback methods.</strong>
      | 
      | When an error callback occurs, the associated
      | stream will be stopped and closed in
      | a separate thread.
      | 
      | A note on why the streamCallback parameter
      | is a raw pointer rather than a smart pointer:
      | 
      | The caller should retain ownership
      | of the object streamCallback points
      | to. At first glance weak_ptr may seem
      | like a good candidate for streamCallback
      | as this implies temporary ownership.
      | However, a weak_ptr can only be created
      | from a shared_ptr. A shared_ptr incurs
      | some performance overhead. The callback
      | object is likely to be accessed every
      | few milliseconds when the stream requires
      | new data so this overhead is something
      | we want to avoid.
      | 
      | This leaves a raw pointer as the logical
      | type choice. The only caveat being that
      | the caller must not destroy the callback
      | before the stream has been closed.
      | 
      | -----------
      | @param streamCallback
      | 
      | @return pointer to the builder so calls
      | can be chained
      |
      */
    pub fn set_callback(&mut self, stream_callback: *mut dyn AudioStreamCallback) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            // Use the same callback object for both, dual inheritance.
            mDataCallback = streamCallback;
            mErrorCallback = streamCallback;
            return this;
        */
    }

    /**
      | If true then Oboe might convert channel
      | counts to achieve optimal results.
      | On some versions of Android for example,
      | stereo streams could not use a FAST track.
      | So a mono stream might be used instead
      | and duplicated to two channels. On some
      | devices, mono streams might be broken,
      | so a stereo stream might be opened and
      | converted to mono.
      | 
      | Default is true.
      |
      */
    pub fn set_channel_conversion_allowed(&mut self, allowed: bool) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mChannelConversionAllowed = allowed;
            return this;
        */
    }

    /**
      | If true then Oboe might convert data
      | formats to achieve optimal results.
      | On some versions of Android, for example,
      | a float stream could not get a low latency
      | data path. So an I16 stream might be opened
      | and converted to float.
      | 
      | Default is false.
      |
      */
    pub fn set_format_conversion_allowed(&mut self, allowed: bool) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mFormatConversionAllowed = allowed;
            return this;
        */
    }

    /**
      | Specify the quality of the sample rate
      | converter in Oboe.
      | 
      | If set to None then Oboe will not do sample
      | rate conversion. But the underlying
      | APIs might still do sample rate conversion
      | if you specify a sample rate. That can
      | prevent you from getting a low latency
      | stream.
      | 
      | If you do the conversion in Oboe then
      | you might still get a low latency stream.
      | 
      | Default is SampleRateConversionQuality::None
      |
      */
    pub fn set_sample_rate_conversion_quality(&mut self, quality: OboeSampleRateConversionQuality) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mSampleRateConversionQuality = quality;
            return this;
        */
    }

    /**
      | Declare the name of the package creating
      | the stream.
      | 
      | This is usually Context#getPackageName()
      | 
      | The default, if you do not call this function,
      | is a random package in the calling uid.
      | 
      | Added in API level 31.
      | 
      | -----------
      | @param packageName
      | 
      | packageName of the calling app.
      |
      */
    pub fn set_package_name(&mut self, package_name: String) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mPackageName = packageName;
            return this;
        */
    }

    /**
      | Declare the attribution tag of the context
      | creating the stream.
      | 
      | This is usually Context#getAttributionTag()
      | 
      | The default, if you do not call this function,
      | is the default attribution tag.
      | 
      | Added in API level 31.
      | 
      | -----------
      | @param attributionTag
      | 
      | attributionTag of the calling context.
      |
      */
    pub fn set_attribution_tag(&mut self, attribution_tag: String) -> *mut AudioStreamBuilder {
        
        todo!();
        /*
            mAttributionTag = attributionTag;
            return this;
        */
    }

    /**
      | @return
      | 
      | true if AAudio will be used based on the
      | current settings.
      |
      */
    pub fn will_use_aaudio(&self) -> bool {
        
        todo!();
        /*
            return (mAudioApi == AudioApi::AAudio && isAAudioSupported())
                    || (mAudioApi == AudioApi::Unspecified && isAAudioRecommended());
        */
    }

    /**
      | Create and open a stream object based
      | on the current settings.
      | 
      | The caller shares the pointer to the
      | AudioStream object. The shared_ptr
      | is used internally by Oboe to prevent
      | the stream from being deleted while
      | it is being used by callbacks.
      | 
      | -----------
      | @param stream
      | 
      | reference to a shared_ptr to receive
      | the stream address @return OBOE_OK
      | if successful or a negative error code
      |
      */
    pub fn open_stream_shared(&mut self, stream: &mut Arc<OboeAudioStream>) -> OboeResult {
        
        todo!();
        /*
        
        */
    }

    /**
      | Is the AAudio API supported on this device?
      | 
      | AAudio was introduced in the Oreo 8.0
      | release.
      | 
      | -----------
      | @return
      | 
      | true if supported
      |
      */
    pub fn is_aaudio_supported(&mut self) -> bool {
        
        todo!();
        /*
            return AudioStreamAAudio::isSupported() && OBOE_ENABLE_AAUDIO;
        */
    }
    
    pub fn is_aaudio_recommended(&mut self) -> bool {
        
        todo!();
        /*
            // See https://github.com/google/oboe/issues/40,
        // AAudio may not be stable on Android O, depending on how it is used.
        // To be safe, use AAudio only on O_MR1 and above.
        return (getSdkVersion() >= __ANDROID_API_O_MR1__) && isAAudioSupported();
        */
    }
    
    /**
      | Create an AudioStream object. The AudioStream
      | must be opened before use.
      | 
      | The caller owns the pointer.
      | 
      | -----------
      | @return
      | 
      | pointer to an AudioStream object or
      | nullptr.
      |
      */
    pub fn build(&mut self) -> *mut AudioStream {
        
        todo!();
        /*
            AudioStream *stream = nullptr;
        if (isAAudioRecommended() && mAudioApi != AudioApi::OpenSLES) {
            stream = new AudioStreamAAudio(*this);
        } else if (isAAudioSupported() && mAudioApi == AudioApi::AAudio) {
            stream = new AudioStreamAAudio(*this);
            LOGE("Creating AAudio stream on 8.0 because it was specified. This is error prone.");
        } else {
            if (getDirection() == OboeDirection::Output) {
                stream = new AudioOutputStreamOpenSLES(*this);
            } else if (getDirection() == OboeDirection::Input) {
                stream = new AudioInputStreamOpenSLES(*this);
            }
        }
        return stream;
        */
    }
    
    /**
      | @param other
      | 
      | @return true if channels, format and
      | sample rate match
      |
      */
    pub fn is_compatible(&mut self, other: &mut AudioStreamBase) -> bool {
        
        todo!();
        /*
            return (getSampleRate() == OboeUnspecified || getSampleRate() == other.getSampleRate())
               && (getFormat() == (AudioFormat)OboeUnspecified || getFormat() == other.getFormat())
               && (getFramesPerDataCallback() == OboeUnspecified || getFramesPerDataCallback() == other.getFramesPerDataCallback())
               && (getChannelCount() == OboeUnspecified || getChannelCount() == other.getChannelCount());
        */
    }
    
    /**
      | Create and open a stream object based
      | on the current settings.
      | 
      | The caller owns the pointer to the AudioStream
      | object and must delete it when finished.
      | 
      | @deprecated Use openStream(std::shared_ptr<OboeAudioStream>
      | &stream) instead. @param stream pointer
      | to a variable to receive the stream address
      | @return OBOE_OK if successful or a negative
      | error code
      |
      */
    pub fn open_stream(&mut self, streampp: *mut *mut AudioStream) -> OboeResult {
        
        todo!();
        /*
            auto result = isValidConfig();
        if (result != OboeResult::OK) {
            LOGW("%s() invalid config %d", __func__, result);
            return result;
        }

        LOGI("%s() %s -------- %s --------",
             __func__, getDirection() == Direction::Input ? "INPUT" : "OUTPUT", getVersionText());

        if (streamPP == nullptr) {
            return OboeResult::ErrorNull;
        }
        *streamPP = nullptr;

        AudioStream *streamP = nullptr;

        // Maybe make a FilterInputStream.
        AudioStreamBuilder childBuilder(*this);
        // Check need for conversion and modify childBuilder for optimal stream.
        bool conversionNeeded = QuirksManager::getInstance().isConversionNeeded(*this, childBuilder);
        // Do we need to make a child stream and convert.
        if (conversionNeeded) {
            AudioStream *tempStream;
            result = childBuilder.openStream(&tempStream);
            if (result != OboeResult::OK) {
                return result;
            }

            if (isCompatible(*tempStream)) {
                // The child stream would work as the requested stream so we can just use it directly.
                *streamPP = tempStream;
                return result;
            } else {
                AudioStreamBuilder parentBuilder = *this;
                // Build a stream that is as close as possible to the childStream.
                if (getFormat() == OboeAudioFormat::Unspecified) {
                    parentBuilder.setFormat(tempStream->getFormat());
                }
                if (getChannelCount() == OboeUnspecified) {
                    parentBuilder.setChannelCount(tempStream->getChannelCount());
                }
                if (getSampleRate() == OboeUnspecified) {
                    parentBuilder.setSampleRate(tempStream->getSampleRate());
                }
                if (getFramesPerDataCallback() == OboeUnspecified) {
                    parentBuilder.setFramesPerCallback(tempStream->getFramesPerDataCallback());
                }

                // Use childStream in a FilterAudioStream.
                LOGI("%s() create a FilterAudioStream for data conversion.", __func__);
                FilterAudioStream *filterStream = new FilterAudioStream(parentBuilder, tempStream);
                result = filterStream->configureFlowGraph();
                if (result !=  OboeResult::OK) {
                    filterStream->close();
                    delete filterStream;
                    // Just open streamP the old way.
                } else {
                    streamP = static_cast<AudioStream *>(filterStream);
                }
            }
        }

        if (streamP == nullptr) {
            streamP = build();
            if (streamP == nullptr) {
                return OboeResult::ErrorNull;
            }
        }

        // If MMAP has a problem in this case then disable it temporarily.
        bool wasMMapOriginallyEnabled = AAudioExtensions::getInstance().isMMapEnabled();
        bool wasMMapTemporarilyDisabled = false;
        if (wasMMapOriginallyEnabled) {
            bool isMMapSafe = QuirksManager::getInstance().isMMapSafe(childBuilder);
            if (!isMMapSafe) {
                AAudioExtensions::getInstance().setMMapEnabled(false);
                wasMMapTemporarilyDisabled = true;
            }
        }
        result = streamP->open();
        if (wasMMapTemporarilyDisabled) {
            AAudioExtensions::getInstance().setMMapEnabled(wasMMapOriginallyEnabled); // restore original
        }
        if (result == OboeResult::OK) {

            int32_t  optimalBufferSize = -1;
            // Use a reasonable default buffer size.
            if (streamP->getDirection() == Direction::Input) {
                // For input, small size does not improve latency because the stream is usually
                // run close to empty. And a low size can result in XRuns so always use the maximum.
                optimalBufferSize = streamP->getBufferCapacityInFrames();
            } else if (streamP->getPerformanceMode() == PerformanceMode::LowLatency
                    && streamP->getDirection() == Direction::Output)  { // Output check is redundant.
                optimalBufferSize = streamP->getFramesPerBurst() *
                                        kBufferSizeInBurstsForLowLatencyStreams;
            }
            if (optimalBufferSize >= 0) {
                auto setBufferResult = streamP->setBufferSizeInFrames(optimalBufferSize);
                if (!setBufferResult) {
                    LOGW("Failed to setBufferSizeInFrames(%d). Error was %s",
                         optimalBufferSize,
                         convertToText(setBufferResult.error()));
                }
            }

            *streamPP = streamP;
        } else {
            delete streamP;
        }
        return result;
        */
    }
    
    /**
      | Create and open a ManagedStream object
      | based on the current builder state.
      | 
      | The caller must create a unique ptr,
      | and pass by reference so it can be modified
      | to point to an opened stream. The caller
      | owns the unique ptr, and it will be automatically
      | closed and deleted when going out of
      | scope.
      | 
      | @deprecated Use openStream(std::shared_ptr<OboeAudioStream>
      | &stream) instead. @param stream Reference
      | to the ManagedStream (uniqueptr) used
      | to keep track of stream @return OBOE_OK
      | if successful or a negative error code.
      |
      */
    pub fn open_managed_stream(&mut self, stream: &mut OboeManagedStream) -> OboeResult {
        
        todo!();
        /*
            stream.reset();
        AudioStream *streamptr;
        auto result = openStream(&streamptr);
        stream.reset(streamptr);
        return result;
        */
    }
    
    pub fn open_shared_stream(&mut self, shared_stream: &mut Arc<AudioStream>) -> OboeResult {
        
        todo!();
        /*
            sharedStream.reset();
        AudioStream *streamptr;
        auto result = openStream(&streamptr);
        if (result == OboeResult::OK) {
            sharedStream.reset(streamptr);
            // Save a weak_ptr in the stream for use with callbacks.
            streamptr->setWeakThis(sharedStream);
        }
        return result;
        */
    }
}
