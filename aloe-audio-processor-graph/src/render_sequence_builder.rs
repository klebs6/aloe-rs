crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_AudioProcessorGraph.cpp]

#[no_copy]
#[leak_detector]
pub struct AudioProcessorGraphRenderSequenceBuilder<'a, RenderSequence> {
    graph:         &'a mut AudioProcessorGraph<'a>,
    sequence:      &'a mut RenderSequence,
    ordered_nodes: Vec<*mut RenderSequenceBuilderAudioProcessorGraphNode<'a>>,
    audio_buffers: Vec<RenderSequenceBuilderAssignedBuffer>,
    midi_buffers:  Vec<RenderSequenceBuilderAssignedBuffer>,
    delays:        HashMap<u32,i32>,
    total_latency: i32, // default = 0
}

impl<'a, RenderSequence> AudioProcessorGraphRenderSequenceBuilder<'a, RenderSequence> {

    pub fn new(
        g: &mut AudioProcessorGraph,
        s: &mut RenderSequence) -> Self {
    
        todo!();
        /*
        : graph(g),
        : sequence(s),
        : ordered_nodes(createOrderedNodeList (graph)),

            audioBuffers.add (RenderSequenceBuilderAssignedBuffer::createReadOnlyEmpty()); // first buffer is read-only zeros
            midiBuffers .add (RenderSequenceBuilderAssignedBuffer::createReadOnlyEmpty());

            for (int i = 0; i < orderedNodes.size(); ++i)
            {
                createRenderingOpsForNode (*orderedNodes.getUnchecked(i), i);
                markAnyUnusedBuffersAsFree (audioBuffers, i);
                markAnyUnusedBuffersAsFree (midiBuffers, i);
            }

            graph.setLatencySamples (totalLatency);

            s.numBuffersNeeded = audioBuffers.size();
            s.numMidiBuffersNeeded = midiBuffers.size();
        */
    }
    
    pub fn get_node_delay(&self, nodeid: RenderSequenceBuilderAudioProcessorGraphNodeID) -> i32 {
        
        todo!();
        /*
            return delays[nodeID.uid];
        */
    }
    
    pub fn get_input_latency_for_node(&self, nodeid: RenderSequenceBuilderAudioProcessorGraphNodeID) -> i32 {
        
        todo!();
        /*
            int maxLatency = 0;

            for (auto&& c : graph.getConnections())
                if (c.destination.nodeID == nodeID)
                    maxLatency = jmax (maxLatency, getNodeDelay (c.source.nodeID));

            return maxLatency;
        */
    }
    
    pub fn get_all_parents_of_node(
        child:         *const RenderSequenceBuilderAudioProcessorGraphNode,
        parents:       &mut HashSet<*mut RenderSequenceBuilderAudioProcessorGraphNode>,
        other_parents: &HashMap<*mut RenderSequenceBuilderAudioProcessorGraphNode,HashSet<*mut RenderSequenceBuilderAudioProcessorGraphNode>>)  {
        
        todo!();
        /*
            for (auto&& i : child->inputs)
            {
                auto* parentNode = i.otherNode;

                if (parentNode == child)
                    continue;

                if (parents.insert (parentNode).second)
                {
                    auto parentParents = otherParents.find (parentNode);

                    if (parentParents != otherParents.end())
                    {
                        parents.insert (parentParents->second.begin(), parentParents->second.end());
                        continue;
                    }

                    getAllParentsOfNode (i.otherNode, parents, otherParents);
                }
            }
        */
    }
    
