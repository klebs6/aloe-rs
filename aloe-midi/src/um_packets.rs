crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPackets.h]

/**
  | Holds a collection of Universal MIDI
  | UniversalMidiPackets.
  | 
  | Unlike MidiBuffer, this collection
  | does not store any additional information
  | (e.g. timestamps) alongside the raw
  | messages.
  | 
  | If timestamps are required, these can
  | be added to the container in UMP format,
  | as Jitter Reduction Utility messages.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPackets {
    storage: Vec<u32>,
}

impl UniversalMidiPackets {

    /**
      | Adds a single packet to the collection.
      | 
      | The UniversalMidiPacketsView must be valid for this to work.
      | If the view points to a malformed message,
      | or if the view points to a region too short
      | for the contained message, this call
      | will result in undefined behaviour.
      |
      */
    pub fn add_with_view(&mut self, v: &UniversalMidiPacketsView)  {
        
        todo!();
        /*
            storage.insert (storage.end(), v.cbegin(), v.cend());
        */
    }
    
    pub fn add_x1(&mut self, p: &UniversalMidiPacketX1)  {
        
        todo!();
        /*
            addImpl (p);
        */
    }
    
    pub fn add_x2(&mut self, p: &UniversalMidiPacketX2)  {
        
        todo!();
        /*
            addImpl (p);
        */
    }
    
    pub fn add_x3(&mut self, p: &UniversalMidiPacketX3)  {
        
        todo!();
        /*
            addImpl (p);
        */
    }
    
    pub fn add_x4(&mut self, p: &UniversalMidiPacketX4)  {
        
        todo!();
        /*
            addImpl (p);
        */
    }

    /**
      | Pre-allocates space for at least `UNIVERSAL_MIDI_PACKET_NUM_WORDS`
      | 32-bit words in this collection.
      |
      */
    pub fn reserve(&mut self, num_words: usize)  {
        
        todo!();
        /*
            storage.reserve (UNIVERSAL_MIDI_PACKET_NUM_WORDS);
        */
    }

    /**
      | Removes all previously-added packets
      | from this collection.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            storage.clear();
        */
    }

    /**
      | Gets an iterator pointing to the first
      | packet in this collection.
      |
      */
    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.storage.iter()
    }

    // Gets an iterator over mutable references
    pub fn iter_mut(&mut self) -> std::slice::IterMut<u32> {
        self.storage.iter_mut()
    }

    /**
      | Gets a pointer to the contents of the
      | collection as a range of raw 32-bit words.
      |
      */
    pub fn data(&self) -> *const u32 {
        
        todo!();
        /*
            return storage.data();
        */
    }

    /**
      | Returns the number of uint32_t words
      | in storage.
      | 
      | Note that this is likely to be larger
      | than the number of packets currently
      | being stored, as some packets span multiple
      | words.
      |
      */
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return storage.size();
        */
    }
    
    pub fn add_impl(
        &mut self, 
        p: &UniversalMidiPackets

    ) {
    
        todo!();
        /*
            jassert (Utils::getNumWordsForMessageType (p[0]) == UNIVERSAL_MIDI_PACKET_NUM_WORDS);
            add (UniversalMidiPacketsView (p.data()));
        */
    }
}
