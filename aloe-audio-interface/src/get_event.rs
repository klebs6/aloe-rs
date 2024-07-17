crate::ix!();

pub trait GetNumEvents {

    /** 
      | Counts the number of events in the buffer.
      |
      | This is actually quite a slow operation,
      | as it has to iterate through all the
      | events, so you might prefer to call
      | isEmpty() if that's all you need to know.
      */
    fn get_num_events(&self) -> i32;
}

pub trait GetFirstEventTime {

    /** 
      | Returns the sample number of the first event
      | in the buffer.
      |
      | If the buffer's empty, this will just
      | return 0.
      */
    fn get_first_event_time(&self) -> i32;
}

pub trait GetLastEventTime {

    /** 
      | Returns the sample number of the last event
      | in the buffer.
      |
      | If the buffer's empty, this will just return 0.
      */
    fn get_last_event_time(&self) -> i32;
}
