crate::ix!();

/**
  | PolyPressure event specific data.
  | Used in \ref VstEvent (union) \ingroup
  | vstEventGrp
  |
  */
#[derive(Copy,Clone)]
pub struct PolyPressureEvent
{
    /**
      | channel index in event bus
      |
      */
    channel:  i16,

    /**
      | range [0, 127] = [C-2, G8] with A3=440Hz
      |
      */
    pitch:    i16,

    /**
      | range [0.0, 1.0]
      |
      */
    pressure: f32,

    /**
      | event should be applied to the noteId
      | (if not -1)
      |
      */
    note_id:  i32,
}
