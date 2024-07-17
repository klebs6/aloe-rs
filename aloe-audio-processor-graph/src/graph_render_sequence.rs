crate::ix!();

pub struct GraphRenderSequence<FloatType> {
    num_buffers_needed:          i32, // default = 0
    num_midi_buffers_needed:     i32, // default = 0
    rendering_buffer:            AudioBuffer<FloatType>,
    current_audio_output_buffer: AudioBuffer<FloatType>,
    current_audio_input_buffer:  *mut AudioBuffer<FloatType>, // default = nullptr
    current_midi_input_buffer:   *mut MidiBuffer, // default = nullptr
    current_midi_output_buffer:  MidiBuffer,
    midi_buffers:                Vec<MidiBuffer>,
    midi_chunk:                  MidiBuffer,
    render_ops:                  Vec<Box<dyn GraphRenderSequenceRenderingOp<FloatType>>>,
}

impl<FloatType> Default for GraphRenderSequence<FloatType> {
    fn default() -> Self {
        todo!();
    }
}

impl<FloatType> GraphRenderSequence<FloatType> {

    pub fn perform(
        &mut self, 
        buffer:          &mut AudioBuffer<FloatType>,
        midi_messages:   &mut MidiBuffer,
        audio_play_head: *mut dyn AudioPlayHeadInterface

    ) {
        
        todo!();
        /*
            auto numSamples = buffer.getNumSamples();
            auto maxSamples = renderingBuffer.getNumSamples();

            if (numSamples > maxSamples)
            {
                // Being asked to render more samples than our buffers have, so divide the buffer into chunks
                int chunkStartSample = 0;
                while (chunkStartSample < numSamples)
                {
                    auto chunkSize = jmin (maxSamples, numSamples - chunkStartSample);

                    AudioBuffer<FloatType> audioChunk (buffer.getArrayOfWritePointers(), buffer.getNumChannels(), chunkStartSample, chunkSize);
                    midiChunk.clear();
                    midiChunk.addEvents (midiMessages, chunkStartSample, chunkSize, -chunkStartSample);

                    perform (audioChunk, midiChunk, audioPlayHead);

                    chunkStartSample += maxSamples;
                }

                return;
            }

            currentAudioInputBuffer = &buffer;
            currentAudioOutputBuffer.setSize (jmax (1, buffer.getNumChannels()), numSamples);
            currentAudioOutputBuffer.clear();
            currentMidiInputBuffer = &midiMessages;
            currentMidiOutputBuffer.clear();

            {
                const GraphRenderSequenceContext context { renderingBuffer.getArrayOfWritePointers(), midiBuffers.begin(), audioPlayHead, numSamples };

                for (auto* op : renderOps)
                    op->perform (context);
            }

            for (int i = 0; i < buffer.getNumChannels(); ++i)
                buffer.copyFrom (i, 0, currentAudioOutputBuffer, i, 0, numSamples);

            midiMessages.clear();
            midiMessages.addEvents (currentMidiOutputBuffer, 0, buffer.getNumSamples(), 0);
            currentAudioInputBuffer = nullptr;
        */
    }
    
    pub fn add_clear_channel_op(&mut self, index: i32)  {
        
        todo!();
        /*
            createOp ([=] (const GraphRenderSequenceContext& c)    { FloatVectorOperations::clear (c.audioBuffers[index], c.numSamples); });
        */
    }
    
    pub fn add_copy_channel_op(&mut self, 
        src_index: i32,
        dst_index: i32)  {
        
        todo!();
        /*
            createOp ([=] (const GraphRenderSequenceContext& c)    { FloatVectorOperations::copy (c.audioBuffers[dstIndex],
                                                                               c.audioBuffers[srcIndex],
                                                                               c.numSamples); });
        */
    }
    
    pub fn add_add_channel_op(&mut self, 
        src_index: i32,
        dst_index: i32)  {
        
        todo!();
        /*
            createOp ([=] (const GraphRenderSequenceContext& c)    { FloatVectorOperations::add (c.audioBuffers[dstIndex],
                                                                              c.audioBuffers[srcIndex],
                                                                              c.numSamples); });
        */
    }
    
    pub fn add_clear_midi_buffer_op(&mut self, index: i32)  {
        
        todo!();
        /*
            createOp ([=] (const GraphRenderSequenceContext& c)    { c.midiBuffers[index].clear(); });
        */
    }
    
    pub fn add_copy_midi_buffer_op(&mut self, 
        src_index: i32,
        dst_index: i32)  {
        
        todo!();
        /*
            createOp ([=] (const GraphRenderSequenceContext& c)    { c.midiBuffers[dstIndex] = c.midiBuffers[srcIndex]; });
        */
    }
    
    pub fn add_add_midi_buffer_op(&mut self, 
        src_index: i32,
        dst_index: i32)  {
        
        todo!();
        /*
            createOp ([=] (const GraphRenderSequenceContext& c)    { c.midiBuffers[dstIndex].addEvents (c.midiBuffers[srcIndex],
                                                                                     0, c.numSamples, 0); });
        */
    }
    
    pub fn add_delay_channel_op(&mut self, 
        chan:       i32,
        delay_size: i32)  {
        
        todo!();
        /*
            renderOps.add (new GraphRenderSequenceDelayChannelOp (chan, delaySize));
        */
    }
    
    pub fn add_process_op(&mut self, 
        node:                &AudioProcessorGraphNodePtr,
        audio_channels_used: &[i32],
        total_num_chans:     i32,
        midi_buffer:         i32)  {
        
        todo!();
        /*
            renderOps.add (new GraphRenderSequenceProcessOp (node, audioChannelsUsed, totalNumChans, midiBuffer));
        */
    }
    
    pub fn prepare_buffers(&mut self, block_size: i32)  {
        
        todo!();
        /*
            renderingBuffer.setSize (numBuffersNeeded + 1, blockSize);
            renderingBuffer.clear();
            currentAudioOutputBuffer.setSize (numBuffersNeeded + 1, blockSize);
            currentAudioOutputBuffer.clear();

            currentAudioInputBuffer = nullptr;
            currentMidiInputBuffer = nullptr;
            currentMidiOutputBuffer.clear();

            midiBuffers.clearQuick();
            midiBuffers.resize (numMidiBuffersNeeded);

            const int defaultMIDIBufferSize = 512;

            midiChunk.ensureSize (defaultMIDIBufferSize);

            for (auto&& m : midiBuffers)
                m.ensureSize (defaultMIDIBufferSize);
        */
    }
    
    pub fn release_buffers(&mut self)  {
        
        todo!();
        /*
            renderingBuffer.setSize (1, 1);
            currentAudioOutputBuffer.setSize (1, 1);
            currentAudioInputBuffer = nullptr;
            currentMidiInputBuffer = nullptr;
            currentMidiOutputBuffer.clear();
            midiBuffers.clear();
        */
    }
    
    pub fn create_op<LambdaType>(&mut self, fn_: LambdaType)  {
    
        todo!();
        /*
            struct LambdaOp  : public GraphRenderSequenceRenderingOp
            {
                LambdaOp (LambdaType&& f) : function (std::forward<LambdaType> (f)) {}
                void perform (const GraphRenderSequenceContext& c) override    { function (c); }

                LambdaType function;
            };

            renderOps.add (new LambdaOp (std::forward<LambdaType> (fn)));
        */
    }
}
