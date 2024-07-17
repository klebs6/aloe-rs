crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeQuirksManager.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/OboeQuirksManager.cpp]

#[cfg(not(__ANDROID_API_R__))]
pub const __ANDROID_API_R__: usize = 30;

/**
  | INTERNAL USE ONLY.
  | 
  | Based on manufacturer, model and Android
  | version number decide whether data
  | conversion needs to occur.
  | 
  | This also manages device and version
  | specific workarounds.
  |
  */
pub struct OboeQuirksManager {
    device_quirks: Box<OboeDeviceQuirks>,
}

impl Default for OboeQuirksManager {

    fn default() -> Self {
    
        todo!();
        /*

            std::string manufacturer = getPropertyString("ro.product.manufacturer");
        if (manufacturer == "samsung") {
            mDeviceQuirks = std::make_unique<SamsungDeviceQuirks>();
        } else {
            mDeviceQuirks = std::make_unique<OboeDeviceQuirks>();
        }
        */
    }
}
    
impl OboeQuirksManager {

    pub fn get_instance<'a>() -> &'a mut OboeQuirksManager {
        
        todo!();
        /*
            static OboeQuirksManager instance; // singleton
            return instance;
        */
    }

    pub fn is_mmap_used(stream: &mut AudioStream) -> bool {
        
        todo!();
        /*
            bool answer = false;
            if (stream.getAudioApi() == AudioApi::AAudio) {
                AudioStreamAAudio *streamAAudio =
                        reinterpret_cast<AudioStreamAAudio *>(&stream);
                answer = streamAAudio->isMMapUsed();
            }
            return answer;
        */
    }
    
    pub fn clip_buffer_size(&mut self, 
        stream:      &mut AudioStream,
        buffer_size: i32) -> i32 {
        
        todo!();
        /*
            return mDeviceQuirks->clipBufferSize(stream, bufferSize);
        */
    }
    
    /**
      | Do we need to do channel, format or rate
      | conversion to provide a low latency
      | stream for this builder? If so then provide
      | a builder for the native child stream
      | that will be used to get low latency.
      | 
      | -----------
      | @param builder
      | 
      | builder provided by application
      | ----------
      | @param childBuilder
      | 
      | modified builder appropriate for the
      | underlying device
      | 
      | -----------
      | @return
      | 
      | true if conversion is needed
      |
      */
    pub fn is_conversion_needed(&mut self, 
        builder:       &AudioStreamBuilder,
        child_builder: &mut AudioStreamBuilder) -> bool {
        
        todo!();
        /*
            bool conversionNeeded = false;
        const bool isLowLatency = builder.getPerformanceMode() == PerformanceMode::LowLatency;
        const bool isInput = builder.getDirection() == Direction::Input;
        const bool isFloat = builder.getFormat() == AudioFormat::Float;

        // There are multiple bugs involving using callback with a specified callback size.
        // Issue #778: O to Q had a problem with Legacy INPUT streams for FLOAT streams
        // and a specified callback size. It would assert because of a bad buffer size.
        //
        // Issue #973: O to R had a problem with Legacy output streams using callback and a specified callback size.
        // An AudioTrack stream could still be running when the AAudio FixedBlockReader was closed.
        // Internally b/161914201#comment25
        //
        // Issue #983: O to R would glitch if the framesPerCallback was too small.
        //
        // Most of these problems were related to Legacy stream. MMAP was OK. But we don't
        // know if we will get an MMAP stream. So, to be safe, just do the conversion in Oboe.
        if (OboeGlobals::areWorkaroundsEnabled()
                && builder.willUseAAudio()
                && builder.isDataCallbackSpecified()
                && builder.getFramesPerDataCallback() != 0
                && getSdkVersion() <= __ANDROID_API_R__) {
            LOGI("OboeQuirksManager::%s() avoid setFramesPerCallback(n>0)", __func__);
            childBuilder.setFramesPerCallback(OboeUnspecified);
            conversionNeeded = true;
        }

        // If a SAMPLE RATE is specified for low latency then let the native code choose an optimal rate.
        // TODO There may be a problem if the devices supports low latency
        //      at a higher rate than the default.
        if (builder.getSampleRate() != OboeUnspecified
                && builder.getSampleRateConversionQuality() != SampleRateConversionQuality::None
                && isLowLatency
                ) {
            childBuilder.setSampleRate(OboeUnspecified); // native API decides the best sample rate
            conversionNeeded = true;
        }

        // Data Format
        // OpenSL ES and AAudio before P do not support FAST path for FLOAT capture.
        if (isFloat
                && isInput
                && builder.isFormatConversionAllowed()
                && isLowLatency
                && (!builder.willUseAAudio() || (getSdkVersion() < __ANDROID_API_P__))
                ) {
            childBuilder.setFormat(AudioFormat::I16); // needed for FAST track
            conversionNeeded = true;
            LOGI("OboeQuirksManager::%s() forcing internal format to I16 for low latency", __func__);
        }

        // Add quirk for float output on API <21
        if (isFloat
                && !isInput
                && getSdkVersion() < __ANDROID_API_L__
                && builder.isFormatConversionAllowed()
                ) {
            childBuilder.setFormat(AudioFormat::I16);
            conversionNeeded = true;
            LOGI("OboeQuirksManager::%s() float was requested but not supported on pre-L devices, "
                 "creating an underlying I16 stream and using format conversion to provide a float "
                 "stream", __func__);
        }

        // Channel Count conversions
        if (OboeGlobals::areWorkaroundsEnabled()
                && builder.isChannelConversionAllowed()
                && builder.getChannelCount() == kChannelCountStereo
                && isInput
                && isLowLatency
                && (!builder.willUseAAudio() && (getSdkVersion() == __ANDROID_API_O__))
                ) {
            // Workaround for heap size regression in O.
            // b/66967812 AudioRecord does not allow FAST track for stereo capture in O
            childBuilder.setChannelCount(kChannelCountMono);
            conversionNeeded = true;
            LOGI("OboeQuirksManager::%s() using mono internally for low latency on O", __func__);
        } else if (OboeGlobals::areWorkaroundsEnabled()
                   && builder.getChannelCount() == kChannelCountMono
                   && isInput
                   && mDeviceQuirks->isMonoMMapActuallyStereo()
                   && builder.willUseAAudio()
                   // Note: we might use this workaround on a device that supports
                   // MMAP but will use Legacy for this stream.  But this will only happen
                   // on devices that have the broken mono.
                   && mDeviceQuirks->isAAudioMMapPossible(builder)
                   ) {
            // Workaround for mono actually running in stereo mode.
            childBuilder.setChannelCount(kChannelCountStereo); // Use stereo and extract first channel.
            conversionNeeded = true;
            LOGI("OboeQuirksManager::%s() using stereo internally to avoid broken mono", __func__);
        }
        // Note that MMAP does not support mono in 8.1. But that would only matter on Pixel 1
        // phones and they have almost all been updated to 9.0.

        return conversionNeeded;
        */
    }
    
    pub fn is_mmap_safe(&mut self, builder: &mut AudioStreamBuilder) -> bool {
        
        todo!();
        /*
            if (!OboeGlobals::areWorkaroundsEnabled()) return true;
        return mDeviceQuirks->isMMapSafe(builder);
        */
    }
}
