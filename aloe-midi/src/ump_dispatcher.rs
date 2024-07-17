crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPDispatcher.h]

/**
  | Parses a raw stream of uint32_t, and
  | calls a user-provided callback every
  | time a full Universal MIDI Packet is
  | encountered.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsDispatcher {
    next_packet:        [u32; 4],
    current_packet_len: usize, // default = 0
}

impl UniversalMidiPacketsDispatcher {

    /**
      | Clears the dispatcher.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            currentPacketLen = 0;
        */
    }

    /**
      | Calls `callback` with a View of each
      | packet encountered in the range delimited
      | by `begin` and `end`.
      | 
      | If the range ends part-way through a
      | packet, the next call to `dispatch`
      | will continue from that point in the
      | packet (unless `reset` is called first).
      |
      */
    pub fn dispatch<PacketCallbackFunction>(&mut self, 
        begin:      *const u32,
        end:        *const u32,
        time_stamp: f64,
        callback:   PacketCallbackFunction)  {
    
        todo!();
        /*
            std::for_each (begin, end, [&] (uint32_t word)
            {
                nextPacket[currentPacketLen++] = word;

                if (currentPacketLen == Utils::getNumWordsForMessageType (nextPacket.front()))
                {
                    callback (View (nextPacket.data()), timeStamp);
                    currentPacketLen = 0;
                }
            });
        */
    }
}
