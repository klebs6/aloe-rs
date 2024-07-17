crate::ix!();

pub trait GetStateInformation {

    /**
      | The host will call this method when it
      | wants to save the processor's internal
      | state.
      | 
      | This must copy any info about the processor's
      | state into the block of memory provided,
      | so that the host can store this and later
      | restore it using setStateInformation().
      | 
      | -----------
      | @note
      | 
      | there's also a getCurrentProgramStateInformation()
      | method, which only stores the current
      | program, not the state of the entire
      | processor.
      | 
      | See also the helper function copyXmlToBinary()
      | for storing settings as XML.
      | 
      | @see getCurrentProgramStateInformation
      |
      */
    fn get_state_information(&mut self, dest_data: &mut MemoryBlock);
}

pub trait GetCurrentProgramStateInformation {

    /**
      | The host will call this method if it wants
      | to save the state of just the processor's
      | current program.
      | 
      | Unlike getStateInformation, this
      | should only return the current program's
      | state.
      | 
      | Not all hosts support this, and if you
      | don't implement it, the base class method
      | just calls getStateInformation()
      | instead. If you do implement it, be sure
      | to also implement getCurrentProgramStateInformation.
      | 
      | @see getStateInformation, setCurrentProgramStateInformation
      |
      */
    fn get_current_program_state_information(&mut self, dest_data: &mut MemoryBlock);
}

pub trait SetStateInformation {

    /**
      | This must restore the processor's state
      | from a block of data previously created
      | using getStateInformation().
      | 
      | -----------
      | @note
      | 
      | there's also a setCurrentProgramStateInformation()
      | method, which tries to restore just
      | the current program, not the state of
      | the entire processor.
      | 
      | See also the helper function getXmlFromBinary()
      | for loading settings as XML.
      | 
      | @see setCurrentProgramStateInformation
      |
      */
    fn set_state_information(&mut self, 
            data:          *const c_void,
            size_in_bytes: i32);
}

pub trait SetCurrentProgramStateInformation {

    /**
      | The host will call this method if it wants
      | to restore the state of just the processor's
      | current program.
      | 
      | Not all hosts support this, and if you
      | don't implement it, the base class method
      | just calls setStateInformation()
      | instead. If you do implement it, be sure
      | to also implement getCurrentProgramStateInformation.
      | 
      | @see setStateInformation, getCurrentProgramStateInformation
      |
      */
    fn set_current_program_state_information(&mut self, 
            data:          *const c_void,
            size_in_bytes: i32);

}
