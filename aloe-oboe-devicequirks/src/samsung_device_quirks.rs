crate::ix!();

pub struct SamsungDeviceQuirks {

    base:             OboeDeviceQuirks,

    /**
      | Stay farther away from DSP position
      | on Exynos devices.
      |
      */
    is_exynos:        bool, // default = false

    is_exynos9810:    bool, // default = false
    is_exynos990:     bool, // default = false
    is_exynos850:     bool, // default = false
    build_changelist: i32, // default = 0
}

pub const SamsungDeviceQuirksBottomMarginExynos: i32 = 2;
pub const SamsungDeviceQuirksBottomMarginOther:  i32 = 1;
pub const SamsungDeviceQuirksTopMargin:          i32 = 1;

impl Default for SamsungDeviceQuirks {
    
    fn default() -> Self {
        todo!();
        /*

            std::string arch = getPropertyString("ro.arch");
            isExynos = (arch.rfind("exynos", 0) == 0); // starts with?

            std::string chipname = getPropertyString("ro.hardware.chipname");
            isExynos9810 = (chipname == "exynos9810");
            isExynos990 = (chipname == "exynos990");
            isExynos850 = (chipname == "exynos850");

            mBuildChangelist = getPropertyInteger("ro.build.changelist", 0)
        */
    }
}

impl SamsungDeviceQuirks {

    pub fn get_exclusive_bottom_margin_in_bursts(&self) -> i32 {
        
        todo!();
        /*
            // TODO Make this conditional on build version when MMAP timing improves.
            return isExynos ? kBottomMarginExynos : kBottomMarginOther;
        */
    }
    
    pub fn get_exclusive_top_margin_in_bursts(&self) -> i32 {
        
        todo!();
        /*
            return kTopMargin;
        */
    }

    /**
      | See Oboe issues #824 and #1247 for more
      | information.
      |
      */
    pub fn is_mono_mmap_actually_stereo(&self) -> bool {
        
        todo!();
        /*
            return isExynos9810 || isExynos850; // TODO We can make this version specific if it gets fixed.
        */
    }
    
    pub fn is_aaudio_mmap_possible(&self, builder: &AudioStreamBuilder) -> bool {
        
        todo!();
        /*
            return OboeDeviceQuirks::isAAudioMMapPossible(builder)
                    // Samsung says they use Legacy for Camcorder
                    && builder.getInputPreset() != OboeInputPreset::Camcorder;
        */
    }
    
    pub fn is_mmap_safe(&mut self, builder: &AudioStreamBuilder) -> bool {
        
        todo!();
        /*
            const bool isInput = builder.getDirection() == Direction::Input;
            // This detects b/159066712 , S20 LSI has corrupt low latency audio recording
            // and turns off MMAP.
            // See also https://github.com/google/oboe/issues/892
            bool isRecordingCorrupted = isInput
                && isExynos990
                && mBuildChangelist < 19350896;

            // Certain S9+ builds record silence when using MMAP and not using the VoiceCommunication
            // preset.
            // See https://github.com/google/oboe/issues/1110
            bool wouldRecordSilence = isInput
                && isExynos9810
                && mBuildChangelist <= 18847185
                && (builder.getInputPreset() != InputPreset::VoiceCommunication);

            if (wouldRecordSilence){
                LOGI("OboeQuirksManager::%s() Requested stream configuration would result in silence on "
                     "this device. Switching off MMAP.", __func__);
            }

            return !isRecordingCorrupted && !wouldRecordSilence;
        */
    }
}
