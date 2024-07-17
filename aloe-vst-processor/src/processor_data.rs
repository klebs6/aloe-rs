crate::ix!();

/**
  | Any data needed in audio processing.
  | 
  | The host prepares AudioBusBuffers
  | for each input/output bus, regardless
  | of the bus activation state. Bus buffer
  | indices always match with bus indices
  | used in IComponent::getBusInfo of
  | media type kAudio. \see AudioBusBuffers,
  | IParameterChanges, IEventList, ProcessContext,
  | IProcessContextRequirements
  |
  */
pub struct ProcessData {

    /**
      | processing mode - value of \ref
      | 
      | ProcessModes
      |
      */
    process_mode:             i32,

    /**
      | sample size - value of \ref
      | SymbolicSampleSizes
      |
      */
    symbolic_sample_size:     i32,

    /**
      | number of samples to process
      |
      */
    num_samples:              i32,

    /**
      | number of audio input busses
      |
      */
    num_inputs:               i32,

    /**
      | number of audio output busses
      |
      */
    num_outputs:              i32,

    /**
      | buffers of input busses
      |
      */
    inputs:                   *mut AudioBusBuffers,

    /**
      | buffers of output busses
      |
      */
    outputs:                  *mut AudioBusBuffers,

    /**
      | incoming parameter changes for this
      | block
      |
      */
    input_parameter_changes:  *mut dyn IParameterChanges,

    /**
      | outgoing parameter changes for this
      | block (optional)
      |
      */
    output_parameter_changes: *mut dyn IParameterChanges,

    /**
      | incoming events for this block (optional)
      |
      */
    input_events:             *mut dyn IEventList,

    /**
      | outgoing events for this block (optional)
      |
      */
    output_events:            *mut dyn IEventList,

    /**
      | processing context (optional, but
      | most welcome)
      |
      */
    process_context:          *mut ProcessContext,
}

impl Default for ProcessData {
    
    fn default() -> Self {
        todo!();
        /*
        : process_mode(0),
        : symbolic_sample_size(kSample32),
        : num_samples(0),
        : num_inputs(0),
        : num_outputs(0),
        : inputs(nullptr),
        : outputs(nullptr),
        : input_parameter_changes(nullptr),
        : output_parameter_changes(nullptr),
        : input_events(nullptr),
        : output_events(nullptr),
        : process_context(nullptr),
        */
    }
}
