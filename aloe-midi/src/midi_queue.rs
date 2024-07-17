crate::ix!();

pub const MIDI_QUEUE_SIZE: usize = 1 << 14;

pub struct MidiQueue {
    fifo:     AbstractFifo,     // default = { queueSize  }
    messages: Vec<MidiMessage>, //= std::vector<MidiMessage> (queueSize);
}

impl MidiQueue {
    
    pub fn push(&mut self, buffer: &MidiBuffer)  {
        
        todo!();
        /*
            for (const auto metadata : buffer)
                fifo.write (1).forEach ([&] (int dest) { messages[(size_t) dest] = metadata.getMessage(); });
        */
    }
    
    pub fn pop<OutputIt>(&mut self, out: OutputIt)  {
    
        todo!();
        /*
            fifo.read (fifo.getNumReady()).forEach ([&] (int source) { *out++ = messages[(size_t) source]; });
        */
    }
}
