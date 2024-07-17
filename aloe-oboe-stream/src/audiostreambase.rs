crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/AudioStreamBase.h]

/**
  | Base class containing parameters for
  | audio streams and builders.
  |
  */
pub struct AudioStreamBase {

    /**
      | The callback which will be fired when
      | new data is ready to be read/written.
      |
      */
    data_callback:                  Option<Box<dyn AudioStreamDataCallback>>,

    /**
      | The callback which will be fired when
      | an error or a disconnect occurs. *
      |
      */
    error_callback:                 Option<Box<dyn AudioStreamErrorCallback>>,

    /**
      | Number of audio frames which will be
      | requested in each callback
      |
      */
    frames_per_callback:            i32, // default = kUnspecified

    /**
      | Stream channel count
      |
      */
    channel_count:                  i32, // default = kUnspecified

    /**
      | Stream sample rate
      |
      */
    sample_rate:                    i32, // default = kUnspecified

    /**
      | Stream audio device ID
      |
      */
    device_id:                      i32, // default = kUnspecified

    /**
      | Stream buffer capacity specified as
      | a number of audio frames
      |
      */
    buffer_capacity_in_frames:      i32, // default = kUnspecified

    /**
      | Stream buffer size specified as a number
      | of audio frames
      |
      */
    buffer_size_in_frames:          i32, // default = kUnspecified

    /**
      | Stream sharing mode
      |
      */
    sharing_mode:                   OboeSharingMode, // default = SharingMode::Shared

    /**
      | Format of audio frames
      |
      */
    format:                         Option<AudioFormat>, // default = AudioFormat::Unspecified

    /**
      | Stream direction
      |
      */
    direction:                      OboeDirection, // default = Direction::Output

    /**
      | Stream performance mode
      |
      */
    performance_mode:               OboePerformanceMode, // default = PerformanceMode::None

    /**
      | Stream usage. Only active on Android
      | 28+
      |
      */
    usage:                          OboeUsage, // default = Usage::Media

    /**
      | Stream content type. Only active on
      | Android 28+
      |
      */
    content_type:                   OboeContentType, // default = ContentType::Music

    /**
      | Stream input preset. Only active on Android 28+
      | TODO InputPreset::Unspecified should be considered
      | as a possible default alternative.
      */
    input_preset:                   OboeInputPreset, // default = InputPreset::VoiceRecognition

    /**
      | Stream session ID allocation strategy.
      | Only active on Android 28+
      |
      */
    session_id:                     OboeSessionId, // default = SessionId::None

    /**
      | Control the name of the package creating
      | the stream. Only active on Android 31+
      |
      */
    package_name:                   String,

    /**
      | Control the attribution tag of the context
      | creating the stream. Only active on
      | Android 31+
      |
      */
    attribution_tag:                String,

    /**
      | Control whether Oboe can convert channel
      | counts to achieve optimal results.
      |
      */
    channel_conversion_allowed:     bool, // default = false

    /**
      | Control whether Oboe can convert data
      | formats to achieve optimal results.
      |
      */
    format_conversion_allowed:      bool, // default = false

    /**
      | Control whether and how Oboe can convert
      | sample rates to achieve optimal results.
      |
      */
    sample_rate_conversion_quality: OboeSampleRateConversionQuality, // default = SampleRateConversionQuality::None

}

impl Default for AudioStreamBase {

    fn default() -> Self {
        Self {
            data_callback:                  None,
            error_callback:                 None,
            frames_per_callback:            kUnspecified,
            channel_count:                  kUnspecified,
            sample_rate:                    kUnspecified,
            device_id:                      kUnspecified,
            buffer_capacity_in_frames:      kUnspecified,
            buffer_size_in_frames:          kUnspecified,
            sharing_mode:                   OboeSharingMode::Shared,
            format:                         None,
            direction:                      OboeDirection::Output,
            performance_mode:               OboePerformanceMode::None,
            usage:                          OboeUsage::Media,
            content_type:                   OboeContentType::Music,
            input_preset:                   OboeInputPreset::VoiceRecognition,
            session_id:                     OboeSessionId::None, // default = SessionId::None
            package_name:                   String::new(),
            attribution_tag:                String::new(),
            channel_conversion_allowed:     false,
            format_conversion_allowed:      false,
            sample_rate_conversion_quality: OboeSampleRateConversionQuality::None,
        }
    }
}

impl AudioStreamBase {

    /*
       This class only contains primitives so we
       can use default constructor and copy
       methods.
      */

    /**
      | @return
      | 
      | number of channels, for example 2 for
      | stereo, or kUnspecified
      |
      */
    pub fn get_channel_count(&self) -> i32 {
        
        todo!();
        /*
            return mChannelCount;
        */
    }

    /**
      | @return
      | 
      | Direction::Input or Direction::Output
      |
      */
    pub fn get_direction(&self) -> OboeDirection {
        
        todo!();
        /*
            return mDirection;
        */
    }

    /**
      | @return
      | 
      | sample rate for the stream or kUnspecified
      |
      */
    pub fn get_sample_rate(&self) -> i32 {
        
        todo!();
        /*
            return mSampleRate;
        */
    }

    /**
      | @deprecated use `getFramesPerDataCallback`
      | instead.
      |
      */
    pub fn get_frames_per_callback(&self) -> i32 {
        
        todo!();
        /*
            return getFramesPerDataCallback();
        */
    }

    /**
      | @return
      | 
      | the number of frames in each data callback
      | or kUnspecified.
      |
      */
    pub fn get_frames_per_data_callback(&self) -> i32 {
        
        todo!();
        /*
            return mFramesPerCallback;
        */
    }

