crate::ix!();

pub trait IsEmpty {

    /** 
      | Returns true if the buffer is empty.
      |
      | To actually retrieve the events, use
      | a MidiBufferIterator object
      */
    fn is_empty(&self) -> bool;
}

pub trait ClearFromStartWithLen {

    /** 
      | Removes all events between two times from the
      | buffer.
      |
      | All events for which (start <= event
      | position < start + numSamples) will be
      | removed.
      */
    fn clear_from_start_with_len(
        &mut self, 
        start_sample: i32,
        num_samples:  i32
    );
}

pub trait Clear {

    /**
      | Deletes all nodes and connections from
      | this graph.
      | 
      | Any processor objects in the graph will
      | be deleted.
      |
      */
    fn clear(&mut self);
}
