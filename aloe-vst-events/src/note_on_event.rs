crate::ix!();

/**
  | Note-on event specific data. Used in
  | \ref Event (union) \ingroup vstEventGrp
  | 
  | Pitch uses the twelve-tone equal temperament
  | tuning (12-TET).
  |
  */
#[derive(Copy,Clone)]
pub struct NoteOnEvent
{
    /**
      | channel index in event bus
      |
      */
    channel:  i16,

    /**
      | range [0, 127] = [C-2, G8] with A3=440Hz
      | (12-TET: twelve-tone equal temperament)
      |
      */
    pitch:    i16,

    /**
      | 1.f = +1 cent, -1.f = -1 cent
      |
      */
    tuning:   f32,

    /**
      | range [0.0, 1.0]
      |
      */
    velocity: f32,

    /**
      | in sample frames (optional, Note Off
      | has to follow in any case!)
      |
      */
    length:   i32,

    /**
      | note identifier (if not available then
      | -1)
      |
      */
    note_id:  i32,
}
