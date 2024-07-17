crate::ix!();

pub struct AudioProcessorGraphNodeConnection<'a> 
{
    other_node:    *mut AudioProcessorGraphNode<'a>,
    other_channel: i32,
    this_channel:  i32,
}

impl<'a> PartialEq<AudioProcessorGraphNodeConnection<'a>> for AudioProcessorGraphNodeConnection<'a> {
    
    #[inline] fn eq(&self, other: &AudioProcessorGraphNodeConnection) -> bool {
        todo!();
        /*
            return otherNode == other.otherNode
            && thisChannel == other.thisChannel
            && otherChannel == other.otherChannel;
        */
    }
}

impl<'a> Eq for AudioProcessorGraphNodeConnection<'a> {}

//------------------------------------------

/**
  | Represents one of the nodes, or processors,
  | in an AudioProcessorGraph.
  | 
  | To create a node, call AudioProcessorGraph::addNode().
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorGraphNode<'a> {

    base:           ReferenceCountedObject,

    /**
      | The ID number assigned to this node.
      | 
      | This is assigned by the graph that owns
      | it, and can't be changed.
      |
      */
    nodeid:         AudioProcessorGraphNodeID,


    /**
      | A set of user-definable properties
      | that are associated with this node.
      | 
      | This can be used to attach values to the
      | node for whatever purpose seems useful.
      | For example, you might store an x and
      | y position if your application is displaying
      | the nodes on-screen.
      |
      */
    properties:     NamedValueSet,

    processor:      Box<AudioProcessor<'a>>,
    inputs:         Vec<AudioProcessorGraphNodeConnection<'a>>,
    outputs:        Vec<AudioProcessorGraphNodeConnection<'a>>,
    is_prepared:    bool, // default = false
    bypassed:       AtomicBool, // default = { false  }
    processor_lock: CriticalSection,
}

impl<'a> AudioProcessorGraphNode<'a> {

    /**
      | The actual processor object that this
      | node represents.
      |
      */
    pub fn get_processor(&self) -> *mut AudioProcessor {
        
        todo!();
        /*
            return processor.get();
        */
    }

    pub fn process_block<Sample>(&mut self, 
        audio: &mut AudioBuffer<Sample>,
        midi:  &mut MidiBuffer)  {
    
        todo!();
        /*
            const ScopedLock lock (processorLock);
                processor->processBlock (audio, midi);
        */
    }
    
    
    pub fn process_block_bypassed<Sample>(&mut self, 
        audio: &mut AudioBuffer<Sample>,
        midi:  &mut MidiBuffer)  {
    
        todo!();
        /*
            const ScopedLock lock (processorLock);
                processor->processBlockBypassed (audio, midi);
        */
    }
    
    pub fn new(
        n: AudioProcessorGraphNodeID,
        p: Box<AudioProcessor>) -> Self {
    
        todo!();
        /*
        : nodeid(n),
        : processor(std::move (p)),

            jassert (processor != nullptr);
        */
    }
    
    pub fn prepare(
        &mut self, 
        new_sample_rate: f64,
        new_block_size:  i32,
        graph:           *mut AudioProcessorGraph,
        precision:       AudioProcessorProcessingPrecision

    ) {
        
        todo!();
        /*
            const ScopedLock lock (processorLock);

        if (! isPrepared)
        {
            setParentGraph (graph);

            // try to align the precision of the processor and the graph
            processor->setProcessingPrecision (processor->supportsDoublePrecisionProcessing() ? precision
                                                                                              : singlePrecision);

            processor->setRateAndBufferSizeDetails (newSampleRate, newBlockSize);
            processor->prepareToPlay (newSampleRate, newBlockSize);

            // This may be checked from other threads that haven't taken the processorLock,
            // so we need to leave it until the processor has been completely prepared
            isPrepared = true;
        }
        */
    }
    
    pub fn unprepare(&mut self)  {
        
        todo!();
        /*
            const ScopedLock lock (processorLock);

        if (isPrepared)
        {
            isPrepared = false;
            processor->releaseResources();
        }
        */
    }
    
    pub fn set_parent_graph(&self, graph: *mut AudioProcessorGraph)  {
        
        todo!();
        /*
            const ScopedLock lock (processorLock);

        if (auto* ioProc = dynamic_cast<AudioProcessorGraph::AudioGraphIOProcessor*> (processor.get()))
            ioProc->setParentGraph (graph);
        */
    }
    
    /**
      | Returns if the node is bypassed or not.
      |
      */
    pub fn is_bypassed(&self) -> bool {
        
        todo!();
        /*
            if (processor != nullptr)
        {
            if (auto* bypassParam = processor->getBypassParameter())
                return (bypassParam->getValue() != 0.0f);
        }

        return bypassed;
        */
    }
    
    /**
      | Tell this node to bypass processing.
      |
      */
    pub fn set_bypassed(&mut self, should_be_bypassed: bool)  {
        
        todo!();
        /*
            if (processor != nullptr)
        {
            if (auto* bypassParam = processor->getBypassParameter())
                bypassParam->setValueNotifyingHost (shouldBeBypassed ? 1.0f : 0.0f);
        }

        bypassed = shouldBeBypassed;
        */
    }
}
