crate::ix!();

pub const DeviceQuirksDefaultBottomMarginInBursts: i32 = 0;
pub const DeviceQuirksDefaultTopMarginInBursts:    i32 = 0;

/**
  | For Legacy streams, do not let the
  | buffer go below one burst.  b/129545119
  | | AAudio Legacy allows
  | setBufferSizeInFrames too low Fixed in
  | Q
  */
pub const DeviceQuirksLegacyBottomMarginInBursts: i32 = 1;
pub const DeviceQuirksCommonNativeRate:           i32 = 48000; // very typical native sample rate

//---------------------------------------------------
pub const kChannelCountMono:   i32 = 1;
pub const kChannelCountStereo: i32 = 2;

pub struct OboeDeviceQuirks {

}

impl OboeDeviceQuirks {

    /**
      | Exclusive MMAP streams can have
      | glitches because they are using
      | a timing model of the DSP to control IO
      | instead of direct synchronization.
      */
    pub fn get_exclusive_bottom_margin_in_bursts(&self) -> i32 {
        
        todo!();
        /*
            return kDefaultBottomMarginInBursts;
        */
    }
    
    pub fn get_exclusive_top_margin_in_bursts(&self) -> i32 {
        
        todo!();
        /*
            return kDefaultTopMarginInBursts;
        */
    }

    /**
      | On some devices, you can open a mono stream
      | but it is actually running in stereo!
      |
      */
    pub fn is_mono_mmap_actually_stereo(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn is_mmap_safe(&mut self, builder: &AudioStreamBuilder) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    /**
      | Restrict buffer size. This is mainly
      | to avoid glitches caused by MMAP timestamp
      | inaccuracies.
      | 
      | -----------
      | @param stream
      | 
      | @param requestedSize
      | 
      | @return
      |
      */
    pub fn clip_buffer_size(&mut self, 
        stream:         &mut AudioStream,
        requested_size: i32) -> i32 {
        
        todo!();
        /*
            if (!OboeGlobals::areWorkaroundsEnabled()) {
            return requestedSize;
        }
        int bottomMargin = kDefaultBottomMarginInBursts;
        int topMargin = kDefaultTopMarginInBursts;
        if (isMMapUsed(stream)) {
            if (stream.getSharingMode() == SharingMode::Exclusive) {
                bottomMargin = getExclusiveBottomMarginInBursts();
                topMargin = getExclusiveTopMarginInBursts();
            }
        } else {
            bottomMargin = kLegacyBottomMarginInBursts;
        }

        int32_t burst = stream.getFramesPerBurst();
        int32_t minSize = bottomMargin * burst;
        int32_t adjustedSize = requestedSize;
        if (adjustedSize < minSize ) {
            adjustedSize = minSize;
        } else {
            int32_t maxSize = stream.getBufferCapacityInFrames() - (topMargin * burst);
            if (adjustedSize > maxSize ) {
                adjustedSize = maxSize;
            }
        }
        return adjustedSize;
        */
    }
    
    pub fn is_aaudio_mmap_possible(&self, builder: &AudioStreamBuilder) -> bool {
        
        todo!();
        /*
            bool isSampleRateCompatible =
                builder.getSampleRate() == OboeUnspecified
                || builder.getSampleRate() == kCommonNativeRate
                || builder.getSampleRateConversionQuality() != SampleRateConversionQuality::None;
        return builder.getPerformanceMode() == PerformanceMode::LowLatency
                && isSampleRateCompatible
                && builder.getChannelCount() <= kChannelCountStereo;
        */
    }
}
