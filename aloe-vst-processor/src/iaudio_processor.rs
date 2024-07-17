crate::ix!();

/**
  | Audio processing interface: Vst::IAudioProcessor
  | \ingroup vstIPlug vst300
  | 
  | - [plug imp]
  | 
  | - [extends IComponent]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | This interface must always be supported
  | by audio processing plug-ins.
  |
  */
pub trait IAudioProcessor: FUnknown {

    /**
      | Try to set (host => plug-in) a wanted
      | arrangement for inputs and outputs.
      | 
      | The host should always deliver the same
      | number of input and output busses than
      | the plug-in needs (see \ref IComponent::getBusCount).
      | The plug-in has 3 possibilities to react
      | on this setBusArrangements call:\n
      | 
      | 1. The plug-in accepts these arrangements,
      | then it should modify, if needed, its
      | busses to match these new arrangements
      | (later on asked by the host with IComponent::getBusInfo
      | () or IAudioProcessor::getBusArrangement
      | ()) and then should return kResultTrue.\n
      | 
      | 2. The plug-in does not accept or support
      | these requested arrangements for all
      | inputs/outputs or just for some or only
      | one bus, but the plug-in can try to adapt
      | its current arrangements according
      | to the requested ones (requested arrangements
      | for kMain busses should be handled with
      | more priority than the ones for kAux
      | busses), then it should modify its busses
      | arrangements and should return kResultFalse.\n
      | 
      | 3. Same than the point 2 above the plug-in
      | does not support these requested arrangements
      | but the plug-in cannot find corresponding
      | arrangements, the plug-in could keep
      | its current arrangement or fall back
      | to a default arrangement by modifying
      | its busses arrangements and should
      | return kResultFalse.\n
      | 
      | -----------
      | @param inputs
      | 
      | pointer to an array of /ref SpeakerArrangement
      | ----------
      | @param numIns
      | 
      | number of /ref SpeakerArrangement
      | in inputs array
      | ----------
      | @param outputs
      | 
      | pointer to an array of /ref SpeakerArrangement
      | ----------
      | @param numOuts
      | 
      | number of /ref SpeakerArrangement
      | in outputs array
      | 
      | -----------
      | @return
      | 
      | kResultTrue when Arrangements is supported
      | and is the current one, else returns
      | kResultFalse.
      |
      */
    #[PLUGIN_API]
    fn set_bus_arrangements(&mut self, 
            inputs:   *mut SpeakerArrangement,
            num_ins:  i32,
            outputs:  *mut SpeakerArrangement,
            num_outs: i32) -> tresult;


    /**
      | Gets the bus arrangement for a given
      | direction (input/output) and index.
      | 
      | -----------
      | @note
      | 
      | IComponent::getBusInfo () and IAudioProcessor::getBusArrangement
      | () should be always return the same information
      | about the busses arrangements.
      |
      */
    #[PLUGIN_API]
    fn get_bus_arrangement(&mut self, 
            dir:   BusDirection,
            index: i32,
            arr:   &mut SpeakerArrangement) -> tresult;

    /**
      | Asks if a given sample size is supported
      | see \ref SymbolicSampleSizes.
      |
      */
    #[PLUGIN_API]
    fn can_process_sample_size(&mut self, symbolic_sample_size: i32) -> tresult;

    /**
      | Gets the current Latency in samples.
      | 
      | The returned value defines the group
      | delay or the latency of the plug-in.
      | For example, if the plug-in internally
      | needs to look in advance (like compressors)
      | 512 samples then this plug-in should
      | report 512 as latency.
      | 
      | If during the use of the plug-in this
      | latency change, the plug-in has to inform
      | the host by using IComponentHandler::restartComponent
      | (kLatencyChanged), this could lead
      | to audio playback interruption because
      | the host has to recompute its internal
      | mixer delay compensation.
      | 
      | -----------
      | @note
      | 
      | for player live recording this latency
      | should be zero or small.
      |
      */
    #[PLUGIN_API]
    fn get_latency_samples(&mut self) -> u32;

    /**
      | Called in disable state (setActive
      | not called with true) before setProcessing
      | is called and processing will begin.
      |
      */
    #[PLUGIN_API]
    fn setup_processing(&mut self, setup: &mut ProcessSetup) -> tresult;

    /**
      | Informs the plug-in about the processing
      | state. This will be called before any
      | process calls start with true and after
      | with false.
      | 
      | -----------
      | @note
      | 
      | setProcessing (false) may be called
      | after setProcessing (true) without
      | any process calls.
      | 
      | Note this function could be called in
      | the UI or in Processing Thread, thats
      | why the plug-in should only light operation
      | (no memory allocation or big setup reconfiguration),
      | this could be used to reset some buffers
      | (like Delay line or Reverb).
      | 
      | The host has to be sure that it is called
      | only when the plug-in is enable (setActive
      | (true) was called).
      |
      */
    #[PLUGIN_API]
    fn set_processing(&mut self, state: TBool) -> tresult;

    /**
      | The Process call, where all information
      | (parameter changes, event, audio buffer)
      | are passed.
      |
      */
    #[PLUGIN_API]
    fn process(&mut self, data: &mut ProcessData) -> tresult;

    /**
      | Gets tail size in samples. For example,
      | if the plug-in is a Reverb plug-in and
      | it knows that the maximum length of the
      | Reverb is 2sec, then it has to return
      | in getTailSamples() (in VST2 it was
      | getGetTailSize ()): 2*sampleRate.
      | 
      | This information could be used by host
      | for offline processing, process optimization
      | and downmix (avoiding signal cut (clicks)).
      | 
      | It should return:
      | 
      | - kNoTail when no tail
      | 
      | - x * sampleRate when x Sec tail.
      | 
      | - kInfiniteTail when infinite tail.
      |
      */
    #[PLUGIN_API]
    fn get_tail_samples(&mut self) -> u32;
}

lazy_static!{
    /*
    static const FUID iaudio_processor_iid;
    */
}

declare_class_iid!{
    IAudioProcessor, 
    0x42043F99, 
    0xB7DA453C, 
    0xA569E79D, 
    0x9AAEC33D
}
