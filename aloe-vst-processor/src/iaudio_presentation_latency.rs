crate::ix!();

/**
  | Extended IAudioProcessor interface
  | for a component: Vst::IAudioPresentationLatency
  | \ingroup vstIPlug vst310
  | 
  | - [plug imp]
  | 
  | - [extends IAudioProcessor]
  | 
  | - [released: 3.1.0]
  | 
  | - [optional]
  | 
  | Inform the plug-in about how long from
  | the moment of generation/acquiring
  | (from file or from Input) it will take
  | for its input to arrive, and how long
  | it will take for its output to be presented
  | (to output or to speaker).
  | 
  | -----------
  | @note
  | 
  | for Input Presentation Latency: when
  | reading from file, the first plug-in
  | will have an input presentation latency
  | set to zero.
  | 
  | When monitoring audio input from an
  | audio device, the initial input latency
  | is the input latency of the audio device
  | itself.
  | ----------
  | @note
  | 
  | for Output Presentation Latency: when
  | writing to a file, the last plug-in will
  | have an output presentation latency
  | set to zero.
  | 
  | When the output of this plug-in is connected
  | to an audio device, the initial output
  | latency is the output latency of the
  | audio device itself.
  | 
  | A value of zero either means no latency
  | or an unknown latency.
  | 
  | Each plug-in adding a latency (returning
  | a none zero value for IAudioProcessor::getLatencySamples)
  | will modify the input presentation
  | latency of the next plug-ins in the mixer
  | routing graph and will modify the output
  | presentation latency of the previous
  | plug-ins.
  | 
  | \n \image html "iaudiopresentationlatency_usage.png"
  | \n \see IAudioProcessor \see IComponent
  |
  */
pub trait IAudioPresentationLatency: FUnknown {

    /**
      | Informs the plug-in about the Audio
      | Presentation Latency in samples for
      | a given direction (kInput/kOutput)
      | and bus index.
      |
      */
    #[PLUGIN_API]
    fn set_audio_presentation_latency_samples(&mut self, 
            dir:                BusDirection,
            bus_index:          i32,
            latency_in_samples: u32) -> tresult;
}

lazy_static!{
    /*
    static const FUID iaudio_presentation_latency_iid;
    */
}

declare_class_iid!{
    IAudioPresentationLatency, 
    0x309ECE78, 
    0xEB7D4fae, 
    0x8B2225D9, 
    0x09FD08B6
}
