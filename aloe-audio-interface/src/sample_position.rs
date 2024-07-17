crate::ix!();

pub trait FindNextSamplePosition {

    /**
      | Get an iterator pointing to the first
      | event with a timestamp greater-than
      | or equal-to `samplePosition`.
      |
      */
    fn find_next_sample_position(&self, sample_position: i32) -> dyn MidiBufferIteratorInterface;
}
