crate::ix!();

pub trait IsSongPositionPointer {

    /**
      | Returns true if this is a song-position-pointer
      | message. @see getSongPositionPointerMidiBeat,
      | songPositionPointer
      */
    fn is_song_position_pointer(&self) -> bool;
}

pub trait GetSongPositionPointerMidiBeat {

    /**
      | Returns the midi beat-number of a song-position-pointer
      | message. @see isSongPositionPointer,
      | songPositionPointer
      |
      */
    fn get_song_position_pointer_midi_beat(&self) -> i32;
}

pub trait SongPositionPointer {

    /**
      | Creates a song-position-pointer message.
      | 
      | -----------
      | @note
      | 
      | The position is a number of midi beats
      | from the start of the song, where 1 midi
      | beat is 6 midi clocks, and there are 24
      | midi clocks in a quarter-note. So there
      | are 4 midi beats in a quarter-note.
      | 
      | @see isSongPositionPointer, getSongPositionPointerMidiBeat
      |
      */
    fn song_position_pointer(&mut self, position_in_midi_beats: i32) -> dyn MidiMessageInterface;
}
