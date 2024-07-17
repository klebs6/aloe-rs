crate::ix!();

#[cfg(not(__COREAUDIO_USE_FLAT_INCLUDES__))]
#[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
#[cfg(MAC_OS_X_VERSION_MAX_ALLOWED_LTE_MAC_OS_X_VERSION_10_5)]
macro_rules! audio_component {
    () => {
        /*
                Component
        */
    }
}

#[cfg(not(__COREAUDIO_USE_FLAT_INCLUDES__))]
#[cfg(MAC_OS_X_VERSION_MAX_ALLOWED_LTE_MAC_OS_X_VERSION_10_4)]
pub type AudioUnitParameterValue = f32;

#[cfg(not(__COREAUDIO_USE_FLAT_INCLUDES__))]
#[cfg(COREAUDIOTYPES_VERSION_LT_1051)]
pub type AudioUnitSampleType = f32;

