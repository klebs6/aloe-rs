crate::ix!();

pub trait GetMidiNoteName {

    /**
      | Returns the name of a midi note number.
      | E.g "C", "D#", etc.
      | 
      | -----------
      | @param noteNumber
      | 
      | the midi note number, 0 to 127
      | ----------
      | @param useSharps
      | 
      | if true, sharpened notes are used, e.g.
      | "C#", otherwise they'll be flattened,
      | e.g. "Db"
      | ----------
      | @param includeOctaveNumber
      | 
      | if true, the octave number will be appended
      | to the string, e.g. "C#4"
      | ----------
      | @param octaveNumForMiddleC
      | 
      | if an octave number is being appended,
      | this indicates the number that will
      | be used for middle C's octave
      | 
      | @see getMidiNoteInHertz
      |
      */
    fn get_midi_note_name(
        &mut self, 
        note:                   i32,
        use_sharps:             bool,
        include_octave_number:  bool,
        octave_num_for_middlec: i32

    ) -> String;
}

pub trait GetMidiNoteInHertz {

    /**
      | Returns the frequency of a midi note
      | number.
      | 
      | -----------
      | @note
      | 
      | The frequencyOfA parameter is an optional
      | frequency for 'A', normally 440-444Hz
      | for concert pitch.
      | 
      | @see getMidiNoteName
      |
      */
    fn get_midi_note_in_hertz(
        &mut self, 
        note_number:   i32,
        frequency_ofa: Option<f64>

    ) -> f64;
}

pub trait IsMidiNoteBlack {

    /**
      | Returns true if the given midi note number
      | is a black key.
      |
      */
    fn is_midi_note_black(&mut self, note_number: i32) -> bool;
}
