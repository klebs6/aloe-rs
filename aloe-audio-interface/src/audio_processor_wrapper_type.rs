crate::ix!();

/**
  | Flags to indicate the type of plugin
  | context in which a processor is being
  | used.
  |
  */
pub enum AudioProcessorWrapperType
{
    Undefined = 0,
    Vst,
    Vst3,
    AudioUnit,
    AudioUnitv3,
    RTAS,
    AAX,
    Standalone,
    Unity
}
