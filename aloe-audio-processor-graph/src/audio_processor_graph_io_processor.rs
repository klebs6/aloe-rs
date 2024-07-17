crate::ix!();

/**
  | A special type of AudioProcessor that
  | can live inside an AudioProcessorGraph
  | in order to use the audio that comes into
  | and out of the graph itself.
  | 
  | If you create an AudioGraphIOProcessor
  | in "input" mode, it will act as a node
  | in the graph which delivers the audio
  | that is coming into the parent graph.
  | This allows you to stream the data to
  | other nodes and process the incoming
  | audio.
  | 
  | Likewise, one of these in "output" mode
  | can be sent data which it will add to the
  | sum of data being sent to the graph's
  | output.
  | 
  | @see AudioProcessorGraph
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioGraphIOProcessor<'a> {
    base:  AudioPluginInstance<'a>,
    ty:    AudioGraphIOProcessorIODeviceType,
    graph: *mut AudioProcessorGraph<'a>, // default = nullptr
}

impl<'a> AudioGraphIOProcessor<'a> {

    /**
      | Returns the mode of this processor.
      |
      */
    pub fn get_type(&self) -> AudioGraphIOProcessorIODeviceType {
        
        todo!();
        /*
            return type;
        */
    }

    /**
      | Returns the parent graph to which this
      | processor belongs, or nullptr if it
      | hasn't yet been added to one.
      |
      */
    pub fn get_parent_graph(&self) -> *mut AudioProcessorGraph {
        
        todo!();
        /*
            return graph;
        */
    }

    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(device_type: AudioGraphIOProcessorIODeviceType) -> Self {
    
        todo!();
        /*
        : ty(deviceType),
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            switch (type)
        {
            case audioOutputNode:   return "Audio Output";
            case audioInputNode:    return "Audio Input";
            case midiOutputNode:    return "MIDI Output";
            case midiInputNode:     return "MIDI Input";
            default:                break;
        }

        return {};
        */
    }
    
    pub fn fill_in_plugin_description(&self, d: &mut PluginDescription)  {
        
        todo!();
        /*
            d.name = getName();
        d.category = "I/O devices";
        d.pluginFormatName = "Internal";
        d.manufacturerName = "Aloe";
        d.version = "1.0";
        d.isInstrument = false;

        d.deprecatedUid = d.uniqueId = d.name.hashCode();

        d.numInputChannels = getTotalNumInputChannels();

        if (type == audioOutputNode && graph != nullptr)
            d.numInputChannels = graph->getTotalNumInputChannels();

        d.numOutputChannels = getTotalNumOutputChannels();

        if (type == audioInputNode && graph != nullptr)
            d.numOutputChannels = graph->getTotalNumOutputChannels();
        */
    }
    
    pub fn prepare_to_play(&mut self, _0: f64, _1: i32)  {
        
        todo!();
        /*
            jassert (graph != nullptr);
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn supports_double_precision_processing(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn process_block_f32(
        &mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer

    ) {
        
        todo!();
        /*
            jassert (graph != nullptr);
        processIOBlock (*this, *graph->renderSequenceFloat, buffer, midiMessages);
        */
    }
    
    pub fn process_block_f64(
        &mut self, 
        buffer:        &mut AudioBuffer<f64>,
        midi_messages: &mut MidiBuffer

    ) {
        
        todo!();
        /*
            jassert (graph != nullptr);
        processIOBlock (*this, *graph->renderSequenceDouble, buffer, midiMessages);
        */
    }
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return type == midiOutputNode;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return type == midiInputNode;
        */
    }
    
    /**
      | True if this is an audio or midi input.
      |
      */
    pub fn is_input(&self) -> bool {
        
        todo!();
        /*
            return type == audioInputNode  || type == midiInputNode;
        */
    }
    
    /**
      | True if this is an audio or midi output.
      |
      */
    pub fn is_output(&self) -> bool {
        
        todo!();
        /*
            return type == audioOutputNode || type == midiOutputNode;
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn set_current_program(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_program_name(&mut self, _0: i32) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    pub fn change_program_name(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_state_information(&mut self, _0: &mut MemoryBlock)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_parent_graph(&mut self, new_graph: *mut AudioProcessorGraph)  {
        
        todo!();
        /*
            graph = newGraph;

        if (graph != nullptr)
        {
            setPlayConfigDetails (type == audioOutputNode ? graph->getTotalNumOutputChannels() : 0,
                                  type == audioInputNode  ? graph->getTotalNumInputChannels()  : 0,
                                  getSampleRate(),
                                  getBlockSize());

            updateHostDisplay();
        }
        */
    }
}
