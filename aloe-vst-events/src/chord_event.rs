crate::ix!();

/**
  | Chord event specific data. Used in \ref
  | VstEvent (union) \ingroup vstEventGrp
  |
  */
#[derive(Copy,Clone)]
pub struct ChordEvent
{
    /**
      | range [0, 127] = [C-2, G8] with A3=440Hz
      |
      */
    root:      i16,

    /**
      | range [0, 127] = [C-2, G8] with A3=440Hz
      |
      */
    bass_note: i16,

    /**
      | root is bit 0
      |
      */
    mask:      i16,

    /**
      | the number of characters (TChar) between
      | the beginning of text and the terminating
      | null character (without including the
      | terminating null character itself)
      */
    text_len:  u16,

    /**
      | UTF-16, null terminated Hosts Chord
      | Name
      |
      */
    text:      *const TChar,
}
