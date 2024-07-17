crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPIterator.h]

/**
  | Enables iteration over a collection
  | of Universal MIDI Packets stored as
  | a contiguous range of 32-bit words.
  | 
  | This iterator is used by Packets to allow
  | access to the messages that it contains.
  | 
  | @tags{Audio}
  |
  */
#[derive(Default)] /** Creates an invalid (singular) iterator. */
pub struct UniversalMidiPacketsIterator {

    view: UniversalMidiPacketsView,

    #[cfg(ALOE_DEBUG)]
    bytes_remaining: usize, // default = 0
}

lazy_static!{
    /*
       pub mod iterator {
           using difference_type    = typename iterator_traits<const uint32_t*>::difference_type;
           using value_type         = UniversalMidiPacketsView;
           using reference          = const UniversalMidiPacketsView&;
           using pointer            = const UniversalMidiPacketsView*;
           using iterator_category  = std::forward_iterator_tag;
       }
       */
}

impl PartialEq<UniversalMidiPacketsIterator> for UniversalMidiPacketsIterator {
    
    /**
      | Returns true if this iterator points
      | to the same address as another iterator.
      |
      */
    #[inline] fn eq(&self, other: &UniversalMidiPacketsIterator) -> bool {
        todo!();
        /*
            return view == other.view;
        */
    }
}

impl Eq for UniversalMidiPacketsIterator {}

impl Deref for UniversalMidiPacketsIterator {

    type Target = UniversalMidiPacketsView;

    /**
      | Returns a reference to a UniversalMidiPacketsView of the packet
      | currently pointed-to by this iterator.
      | 
      | The UniversalMidiPacketsView can be queried for its size and
      | content.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return view;
        */
    }
}

impl UniversalMidiPacketsIterator {

    /**
      | Creates an iterator pointing at `ptr`.
      |
      */
    pub fn new(
        ptr:   *const u32,
        bytes: usize) -> Self {
    
        todo!();
        /*


            : view (ptr)
           #if ALOE_DEBUG
            , bytesRemaining (bytes)
           #endif
            ignoreUnused (bytes);
        */
    }

    /**
      | Moves this iterator to the next packet
      | in the range.
      |
      */
    pub fn prefix_increment(&mut self) -> &mut UniversalMidiPacketsIterator {
        
        todo!();
        /*
            const auto increment = view.size();

           #if ALOE_DEBUG
            // If you hit this, the memory region contained a truncated or otherwise
            // malformed Universal MIDI Packet.
            // The UniversalMidiPacketsIterator can only be used on regions containing complete packets!
            jassert (increment <= bytesRemaining);
            bytesRemaining -= increment;
           #endif

            view = UniversalMidiPacketsView (view.data() + increment);
            return *this;
        */
    }

    /**
      | Moves this iterator to the next packet
      | in the range, returning the value of
      | the iterator before it was incremented.
      |
      */
    pub fn postfix_increment(&mut self) -> UniversalMidiPacketsIterator {
        
        todo!();
        /*
            auto copy = *this;
            ++(*this);
            return copy;
        */
    }
}