    pub fn create_ordered_node_list(graph: &AudioProcessorGraph<'a>) 
        -> Vec<*mut RenderSequenceBuilderAudioProcessorGraphNode<'a>> 
    {
        todo!();
        /*
            Vec<RenderSequenceBuilderAudioProcessorGraphNode*> result;

            std::unordered_map<RenderSequenceBuilderAudioProcessorGraphNode*, std::unordered_set<RenderSequenceBuilderAudioProcessorGraphNode*>> nodeParents;

            for (auto* node : graph.getNodes())
            {
                int insertionIndex = 0;

                for (; insertionIndex < result.size(); ++insertionIndex)
                {
                    auto& parents = nodeParents[result.getUnchecked (insertionIndex)];

                    if (parents.find (node) != parents.end())
                        break;
                }

                result.insert (insertionIndex, node);
                getAllParentsOfNode (node, nodeParents[node], nodeParents);
            }

            return result;
        */
    }
    
    pub fn find_buffer_for_input_audio_channel(&mut self, 
        node:                &mut RenderSequenceBuilderAudioProcessorGraphNode,
        input_chan:          i32,
        our_rendering_index: i32,
        max_latency:         i32) -> i32 {
        
        todo!();
        /*
            auto& processor = *node.getProcessor();
            auto numOuts = processor.getTotalNumOutputChannels();

            auto sources = getSourcesForChannel (node, inputChan);

            // Handle an unconnected input channel...
            if (sources.isEmpty())
            {
                if (inputChan >= numOuts)
                    return readOnlyEmptyBufferIndex;

                auto index = getFreeBuffer (audioBuffers);
                sequence.addClearChannelOp (index);
                return index;
            }

            // Handle an input from a single source..
            if (sources.size() == 1)
            {
                // channel with a straightforward single input..
                auto src = sources.getUnchecked(0);

                int bufIndex = getBufferContaining (src);

                if (bufIndex < 0)
                {
                    // if not found, this is probably a feedback loop
                    bufIndex = readOnlyEmptyBufferIndex;
                    jassert (bufIndex >= 0);
                }

                if (inputChan < numOuts
                     && isBufferNeededLater (ourRenderingIndex, inputChan, src))
                {
                    // can't mess up this channel because it's needed later by another node,
                    // so we need to use a copy of it..
                    auto newFreeBuffer = getFreeBuffer (audioBuffers);
                    sequence.addCopyChannelOp (bufIndex, newFreeBuffer);
                    bufIndex = newFreeBuffer;
                }

                auto nodeDelay = getNodeDelay (src.nodeID);

                if (nodeDelay < maxLatency)
                    sequence.addDelayChannelOp (bufIndex, maxLatency - nodeDelay);

                return bufIndex;
            }

            // Handle a mix of several outputs coming into this input..
            int reusableInputIndex = -1;
            int bufIndex = -1;

            for (int i = 0; i < sources.size(); ++i)
            {
                auto src = sources.getReference(i);
                auto sourceBufIndex = getBufferContaining (src);

                if (sourceBufIndex >= 0 && ! isBufferNeededLater (ourRenderingIndex, inputChan, src))
                {
                    // we've found one of our input chans that can be re-used..
                    reusableInputIndex = i;
                    bufIndex = sourceBufIndex;

                    auto nodeDelay = getNodeDelay (src.nodeID);

                    if (nodeDelay < maxLatency)
                        sequence.addDelayChannelOp (bufIndex, maxLatency - nodeDelay);

                    break;
                }
            }

            if (reusableInputIndex < 0)
            {
                // can't re-use any of our input chans, so get a new one and copy everything into it..
                bufIndex = getFreeBuffer (audioBuffers);
                jassert (bufIndex != 0);

                audioBuffers.getReference (bufIndex).setAssignedToNonExistentNode();

                auto srcIndex = getBufferContaining (sources.getFirst());

                if (srcIndex < 0)
                    sequence.addClearChannelOp (bufIndex);  // if not found, this is probably a feedback loop
                else
                    sequence.addCopyChannelOp (srcIndex, bufIndex);

                reusableInputIndex = 0;
                auto nodeDelay = getNodeDelay (sources.getFirst().nodeID);

                if (nodeDelay < maxLatency)
                    sequence.addDelayChannelOp (bufIndex, maxLatency - nodeDelay);
            }

            for (int i = 0; i < sources.size(); ++i)
            {
                if (i != reusableInputIndex)
                {
                    auto src = sources.getReference(i);
                    int srcIndex = getBufferContaining (src);

                    if (srcIndex >= 0)
                    {
                        auto nodeDelay = getNodeDelay (src.nodeID);

                        if (nodeDelay < maxLatency)
                        {
                            if (! isBufferNeededLater (ourRenderingIndex, inputChan, src))
                            {
                                sequence.addDelayChannelOp (srcIndex, maxLatency - nodeDelay);
                            }
                            else // buffer is reused elsewhere, can't be delayed
                            {
                                auto bufferToDelay = getFreeBuffer (audioBuffers);
                                sequence.addCopyChannelOp (srcIndex, bufferToDelay);
                                sequence.addDelayChannelOp (bufferToDelay, maxLatency - nodeDelay);
                                srcIndex = bufferToDelay;
                            }
                        }

                        sequence.addAddChannelOp (srcIndex, bufIndex);
                    }
                }
            }

            return bufIndex;
        */
    }
    
    pub fn find_buffer_for_input_midi_channel(&mut self, 
        node:                &mut RenderSequenceBuilderAudioProcessorGraphNode,
        our_rendering_index: i32) -> i32 {
        
        todo!();
        /*
            auto& processor = *node.getProcessor();
            auto sources = getSourcesForChannel (node, AudioProcessorGraph::midiChannelIndex);

            // No midi inputs..
            if (sources.isEmpty())
            {
                auto midiBufferToUse = getFreeBuffer (midiBuffers); // need to pick a buffer even if the processor doesn't use midi

                if (processor.acceptsMidi() || processor.producesMidi())
                    sequence.addClearMidiBufferOp (midiBufferToUse);

                return midiBufferToUse;
            }

            // One midi input..
            if (sources.size() == 1)
            {
                auto src = sources.getReference (0);
                auto midiBufferToUse = getBufferContaining (src);

                if (midiBufferToUse >= 0)
                {
                    if (isBufferNeededLater (ourRenderingIndex, AudioProcessorGraph::midiChannelIndex, src))
                    {
                        // can't mess up this channel because it's needed later by another node, so we
                        // need to use a copy of it..
                        auto newFreeBuffer = getFreeBuffer (midiBuffers);
                        sequence.addCopyMidiBufferOp (midiBufferToUse, newFreeBuffer);
                        midiBufferToUse = newFreeBuffer;
                    }
                }
                else
                {
                    // probably a feedback loop, so just use an empty one..
                    midiBufferToUse = getFreeBuffer (midiBuffers); // need to pick a buffer even if the processor doesn't use midi
                }

                return midiBufferToUse;
            }

            // Multiple midi inputs..
            int midiBufferToUse = -1;
            int reusableInputIndex = -1;

            for (int i = 0; i < sources.size(); ++i)
            {
                auto src = sources.getReference (i);
                auto sourceBufIndex = getBufferContaining (src);

                if (sourceBufIndex >= 0
                     && ! isBufferNeededLater (ourRenderingIndex, AudioProcessorGraph::midiChannelIndex, src))
                {
                    // we've found one of our input buffers that can be re-used..
                    reusableInputIndex = i;
                    midiBufferToUse = sourceBufIndex;
                    break;
                }
            }

            if (reusableInputIndex < 0)
            {
                // can't re-use any of our input buffers, so get a new one and copy everything into it..
                midiBufferToUse = getFreeBuffer (midiBuffers);
                jassert (midiBufferToUse >= 0);

                auto srcIndex = getBufferContaining (sources.getUnchecked(0));

                if (srcIndex >= 0)
                    sequence.addCopyMidiBufferOp (srcIndex, midiBufferToUse);
                else
                    sequence.addClearMidiBufferOp (midiBufferToUse);

                reusableInputIndex = 0;
            }

            for (int i = 0; i < sources.size(); ++i)
            {
                if (i != reusableInputIndex)
                {
                    auto srcIndex = getBufferContaining (sources.getUnchecked(i));

                    if (srcIndex >= 0)
                        sequence.addAddMidiBufferOp (srcIndex, midiBufferToUse);
                }
            }

            return midiBufferToUse;
        */
    }
    
    pub fn create_rendering_ops_for_node(&mut self, 
        node:                &mut RenderSequenceBuilderAudioProcessorGraphNode,
        our_rendering_index: i32)  {
        
        todo!();
        /*
            auto& processor = *node.getProcessor();
            auto numIns  = processor.getTotalNumInputChannels();
            auto numOuts = processor.getTotalNumOutputChannels();
            auto totalChans = jmax (numIns, numOuts);

            Vec<int> audioChannelsToUse;
            auto maxLatency = getInputLatencyForNode (node.nodeID);

            for (int inputChan = 0; inputChan < numIns; ++inputChan)
            {
                // get a list of all the inputs to this node
                auto index = findBufferForInputAudioChannel (node, inputChan, ourRenderingIndex, maxLatency);
                jassert (index >= 0);

                audioChannelsToUse.add (index);

                if (inputChan < numOuts)
                    audioBuffers.getReference (index).channel = { node.nodeID, inputChan };
            }

            for (int outputChan = numIns; outputChan < numOuts; ++outputChan)
            {
                auto index = getFreeBuffer (audioBuffers);
                jassert (index != 0);
                audioChannelsToUse.add (index);

                audioBuffers.getReference (index).channel = { node.nodeID, outputChan };
            }

            auto midiBufferToUse = findBufferForInputMidiChannel (node, ourRenderingIndex);

            if (processor.producesMidi())
                midiBuffers.getReference (midiBufferToUse).channel = { node.nodeID, AudioProcessorGraph::midiChannelIndex };

            delays.set (node.nodeID.uid, maxLatency + processor.getLatencySamples());

            if (numOuts == 0)
                totalLatency = maxLatency;

            sequence.addProcessOp (node, audioChannelsToUse, totalChans, midiBufferToUse);
        */
    }
    
    pub fn get_sources_for_channel(
        &mut self, 
        node:                &mut RenderSequenceBuilderAudioProcessorGraphNode,
        input_channel_index: i32

    ) -> Vec<AudioProcessorGraphNodeAndChannel> {
        
        todo!();
        /*
            Vec<AudioProcessorGraph::AudioProcessorGraphNodeAndChannel> results;
            AudioProcessorGraph::AudioProcessorGraphNodeAndChannel nc { node.nodeID, inputChannelIndex };

            for (auto&& c : graph.getConnections())
                if (c.destination == nc)
                    results.add (c.source);

            return results;
        */
    }
    
    pub fn get_free_buffer(buffers: &mut Vec<RenderSequenceBuilderAssignedBuffer>) -> i32 {
        
        todo!();
        /*
            for (int i = 1; i < buffers.size(); ++i)
                if (buffers.getReference (i).isFree())
                    return i;

            buffers.add (RenderSequenceBuilderAssignedBuffer::createFree());
            return buffers.size() - 1;
        */
    }
    
    pub fn get_buffer_containing(&self, output: AudioProcessorGraphNodeAndChannel) -> i32 {
        
        todo!();
        /*
            int i = 0;

            for (auto& b : output.isMIDI() ? midiBuffers : audioBuffers)
            {
                if (b.channel == output)
                    return i;

                ++i;
            }

            return -1;
        */
    }
    
    pub fn mark_any_unused_buffers_as_free(&mut self, 
        buffers:    &mut Vec<RenderSequenceBuilderAssignedBuffer>,
        step_index: i32)  {
        
        todo!();
        /*
            for (auto& b : buffers)
                if (b.isAssigned() && ! isBufferNeededLater (stepIndex, -1, b.channel))
                    b.setFree();
        */
    }
    
    pub fn is_buffer_needed_later(
        &self, 
        step_index_to_search_from:        i32,
        input_channel_of_index_to_ignore: i32,
        output:                           AudioProcessorGraphNodeAndChannel

    ) -> bool {
        
        todo!();
        /*
            while (stepIndexToSearchFrom < orderedNodes.size())
            {
                auto* node = orderedNodes.getUnchecked (stepIndexToSearchFrom);

                if (output.isMIDI())
                {
                    if (inputChannelOfIndexToIgnore != AudioProcessorGraph::midiChannelIndex
                         && graph.isConnected ({ { output.nodeID, AudioProcessorGraph::midiChannelIndex },
                                                 { node->nodeID,  AudioProcessorGraph::midiChannelIndex } }))
                        return true;
                }
                else
                {
                    for (int i = 0; i < node->getProcessor()->getTotalNumInputChannels(); ++i)
                        if (i != inputChannelOfIndexToIgnore && graph.isConnected ({ output, { node->nodeID, i } }))
                            return true;
                }

                inputChannelOfIndexToIgnore = -1;
                ++stepIndexToSearchFrom;
            }

            return false;
        */
    }
}
