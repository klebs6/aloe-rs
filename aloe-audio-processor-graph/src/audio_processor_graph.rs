crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorGraph.h]

/**
  | A type of AudioProcessor which plays
  | back a graph of other AudioProcessors.
  | 
  | Use one of these objects if you want to
  | wire-up a set of AudioProcessors and
  | play back the result.
  | 
  | Processors can be added to the graph
  | as "nodes" using addNode(), and once
  | added, you can connect any of their input
  | or output channels to other nodes using
  | addConnection().
  | 
  | To play back a graph through an audio
  | device, you might want to use an
  | 
  | AudioProcessorPlayer object.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioProcessorGraph<'a> {
    base:                   AudioProcessor<'a>,
    base2:                  ChangeBroadcaster<'a>,
    base3:                  AsyncUpdater<'a>,
    nodes:                  ReferenceCountedArray<AudioProcessorGraphNode<'a>>,
    last_nodeid:            AudioProcessorGraphNodeID,
    render_sequence_float:  Box<AudioProcessorGraphRenderSequenceFloat>,
    render_sequence_double: Box<AudioProcessorGraphRenderSequenceDouble>,
    prepare_settings:       AudioProcessorGraphPrepareSettings,
    is_prepared:            AtomicBool, // default = { false  }
}

impl<'a> Default for AudioProcessorGraph<'a> {

    fn default() -> Self {
        todo!();
    }
}

impl<'a> Drop for AudioProcessorGraph<'a> {

    /**
      | Destructor.
      | 
      | Any processor objects that have been
      | added to the graph will also be deleted.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            cancelPendingUpdate();
        clearRenderingSequence();
        clear();
        */
    }
}

impl<'a> AudioProcessorGraph<'a> {

    /**
      | Returns the array of nodes in the graph.
      |
      */
    pub fn get_nodes(&self) -> &ReferenceCountedArray<AudioProcessorGraphNode> {
        
        todo!();
        /*
            return nodes;
        */
    }

    /**
      | Returns the number of nodes in the graph.
      |
      */
    pub fn get_num_nodes(&self) -> i32 {
        
        todo!();
        /*
            return nodes.size();
        */
    }

    /**
      | Returns a pointer to one of the nodes
      | in the graph.
      | 
      | This will return nullptr if the index
      | is out of range. @see getNodeForId
      |
      */
    pub fn get_node(&self, index: i32) -> AudioProcessorGraphNodePtr {
        
        todo!();
        /*
            return nodes[index];
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
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "Audio Graph";
        */
    }
    
    pub fn topology_changed(&mut self)  {
        
        todo!();
        /*
            sendChangeMessage();

        if (isPrepared)
            updateOnMessageThread (*this);
        */
    }
    
