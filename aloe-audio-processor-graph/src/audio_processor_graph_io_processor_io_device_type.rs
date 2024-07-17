crate::ix!();

/**
  | Specifies the mode in which this processor
  | will operate.
  |
  */
pub enum AudioGraphIOProcessorIODeviceType
{
    /**
      | In this mode, the processor has output
      | channels representing all the audio
      | input channels that are coming into
      | its parent audio graph.
      |
      */
    audioInputNode,     

    /**
      | In this mode, the processor has input
      | channels representing all the audio
      | output channels that are going out of
      | its parent audio graph.
      |
      */
    audioOutputNode,    

    /**
      | In this mode, the processor has a midi
      | output which delivers the same midi
      | data that is arriving at its parent graph.
      |
      */
    midiInputNode,      

    /**
      | In this mode, the processor has a midi
      | input and any data sent to it will be passed
      | out of the parent graph.
      |
      */
    midiOutputNode,      
}
