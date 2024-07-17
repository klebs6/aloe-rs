crate::ix!();

/**
  | Note Expression Value event. Used in
  | \ref VstEvent (union)
  | 
  | A note expression event affects one
  | single playing note (referring its
  | noteId).
  | 
  | This kind of event is send from host to
  | the plug-in like other events (NoteOnEvent,
  | NoteOffEvent,...) in \ref ProcessData
  | during the process call.
  | 
  | Note expression events for a specific
  | noteId can only occur after a NoteOnEvent.
  | The host must take care that the event
  | list (\ref IEventList) is properly
  | sorted.
  | 
  | Expression events are always absolute
  | normalized values [0.0, 1.0].
  | 
  | The predefined types have a predefined
  | mapping of the normalized values (see
  | \ref NoteExpressionTypeIDs) \sa INoteExpressionController
  |
  */
#[derive(Copy,Clone)]
pub struct NoteExpressionValueEvent
{
    /**
      | see \ref NoteExpressionTypeID
      |
      */
    type_id: NoteExpressionTypeID,

    /**
      | associated note identifier to apply
      | the change
      |
      */
    note_id: i32,

    /**
      | normalized value [0.0, 1.0].
      |
      */
    value:   NoteExpressionValue,
}