    /**
      | Deletes all nodes and connections from
      | this graph.
      | 
      | Any processor objects in the graph will
      | be deleted.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (getCallbackLock());

        if (nodes.isEmpty())
            return;

        nodes.clear();
        topologyChanged();
        */
    }
    
    /**
      | Searches the graph for a node with the
      | given ID number and returns it.
      | 
      | If no such node was found, this returns
      | nullptr. @see getNode
      |
      */
    pub fn get_node_for_id(&self, nodeid: AudioProcessorGraphNodeID) -> *mut AudioProcessorGraphNode {
        
        todo!();
        /*
            for (auto* n : nodes)
            if (n->nodeID == nodeID)
                return n;

        return {};
        */
    }
    
    /**
      | Adds a node to the graph.
      | 
      | This creates a new node in the graph,
      | for the specified processor. Once you
      | have added a processor to the graph,
      | the graph owns it and will delete it later
      | when it is no longer needed.
      | 
      | The optional nodeId parameter lets
      | you specify a unique ID to use for the
      | node.
      | 
      | If the value is already in use, this method
      | will fail and return an empty node.
      | 
      | If this succeeds, it returns a pointer
      | to the newly-created node.
      |
      */
    pub fn add_node(
        &mut self, 
        new_processor: Box<AudioProcessor>,
        nodeid:        AudioProcessorGraphNodeID

    ) -> AudioProcessorGraphNodePtr {
        
        todo!();
        /*
            if (newProcessor == nullptr || newProcessor.get() == this)
        {
            jassertfalse;
            return {};
        }

        if (nodeID == AudioProcessorGraphNodeID())
            nodeID.uid = ++(lastNodeID.uid);

        for (auto* n : nodes)
        {
            if (n->getProcessor() == newProcessor.get() || n->nodeID == nodeID)
            {
                jassertfalse; // Cannot add two copies of the same processor, or duplicate node IDs!
                return {};
            }
        }

        if (lastNodeID < nodeID)
            lastNodeID = nodeID;

        newProcessor->setPlayHead (getPlayHead());

        AudioProcessorGraphNode::Ptr n (new AudioProcessorGraphNode (nodeID, std::move (newProcessor)));

        {
            const ScopedLock sl (getCallbackLock());
            nodes.add (n.get());
        }

        n->setParentGraph (this);
        topologyChanged();
        return n;
        */
    }
    
    /**
      | Deletes a node within the graph which
      | has the specified ID.
      | 
      | This will also delete any connections
      | that are attached to this node.
      |
      */
    pub fn remove_node_by_id(&mut self, node_id: AudioProcessorGraphNodeID) -> AudioProcessorGraphNodePtr {
        
        todo!();
        /*
            const ScopedLock sl (getCallbackLock());

        for (int i = nodes.size(); --i >= 0;)
        {
            if (nodes.getUnchecked (i)->nodeID == nodeId)
            {
                disconnectNode (nodeId);
                auto node = nodes.removeAndReturn (i);
                topologyChanged();
                return node;
            }
        }

        return {};
        */
    }
    
    /**
      | Deletes a node within the graph.
      | 
      | This will also delete any connections
      | that are attached to this node.
      |
      */
    pub fn remove_node(&mut self, node: *mut AudioProcessorGraphNode) -> AudioProcessorGraphNodePtr {
        
        todo!();
        /*
            if (node != nullptr)
            return removeNode (node->nodeID);

        jassertfalse;
        return {};
        */
    }
    
    pub fn get_node_connections(&mut self, 
        node:        &mut AudioProcessorGraphNode,
        connections: &mut Vec<AudioProcessorGraphConnection>)  {
        
        todo!();
        /*
            for (auto& i : node.inputs)
            connections.push_back ({ { i.otherNode->nodeID, i.otherChannel }, { node.nodeID, i.thisChannel } });

        for (auto& o : node.outputs)
            connections.push_back ({ { node.nodeID, o.thisChannel }, { o.otherNode->nodeID, o.otherChannel } });
        */
    }
    
    /**
      | Returns the list of connections in the
      | graph.
      |
      */
    pub fn get_connections(&self) -> Vec<AudioProcessorGraphConnection> {
        
        todo!();
        /*
            std::vector<AudioProcessorGraphConnection> connections;

        for (auto& n : nodes)
            getNodeConnections (*n, connections);

        std::sort (connections.begin(), connections.end());
        auto last = std::unique (connections.begin(), connections.end());
        connections.erase (last, connections.end());

        return connections;
        */
    }
    
    pub fn is_connected_src_dst(
        &self, 
        source:         *mut AudioProcessorGraphNode,
        source_channel: i32,
        dest:           *mut AudioProcessorGraphNode,
        dest_channel:   i32

    ) -> bool {
        
        todo!();
        /*
            for (auto& o : source->outputs)
            if (o.otherNode == dest && o.thisChannel == sourceChannel && o.otherChannel == destChannel)
                return true;

        return false;
        */
    }
    
    /**
      | Returns true if the given connection
      | exists.
      |
      */
    pub fn is_connected(&self, c: &AudioProcessorGraphConnection) -> bool {
        
        todo!();
        /*
            if (auto* source = getNodeForId (c.source.nodeID))
            if (auto* dest = getNodeForId (c.destination.nodeID))
                return isConnected (source, c.source.channelIndex,
                                    dest, c.destination.channelIndex);

        return false;
        */
    }
    
    /**
      | Returns true if there is a direct connection
      | between any of the channels of two specified
      | nodes.
      |
      */
    pub fn is_connected_by_ids(
        &self, 
        srcid:  AudioProcessorGraphNodeID,
        destid: AudioProcessorGraphNodeID) -> bool {
        
        todo!();
        /*
            if (auto* source = getNodeForId (srcID))
            if (auto* dest = getNodeForId (destID))
                for (auto& out : source->outputs)
                    if (out.otherNode == dest)
                        return true;

        return false;
        */
    }
    
    /**
      | Does a recursive check to see if there's
      | a direct or indirect series of connections
      | between these two nodes.
      |
      */
    pub fn is_an_input_to(&self, 
        src: &mut AudioProcessorGraphNode,
        dst: &mut AudioProcessorGraphNode) -> bool {
        
        todo!();
        /*
            jassert (nodes.contains (&src));
        jassert (nodes.contains (&dst));

        return isAnInputTo (src, dst, nodes.size());
        */
    }
    
    pub fn is_an_input_to_with_recursion_check(
        &self, 
        src:             &mut AudioProcessorGraphNode,
        dst:             &mut AudioProcessorGraphNode,
        recursion_check: i32

    ) -> bool {
        
        todo!();
        /*
            for (auto&& i : dst.inputs)
            if (i.otherNode == &src)
                return true;

        if (recursionCheck > 0)
            for (auto&& i : dst.inputs)
                if (isAnInputTo (src, *i.otherNode, recursionCheck - 1))
                    return true;

        return false;
        */
    }
    
    pub fn can_connect_src_dst(
        &self, 
        source:         *mut AudioProcessorGraphNode,
        source_channel: i32,
        dest:           *mut AudioProcessorGraphNode,
        dest_channel:   i32

    ) -> bool {
        
        todo!();
        /*
            bool sourceIsMIDI = sourceChannel == midiChannelIndex;
        bool destIsMIDI   = destChannel == midiChannelIndex;

        if (sourceChannel < 0
             || destChannel < 0
             || source == dest
             || sourceIsMIDI != destIsMIDI)
            return false;

        if (source == nullptr
             || (! sourceIsMIDI && sourceChannel >= source->processor->getTotalNumOutputChannels())
             || (sourceIsMIDI && ! source->processor->producesMidi()))
            return false;

        if (dest == nullptr
             || (! destIsMIDI && destChannel >= dest->processor->getTotalNumInputChannels())
             || (destIsMIDI && ! dest->processor->acceptsMidi()))
            return false;

        return ! isConnected (source, sourceChannel, dest, destChannel);

        */
    }
    
    /**
      | Returns true if it would be legal to connect
      | the specified points.
      |
      */
    pub fn can_connect(&self, c: &AudioProcessorGraphConnection) -> bool {
        
        todo!();
        /*
            if (auto* source = getNodeForId (c.source.nodeID))
                if (auto* dest = getNodeForId (c.destination.nodeID))
                    return canConnect (source, c.source.channelIndex,
                                       dest, c.destination.channelIndex);

            return false;
        */
    }

    /**
      | Attempts to connect two specified channels
      | of two nodes.
      | 
      | If this isn't allowed (e.g. because
      | you're trying to connect a midi channel
      | to an audio one or other such nonsense),
      | then it'll return false.
      |
      */
    pub fn add_connection(&mut self, c: &AudioProcessorGraphConnection) -> bool {
        
        todo!();
        /*
            if (auto* source = getNodeForId (c.source.nodeID))
        {
            if (auto* dest = getNodeForId (c.destination.nodeID))
            {
                auto sourceChan = c.source.channelIndex;
                auto destChan = c.destination.channelIndex;

                if (canConnect (source, sourceChan, dest, destChan))
                {
                    source->outputs.add ({ dest, destChan, sourceChan });
                    dest->inputs.add ({ source, sourceChan, destChan });
                    jassert (isConnected (c));
                    topologyChanged();
                    return true;
                }
            }
        }

        return false;
        */
    }
    
    /**
      | Deletes the given connection.
      |
      */
    pub fn remove_connection(&mut self, c: &AudioProcessorGraphConnection) -> bool {
        
        todo!();
        /*
            if (auto* source = getNodeForId (c.source.nodeID))
        {
            if (auto* dest = getNodeForId (c.destination.nodeID))
            {
                auto sourceChan = c.source.channelIndex;
                auto destChan = c.destination.channelIndex;

                if (isConnected (source, sourceChan, dest, destChan))
                {
                    source->outputs.removeAllInstancesOf ({ dest, destChan, sourceChan });
                    dest->inputs.removeAllInstancesOf ({ source, sourceChan, destChan });
                    topologyChanged();
                    return true;
                }
            }
        }

        return false;
        */
    }
    
    /**
      | Removes all connections from the specified
      | node.
      |
      */
    pub fn disconnect_node(&mut self, nodeid: AudioProcessorGraphNodeID) -> bool {
        
        todo!();
        /*
            if (auto* node = getNodeForId (nodeID))
        {
            std::vector<AudioProcessorGraphConnection> connections;
            getNodeConnections (*node, connections);

            if (! connections.empty())
            {
                for (auto c : connections)
                    removeConnection (c);

                return true;
            }
        }

        return false;
        */
    }
    
    pub fn is_legal(&self, 
        source:         *mut AudioProcessorGraphNode,
        source_channel: i32,
        dest:           *mut AudioProcessorGraphNode,
        dest_channel:   i32) -> bool {
        
        todo!();
        /*
            return (sourceChannel == midiChannelIndex ? source->processor->producesMidi()
                                                  : isPositiveAndBelow (sourceChannel, source->processor->getTotalNumOutputChannels()))
            && (destChannel == midiChannelIndex ? dest->processor->acceptsMidi()
                                                : isPositiveAndBelow (destChannel, dest->processor->getTotalNumInputChannels()));
        */
    }
    
    /**
      | Returns true if the given connection's
      | channel numbers map on to valid channels
      | at each end.
      | 
      | Even if a connection is valid when created,
      | its status could change if a node changes
      | its channel config.
      |
      */
    pub fn is_connection_legal(&self, c: &AudioProcessorGraphConnection) -> bool {
        
        todo!();
        /*
            if (auto* source = getNodeForId (c.source.nodeID))
            if (auto* dest = getNodeForId (c.destination.nodeID))
                return isLegal (source, c.source.channelIndex, dest, c.destination.channelIndex);

        return false;
        */
    }
    
    /**
      | Performs a sanity checks of all the connections.
      | 
      | This might be useful if some of the processors
      | are doing things like changing their
      | channel counts, which could render
      | some connections obsolete.
      |
      */
    pub fn remove_illegal_connections(&mut self) -> bool {
        
        todo!();
        /*
            bool anyRemoved = false;

        for (auto* node : nodes)
        {
            std::vector<AudioProcessorGraphConnection> connections;
            getNodeConnections (*node, connections);

            for (auto c : connections)
                if (! isConnectionLegal (c))
                    anyRemoved = removeConnection (c) || anyRemoved;
        }

        return anyRemoved;
        */
    }
    
    pub fn clear_rendering_sequence(&mut self)  {
        
        todo!();
        /*
            std::unique_ptr<AudioProcessorGraphRenderSequenceFloat> oldSequenceF;
        std::unique_ptr<AudioProcessorGraphRenderSequenceDouble> oldSequenceD;

        {
            const ScopedLock sl (getCallbackLock());
            std::swap (renderSequenceFloat, oldSequenceF);
            std::swap (renderSequenceDouble, oldSequenceD);
        }
        */
    }
    
    pub fn any_nodes_need_preparing(&self) -> bool {
        
        todo!();
        /*
            for (auto* node : nodes)
            if (! node->isPrepared)
                return true;

        return false;
        */
    }
    
    pub fn build_rendering_sequence(&mut self)  {
        
        todo!();
        /*
            auto newSequenceF = std::make_unique<AudioProcessorGraphRenderSequenceFloat>();
        auto newSequenceD = std::make_unique<AudioProcessorGraphRenderSequenceDouble>();

        AudioProcessorGraphRenderSequenceBuilder<AudioProcessorGraphRenderSequenceFloat>  builderF (*this, *newSequenceF);
        AudioProcessorGraphRenderSequenceBuilder<AudioProcessorGraphRenderSequenceDouble> builderD (*this, *newSequenceD);

        const ScopedLock sl (getCallbackLock());

        const auto currentBlockSize = getBlockSize();
        newSequenceF->prepareBuffers (currentBlockSize);
        newSequenceD->prepareBuffers (currentBlockSize);

        if (anyNodesNeedPreparing())
        {
            renderSequenceFloat.reset();
            renderSequenceDouble.reset();

            for (auto* node : nodes)
                node->prepare (getSampleRate(), currentBlockSize, this, getProcessingPrecision());
        }

        isPrepared = 1;

        std::swap (renderSequenceFloat,  newSequenceF);
        std::swap (renderSequenceDouble, newSequenceD);
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            buildRenderingSequence();
        */
    }
    
    pub fn prepare_to_play(&mut self, 
        sample_rate:                 f64,
        estimated_samples_per_block: i32)  {
        
        todo!();
        /*
            {
            const ScopedLock sl (getCallbackLock());
            setRateAndBufferSizeDetails (sampleRate, estimatedSamplesPerBlock);

            const auto newPrepareSettings = [&]
            {
                AudioProcessorGraphPrepareSettings settings;
                settings.precision  = getProcessingPrecision();
                settings.sampleRate = sampleRate;
                settings.blockSize  = estimatedSamplesPerBlock;
                settings.valid      = true;
                return settings;
            }();

            if (prepareSettings != newPrepareSettings)
            {
                unprepare();
                prepareSettings = newPrepareSettings;
            }
        }

        clearRenderingSequence();

        updateOnMessageThread (*this);
        */
    }
    
    pub fn supports_double_precision_processing(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn unprepare(&mut self)  {
        
        todo!();
        /*
            prepareSettings.valid = false;

        isPrepared = 0;

        for (auto* n : nodes)
            n->unprepare();
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (getCallbackLock());

        cancelPendingUpdate();

        unprepare();

        if (renderSequenceFloat != nullptr)
            renderSequenceFloat->releaseBuffers();

        if (renderSequenceDouble != nullptr)
            renderSequenceDouble->releaseBuffers();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (getCallbackLock());

        for (auto* n : nodes)
            n->getProcessor()->reset();
        */
    }
    
    pub fn set_non_realtime(&mut self, is_processing_non_realtime: bool)  {
        
        todo!();
        /*
            const ScopedLock sl (getCallbackLock());

        AudioProcessor::setNonRealtime (isProcessingNonRealtime);

        for (auto* n : nodes)
            n->getProcessor()->setNonRealtime (isProcessingNonRealtime);
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
            return true;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
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
            if ((! isPrepared) && MessageManager::getInstance()->isThisTheMessageThread())
            handleAsyncUpdate();

        processBlockForBuffer<float> (buffer, midiMessages, *this, renderSequenceFloat, isPrepared);
        */
    }
    
    pub fn process_block_f64(
        &mut self, 
        buffer:        &mut AudioBuffer<f64>,
        midi_messages: &mut MidiBuffer

    ) {
        
        todo!();
        /*
            if ((! isPrepared) && MessageManager::getInstance()->isThisTheMessageThread())
            handleAsyncUpdate();

        processBlockForBuffer<double> (buffer, midiMessages, *this, renderSequenceDouble, isPrepared);
        */
    }
}
