crate::ix!();

pub trait MidiBufferIteratorInterface {}

pub trait MidiBufferInterface 
: SwapWith

/*
  | Removes all events from the buffer.
  |
  */
+ Clear
+ EnsureSize
+ IsEmpty
+ ClearFromStartWithLen
+ AddEventWithMessageAndSampleNumber
+ AddEvent
+ AddEvents
+ GetNumEvents
+ GetFirstEventTime
+ GetLastEventTime
+ FindNextSamplePosition
{ }
