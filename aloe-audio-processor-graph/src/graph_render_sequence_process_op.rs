crate::ix!();

#[no_copy]
pub struct GraphRenderSequenceProcessOp<'a, FloatType: num::Float> {
    node:                  AudioProcessorGraphNodePtr,
    processor:             &'a mut AudioProcessor<'a>,
    audio_channels_to_use: Vec<i32>,
    audio_channels:        HeapBlock<*mut FloatType>,
    temp_buffer_float:     AudioBuffer<f32>,
    temp_buffer_double:    AudioBuffer<f32>,
    total_chans:           i32,
    midi_buffer_to_use:    i32,
}

impl<'a, FloatType: num::Float> GraphRenderSequenceRenderingOp<FloatType> for GraphRenderSequenceProcessOp<'a, FloatType>
{
    fn perform(&mut self, c: &GraphRenderSequenceContext<FloatType>)  {
        
        todo!();
        /*
            processor.setPlayHead (c.audioPlayHead);

                for (int i = 0; i < totalChans; ++i)
                    audioChannels[i] = c.audioBuffers[audioChannelsToUse.getUnchecked (i)];

                AudioBuffer<FloatType> buffer (audioChannels, totalChans, c.numSamples);

                if (processor.isSuspended())
                    buffer.clear();
                else
                    callProcess (buffer, c.midiBuffers[midiBufferToUse]);
        */
    }
}

impl<'a, FloatType: num::Float> GraphRenderSequenceProcessOp<'a, FloatType> {

    pub fn new(
        n:                   &AudioProcessorGraphNodePtr,
        audio_channels_used: &[i32],
        total_num_chans:     i32,
        midi_buffer:         i32) -> Self {
    
        todo!();
        /*


            : node (n),
                  processor (*n->getProcessor()),
                  audioChannelsToUse (audioChannelsUsed),
                  totalChans (jmax (1, totalNumChans)),
                  midiBufferToUse (midiBuffer)

                audioChannels.calloc ((size_t) totalChans);

                while (audioChannelsToUse.size() < totalChans)
                    audioChannelsToUse.add (0);
        */
    }
    
    pub fn call_process(
        &mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer

    ) {
        
        todo!();
        /*
            if (processor.isUsingDoublePrecision())
                {
                    tempBufferDouble.makeCopyOf (buffer, true);

                    if (node->isBypassed())
                        node->processBlockBypassed (tempBufferDouble, midiMessages);
                    else
                        node->processBlock (tempBufferDouble, midiMessages);

                    buffer.makeCopyOf (tempBufferDouble, true);
                }
                else
                {
                    if (node->isBypassed())
                        node->processBlockBypassed (buffer, midiMessages);
                    else
                        node->processBlock (buffer, midiMessages);
                }
        */
    }
    
    pub fn call_process_f64(
        &mut self, 
        buffer:        &mut AudioBuffer<f64>,
        midi_messages: &mut MidiBuffer

    ) {
        
        todo!();
        /*
            if (processor.isUsingDoublePrecision())
                {
                    if (node->isBypassed())
                        node->processBlockBypassed (buffer, midiMessages);
                    else
                        node->processBlock (buffer, midiMessages);
                }
                else
                {
                    tempBufferFloat.makeCopyOf (buffer, true);

                    if (node->isBypassed())
                        node->processBlockBypassed (tempBufferFloat, midiMessages);
                    else
                        node->processBlock (tempBufferFloat, midiMessages);

                    buffer.makeCopyOf (tempBufferFloat, true);
                }
        */
    }
}
