/*!
  | Some shared helpers methods for using
  | the high-performance audio paths on
  | Android devices (OpenSL and Oboe).
  | 
  | @tags{Audio}
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_android_HighPerformanceAudioHelpers.h]

#[cfg(target_os="android")]
pub fn android_high_performance_get_native_sample_rate() -> f64 {
    
    todo!();
    /*
        return audioManagerGetProperty ("android.media.property.OUTPUT_SAMPLE_RATE").getDoubleValue();
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_get_native_buffer_size_hint() -> i32 {
    
    todo!();
    /*
        // This property is a hint of a native buffer size but it does not guarantee the size used.
            auto deviceBufferSize = audioManagerGetProperty ("android.media.property.OUTPUT_FRAMES_PER_BUFFER").getIntValue();

            if (deviceBufferSize == 0)
                return 192;

            return deviceBufferSize;
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_is_pro_audio_device() -> bool {
    
    todo!();
    /*
        static bool isSapaSupported = SystemStats::getDeviceManufacturer().containsIgnoreCase ("SAMSUNG")
                                         && DynamicLibrary().open ("libapa_jni.so");

            return androidHasSystemFeature ("android.hardware.audio.pro") || isSapaSupported;
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_has_low_latency_audio_path() -> bool {
    
    todo!();
    /*
        return androidHasSystemFeature ("android.hardware.audio.low_latency");
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_can_use_high_performance_audio_path(
        native_buffer_size:    i32,
        requested_buffer_size: i32,
        requested_sample_rate: i32) -> bool {
    
    todo!();
    /*
        return ((requestedBufferSize % nativeBufferSize) == 0)
                   && (requestedSampleRate == getNativeSampleRate())
                   && isProAudioDevice();
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_get_minimum_buffers_to_enqueue(
        native_buffer_size:    i32,
        requested_sample_rate: f64) -> i32 {
    
    todo!();
    /*
        if (canUseHighPerformanceAudioPath (nativeBufferSize, nativeBufferSize, (int) requestedSampleRate))
            {
                // see https://developer.android.com/ndk/guides/audio/opensl/opensl-prog-notes.html#sandp
                // "For Android 4.2 (API level 17) and earlier, a buffer count of two or more is required
                //  for lower latency. Beginning with Android 4.3 (API level 18), a buffer count of one
                //  is sufficient for lower latency."
                return (getAndroidSDKVersion() >= 18 ? 1 : 2);
            }

            // not using low-latency path so we can use the absolute minimum number of buffers to queue
            return 1;
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_buffers_to_queue_for_buffer_duration(
        native_buffer_size:    i32,
        buffer_duration_in_ms: i32,
        sample_rate:           f64) -> i32 {
    
    todo!();
    /*
        auto maxBufferFrames = static_cast<int> (std::ceil (bufferDurationInMs * sampleRate / 1000.0));
            auto maxNumBuffers   = static_cast<int> (std::ceil (static_cast<double> (maxBufferFrames)
                                                      / static_cast<double> (nativeBufferSize)));

            return jmax (getMinimumBuffersToEnqueue (nativeBufferSize, sampleRate), maxNumBuffers);
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_get_maximum_buffers_to_enqueue(
        native_buffer_size:  i32,
        maximum_sample_rate: f64) -> i32 {
    
    todo!();
    /*
        static constexpr int maxBufferSizeMs = 200;

            return jmax (8, buffersToQueueForBufferDuration (nativeBufferSize, maxBufferSizeMs, maximumSampleRate));
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_get_available_buffer_sizes(
        native_buffer_size:     i32,
        available_sample_rates: Vec<f64>) -> Vec<i32> {
    
    todo!();
    /*
        auto minBuffersToQueue = getMinimumBuffersToEnqueue (nativeBufferSize, getNativeSampleRate());
            auto maxBuffersToQueue = getMaximumBuffersToEnqueue (nativeBufferSize, findMaximum (availableSampleRates.getRawDataPointer(),
                                                                                                availableSampleRates.size()));

            Vec<int> bufferSizes;

            for (int i = minBuffersToQueue; i <= maxBuffersToQueue; ++i)
                bufferSizes.add (i * nativeBufferSize);

            return bufferSizes;
    */
}

#[cfg(target_os="android")]
pub fn android_high_performance_get_default_buffer_size(
        native_buffer_size:  i32,
        current_sample_rate: f64) -> i32 {
    
    todo!();
    /*
        static constexpr int defaultBufferSizeForLowLatencyDeviceMs = 40;
            static constexpr int defaultBufferSizeForStandardLatencyDeviceMs = 100;

            auto defaultBufferLength = (hasLowLatencyAudioPath() ? defaultBufferSizeForLowLatencyDeviceMs
                                                                 : defaultBufferSizeForStandardLatencyDeviceMs);

            auto defaultBuffersToEnqueue = buffersToQueueForBufferDuration (nativeBufferSize, defaultBufferLength, currentSampleRate);
            return defaultBuffersToEnqueue * nativeBufferSize;
    */
}

