crate::ix!();

/**
  | Note-off event specific data. Used
  | in \ref VstEvent (union) \ingroup vstEventGrp
  |
  */
#[derive(Copy,Clone)]
pub struct NoteOffEvent
{
    /**
      | channel index in event bus
      |
      */
    channel:  i16,

    /**
      | range [0, 127] = [C-2, G8] with A3=440Hz
      | (12-TET)
      |
      */
    pitch:    i16,

    /**
      | range [0.0, 1.0]
      |
      */
    velocity: f32,

    /**
      | associated noteOn identifier (if not
      | available then -1)
      |
      */
    note_id:  i32,

    /**
      | 1.f = +1 cent, -1.f = -1 cent
      |
      */
    tuning:   f32,
}
