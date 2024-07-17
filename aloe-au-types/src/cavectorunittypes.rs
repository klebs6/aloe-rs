crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAVectorUnitTypes.h]
pub enum CAVectorUnitType {
    VecUninitialized = -1,
    VecNone          = 0,
    VecAltivec       = 1,
    VecSSE2          = 100,
    VecSSE3          = 101,
    VecAVX1          = 110,
    VecNeon          = 200
}
