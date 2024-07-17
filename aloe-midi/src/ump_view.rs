crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPView.h]

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPView.cpp]

/**
  | Points to a single Universal MIDI Packet.
  | 
  | The packet must be well-formed for member
  | functions to work correctly.
  | 
  | Specifically, the constructor argument
  | must be the beginning of a region of uint32_t
  | that contains at least `getNumWordsForMessageType(*ddata)`
  | items, where `data` is the constructor
  | argument.
  | 
  | NOTE: Instances of this class do not
  | own the memory that they point to! If
  | you need to store a packet pointed-to
  | by a UniversalMidiPacketsView for later use, copy the view
  | contents to a Packets collection, or
  | use the Utils::PacketX types.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsView {
    ptr: *const u32,
}

impl Default for UniversalMidiPacketsView {

    /** Create an invalid view. */
    fn default() -> Self {
        Self {
            ptr: null_mut()
        }
    }
}

impl Index<usize> for UniversalMidiPacketsView {

    type Output = u32;

    /**
      | Get a specific word from this packet.
      | 
      | Passing an `index` that is greater than
      | or equal to the result of `size` will
      | cause undefined behaviour.
      |
      */
    #[inline] fn index(&self, index: usize) -> &Self::Output {
        todo!();
        /*
            return ptr[index];
        */
    }
}

impl PartialEq<UniversalMidiPacketsView> for UniversalMidiPacketsView {
    
    /**
      | Return true if this view is pointing
      | to the same address as another view.
      |
      */
    #[inline] fn eq(&self, other: &UniversalMidiPacketsView) -> bool {
        todo!();
        /*
            return ptr == other.ptr;
        */
    }
}

impl Eq for UniversalMidiPacketsView {}

impl UniversalMidiPacketsView {

    /**
      | Create a view of the packet starting
      | at address `d`.
      |
      */
    pub fn new(data: *const u32) -> Self {
    
        todo!();
        /*
        : ptr(data),
        */
    }

    /**
      | Get a pointer to the first word in the
      | Universal MIDI Packet currently pointed-to
      | by this view.
      |
      */
    pub fn data(&self) -> *const u32 {
        
        todo!();
        /*
            return ptr;
        */
    }

    /**
      | Get an iterator pointing to the first
      | word in the packet.
      |
      */
    pub fn begin(&self) -> *const u32 {
        
        todo!();
        /*
            return ptr;
        */
    }
    
    pub fn cbegin(&self) -> *const u32 {
        
        todo!();
        /*
            return ptr;
        */
    }

    /**
      | Get an iterator pointing one-past the
      | last word in the packet.
      |
      */
    pub fn end(&self) -> *const u32 {
        
        todo!();
        /*
            return ptr + size();
        */
    }
    
    pub fn cend(&self) -> *const u32 {
        
        todo!();
        /*
            return ptr + size();
        */
    }
    
    /**
      | Get the number of 32-words (between
      | 1 and 4 inclusive) in the Universal MIDI
      | Packet currently pointed-to by this
      | view.
      |
      */
    pub fn size(&self) -> u32 {
        
        todo!();
        /*
            jassert (ptr != nullptr);
        return Utils::getNumWordsForMessageType (*ptr);
        */
    }
}