    /**
      | @return
      | 
      | the audio sample format (e.g. Float
      | or I16)
      |
      */
    pub fn get_format(&self) -> AudioFormat {
        
        todo!();
        /*
            return mFormat;
        */
    }

    /**
      | Query the maximum number of frames that
      | can be filled without blocking. If the
      | stream has been closed the last known
      | value will be returned.
      | 
      | -----------
      | @return
      | 
      | buffer size
      |
      */
    pub fn get_buffer_size_in_frames(&mut self) -> i32 {
        
        todo!();
        /*
            return mBufferSizeInFrames;
        */
    }

    /**
      | @return
      | 
      | capacityInFrames or kUnspecified
      |
      */
    pub fn get_buffer_capacity_in_frames(&self) -> i32 {
        
        todo!();
        /*
            return mBufferCapacityInFrames;
        */
    }

    /**
      | @return
      | 
      | the sharing mode of the stream.
      |
      */
    pub fn get_sharing_mode(&self) -> OboeSharingMode {
        
        todo!();
        /*
            return mSharingMode;
        */
    }

    /**
      | @return
      | 
      | the performance mode of the stream.
      |
      */
    pub fn get_performance_mode(&self) -> OboePerformanceMode {
        
        todo!();
        /*
            return mPerformanceMode;
        */
    }

    /**
      | @return
      | 
      | the device ID of the stream.
      |
      */
    pub fn get_device_id(&self) -> i32 {
        
        todo!();
        /*
            return mDeviceId;
        */
    }

    /**
      | For internal use only.
      | 
      | -----------
      | @return
      | 
      | the data callback object for this stream,
      | if set.
      |
      */
    pub fn get_data_callback(&self) -> *mut dyn AudioStreamDataCallback {
        
        todo!();
        /*
            return mDataCallback;
        */
    }

    /**
      | For internal use only.
      | 
      | -----------
      | @return
      | 
      | the error callback object for this stream,
      | if set.
      |
      */
    pub fn get_error_callback(&self) -> *mut dyn AudioStreamErrorCallback {
        
        todo!();
        /*
            return mErrorCallback;
        */
    }

    /**
      | @return
      | 
      | true if a data callback was set for this
      | stream
      |
      */
    pub fn is_data_callback_specified(&self) -> bool {
        
        todo!();
        /*
            return mDataCallback != nullptr;
        */
    }

    /**
      | @note
      | 
      | if the app does not set an error callback
      | then a default one may be provided. @return
      | true if an error callback was set for
      | this stream
      |
      */
    pub fn is_error_callback_specified(&self) -> bool {
        
        todo!();
        /*
            return mErrorCallback != nullptr;
        */
    }

    /**
      | @return
      | 
      | the usage for this stream.
      |
      */
    pub fn get_usage(&self) -> OboeUsage {
        
        todo!();
        /*
            return mUsage;
        */
    }

    /**
      | @return
      | 
      | the stream's content type.
      |
      */
    pub fn get_content_type(&self) -> OboeContentType {
        
        todo!();
        /*
            return mContentType;
        */
    }

    /**
      | @return
      | 
      | the stream's input preset.
      |
      */
    pub fn get_input_preset(&self) -> OboeInputPreset {
        
        todo!();
        /*
            return mInputPreset;
        */
    }

    /**
      | @return
      | 
      | the stream's session ID allocation
      | strategy (None or Allocate).
      |
      */
    pub fn get_session_id(&self) -> OboeSessionId {
        
        todo!();
        /*
            return mSessionId;
        */
    }

    /**
      | @return
      | 
      | true if Oboe can convert channel counts
      | to achieve optimal results.
      |
      */
    pub fn is_channel_conversion_allowed(&self) -> bool {
        
        todo!();
        /*
            return mChannelConversionAllowed;
        */
    }

    /**
      | @return
      | 
      | true if Oboe can convert data formats
      | to achieve optimal results.
      |
      */
    pub fn is_format_conversion_allowed(&self) -> bool {
        
        todo!();
        /*
            return mFormatConversionAllowed;
        */
    }

    /**
      | @return
      | 
      | whether and how Oboe can convert sample
      | rates to achieve optimal results.
      |
      */
    pub fn get_sample_rate_conversion_quality(&self) -> OboeSampleRateConversionQuality {
        
        todo!();
        /*
            return mSampleRateConversionQuality;
        */
    }

    /**
      | Validate stream parameters that might
      | not be checked in lower layers
      |
      */
    pub fn is_valid_config(&mut self) -> OboeResult {
        
        todo!();
        /*
            switch (mFormat) {
                case AudioFormat::Unspecified:
                case AudioFormat::I16:
                case AudioFormat::Float:
                case AudioFormat::I24:
                case AudioFormat::I32:
                    break;
                // Aloe CHANGE STARTS HERE
                case AudioFormat::Invalid:
                // Aloe CHANGE ENDS HERE
                default:
                    return OboeResult::ErrorInvalidFormat;
            }

            switch (mSampleRateConversionQuality) {
                case SampleRateConversionQuality::None:
                case SampleRateConversionQuality::Fastest:
                case SampleRateConversionQuality::Low:
                case SampleRateConversionQuality::Medium:
                case SampleRateConversionQuality::High:
                case SampleRateConversionQuality::Best:
                    return OboeResult::OK;
                default:
                    return OboeResult::ErrorIllegalArgument;
            }
        */
    }
}
