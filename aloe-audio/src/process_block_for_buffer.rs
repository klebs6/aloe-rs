crate::ix!();

pub fn process_block_for_buffer<FloatType, SequenceType>(
        buffer:          &mut AudioBuffer<FloatType>,
        midi_messages:   &mut MidiBuffer,
        graph:           &mut AudioProcessorGraph,
        render_sequence: &mut Box<SequenceType>,
        is_prepared:     &mut AtomicBool)  {

    todo!();
        /*
            if (graph.isNonRealtime())
        {
            while (! isPrepared)
                Thread::sleep (1);

            const ScopedLock sl (graph.getCallbackLock());

            if (renderSequence != nullptr)
                renderSequence->perform (buffer, midiMessages, graph.getPlayHead());
        }
        else
        {
            const ScopedLock sl (graph.getCallbackLock());

            if (isPrepared)
            {
                if (renderSequence != nullptr)
                    renderSequence->perform (buffer, midiMessages, graph.getPlayHead());
            }
            else
            {
                buffer.clear();
                midiMessages.clear();
            }
        }
        */
}

