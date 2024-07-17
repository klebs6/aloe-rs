/*!
  | Create a derived implementation of
  | this class and pass it to
  | 
  | AudioPluginInstance::getExtensions()
  | to retrieve format-specific information
  | about a plugin instance.
  | 
  | -----------
  | @note
  | 
  | the references passed to the visit member
  | functions are only guaranteed to live
  | for the duration of the function call,
  | so don't store pointers to these objects!
  | If you need to store and reuse format-specific
  | information, it is recommended to copy
  | the result of the function calls that
  | you care about. For example, you should
  | store the result of Vst::getAEffectPtr()
  | rather than storing a pointer to the
  | Vst instance.
  | 
  | @tags{Audio}
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_ExtensionsVisitor.h]
pub trait ExtensionsVisitorInterface:
VisitUnknown
+ VisitVst3Client
+ VisitVstClient
+ VisitAudioUnitClient {}

/**
  | Indicates that there is no platform
  | specific information available.
  |
  */
pub struct Unknown {}

/**
  | Can be used to retrieve information
  | about a Vst3 that is wrapped by an AudioProcessor.
  |
  */
pub trait Vst3Client
{
    fn get_icomponent_ptr(&self);
    fn get_preset(&self) -> MemoryBlock;
    fn set_preset(&self, _0: &MemoryBlock) -> bool;
}

/**
  | Can be used to retrieve information
  | about an AudioUnit that is wrapped by
  | an AudioProcessor.
  |
  */
pub trait AudioUnitClient
{
    fn get_audio_unit_handle(&self);
}

/**
  | Can be used to retrieve information
  | about a Vst that is wrapped by an AudioProcessor.
  |
  */
pub trait VstClient
{
    fn get_aeffect_ptr(&self);
}

pub trait VisitUnknown {

    /**
      | Will be called if there is no platform
      | specific information available.
      |
      */
    fn visit_unknown(&mut self, _0: &Unknown)  { }
}

pub trait VisitVst3Client {

    /**
      | Called with Vst3-specific information.
      |
      */
    fn visit_vst3client(&mut self, _0: &dyn Vst3Client)  { }
}

pub trait VisitVstClient {

    /**
      | Called with Vst-specific information.
      |
      */
    fn visit_vst_client(&mut self, _0: &dyn VstClient)  { }
}

pub trait VisitAudioUnitClient {

    /**
      | Called with AU-specific information.
      |
      */
    fn visit_audio_unit_client(&mut self, _0: &dyn AudioUnitClient)  { }
}
