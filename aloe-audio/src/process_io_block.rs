crate::ix!();

pub fn process_io_block<FloatType, SequenceType>(
        io:            &mut AudioGraphIOProcessor,
        sequence:      &mut SequenceType,
        buffer:        &mut AudioBuffer<FloatType>,
        midi_messages: &mut MidiBuffer)  {

    todo!();
        /*
            switch (io.getType())
        {
            case AudioProcessorGraph::AudioGraphIOProcessor::audioOutputNode:
            {
                auto&& currentAudioOutputBuffer = sequence.currentAudioOutputBuffer;

                for (int i = jmin (currentAudioOutputBuffer.getNumChannels(), buffer.getNumChannels()); --i >= 0;)
                    currentAudioOutputBuffer.addFrom (i, 0, buffer, i, 0, buffer.getNumSamples());

                break;
            }

            case AudioProcessorGraph::AudioGraphIOProcessor::audioInputNode:
            {
                auto* currentInputBuffer = sequence.currentAudioInputBuffer;

                for (int i = jmin (currentInputBuffer->getNumChannels(), buffer.getNumChannels()); --i >= 0;)
                    buffer.copyFrom (i, 0, *currentInputBuffer, i, 0, buffer.getNumSamples());

                break;
            }

            case AudioProcessorGraph::AudioGraphIOProcessor::midiOutputNode:
                sequence.currentMidiOutputBuffer.addEvents (midiMessages, 0, buffer.getNumSamples(), 0);
                break;

            case AudioProcessorGraph::AudioGraphIOProcessor::midiInputNode:
                midiMessages.addEvents (*sequence.currentMidiInputBuffer, 0, buffer.getNumSamples(), 0);
                break;

            default:
                break;
        }
        */
}
