crate::ix!();

pub enum ChordMasks {

    /**
      | mask for chordMask
      |
      */
    ChordMask    = 0x0FFF,    

    /**
      | reserved for future use
      |
      */
    ReservedMask = 0xF000  
}

/**
  | Description of a chord.
  | 
  | A chord is described with a key note,
  | a root note and the \copydoc chordMask
  | \see ProcessContext
  |
  */
pub struct Chord
{
    /**
      | key note in chord
      |
      */
    key_note:  u8,

    /**
      | lowest note in chord
      |
      */
    root_note: u8,

    /** 
      | Bitmask of a chord. \n
      |
      |    1st bit set: minor second; 2nd bit set:
      |    major second, and so on. \n
      |
      |    There is \b no bit for the keynote
      |    (root of the chord) because it is inherently
      |    always present. \n
      |
      |    Examples:
      |    - XXXX 0000 0100 1000 (= 0x0048) -> major chord
      |    - XXXX 0000 0100 0100 (= 0x0044) -> minor chord
      |    - XXXX 0010 0100 0100 (= 0x0244) -> minor chord with minor seventh 
      */
    chord_mask: i16,
}
