crate::ix!();

/**
  | An iterator to move over contiguous raw MIDI
  | data, which Allows iterating over a MidiBuffer
  | using C++11 range-for syntax.
  |
  | In the following example, we log all
  | three-byte messages in a midi buffer.
  |
  | @code
  | void processBlock (
  | AudioBuffer<float>&, MidiBuffer& midiBuffer) override
  | {
  |     for (const MidiMessageMetadata metadata : midiBuffer)
  |         if (metadata.numBytes == 3)
  |             Logger::writeToLog (metadata.getMessage().getDescription());
  | }
  | @endcode
  |
  | @tags{Audio}
  */
pub struct MidiBufferIterator {
    data: <Self as HasPtr>::Ptr, // default = nullptr
}

impl Default for MidiBufferIterator {

    fn default() -> Self {
        Self {
            data: null_mut()
        }
    }
}

impl Deref for MidiBufferIterator {

    type Target = MidiBufferIterator;
    
    /** 
      | Return an instance of MidiMessageMetadata
      | which describes the message to which the
      | iterator is currently pointing.
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return { data + sizeof (int32) + sizeof (uint16),
                 MidiBufferHelpers::getEventDataSize (data),
                 MidiBufferHelpers::getEventTime (data) };
        */
    }
}

pub trait HasPtr {
    type Ptr;
}

impl HasPtr for MidiBufferIterator {
    type Ptr = *const u8;
}

impl HasReference for MidiBufferIterator {
    type Reference = MidiMessageMetadata;
}

impl HasValueType for MidiBufferIterator {
    type ValueType = MidiMessageMetadata;
}

impl HasPointer for MidiBufferIterator {
    type Pointer = c_void;
}

impl HasDifferenceType for MidiBufferIterator {
    type DifferenceType = usize;
}

impl PartialEq<MidiBufferIterator> for MidiBufferIterator {
    
    /** 
      | Return true if this iterator points to the
      | same message as another iterator instance,
      | otherwise return false.
      */
    #[inline] fn eq(&self, other: &MidiBufferIterator) -> bool {
        todo!();
        /*
            return data == other.data;
        */
    }
}

impl Eq for MidiBufferIterator {}

impl MidiBufferIterator {

    /** 
      | Constructs an iterator pointing at the
      | message starting at the byte `dataIn`.
      | `dataIn` must point to the start of a valid
      | MIDI message. If it does not, calling other
      | member functions on the iterator will result
      | in undefined behaviour.
      */
    pub fn new(data_in: *const u8) -> Self {

        todo!();
        /*
        : data(dataIn),

        
        */
    }

    /**
      | Make this iterator point to the next
      | message in the buffer.
      |
      */
    pub fn prefix_increment(&mut self) -> &mut MidiBufferIterator {
        
        todo!();
        /*
            data += sizeof (int32) + sizeof (uint16) + size_t (MidiBufferHelpers::getEventDataSize (data));
        return *this;
        */
    }
    
    /** 
      | Create a copy of this object, make this
      | iterator point to the next message in the
      | buffer, then return the copy.
      */
    pub fn postfix_increment(&mut self) -> MidiBufferIterator {
        
        todo!();
        /*
            auto copy = *this;
        ++(*this);
        return copy;
        */
    }
}
