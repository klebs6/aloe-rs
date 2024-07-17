crate::ix!();

pub trait IsProgramChange {

    /** 
      | Returns true if the message is a program
      | (patch) change message.
      | @see getProgramChangeNumber,
      | getGMInstrumentName
      */
    fn is_program_change(&self) -> bool;
}

pub trait GetProgramChangeNumber {

    /** 
     | Returns the new program number of a program
     | change message.  If the message isn't
     | a program change, the value returned is
     | undefined.  @see isProgramChange,
     | getGMInstrumentName
    */
    fn get_program_change_number(&self) -> i32;
}

pub trait ProgramChange {

    /**
      | Creates a program-change message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | ----------
      | @param programNumber
      | 
      | the midi program number, 0 to 127
      | 
      | @see isProgramChange, getGMInstrumentName
      |
      */
    fn program_change(
        &mut self, 
        channel:        i32,
        program_number: i32
    ) -> dyn MidiMessageInterface;
}
