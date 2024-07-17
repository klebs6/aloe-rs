crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstaudioprocessor.h]

/**
  | Class Category Name for Audio Processor
  | Component
  |
  */
#[cfg(not(kVstAudioEffectClass))]
pub const VstAudioEffectClass: &'static str = "Audio Module Class";

/**
  | Component Flags used as classFlags
  | in PClassInfo2
  |
  */
pub enum ComponentFlags
{
    /**
      | Component can be run on remote computer
      |
      */
    kDistributable          = 1 << 0,   

    /**
      | Component supports simple IO mode (or
      | works in simple mode anyway) see \ref
      | vst3IoMode
      |
      */
    kSimpleModeSupported    = 1 << 1,    
}

/**
  | Symbolic sample size. \see ProcessSetup,
  | ProcessData
  |
  */
pub enum SymbolicSampleSizes
{
    /**
      | 32-bit precision
      |
      */
    kSample32,      

    /**
      | 64-bit precision
      |
      */
    kSample64       
}
