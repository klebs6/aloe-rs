crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorListener.h]

/**
  | Base class for listeners that want to
  | know about changes to an AudioProcessor.
  | 
  | Use AudioProcessor::addListener()
  | to register your listener with an AudioProcessor.
  | 
  | @see AudioProcessor
  | 
  | @tags{Audio}
  |
  */
pub trait AudioProcessorListener:
    AudioProcessorParameterChanged
    + AudioProcessorChanged
    + AudioProcessorParameterChangeGestureBegin
    + AudioProcessorParameterChangeGestureEnd {}

