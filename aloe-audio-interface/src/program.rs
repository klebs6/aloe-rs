crate::ix!();

pub trait GetNumPrograms {

    /**
      | Returns the number of preset programs
      | the processor supports.
      | 
      | The value returned must be valid as soon
      | as this object is created, and must not
      | change over its lifetime.
      | 
      | This value shouldn't be less than 1.
      |
      */
    fn get_num_programs(&mut self) -> i32 { 0 }
}

pub trait GetCurrentProgram {

    /**
      | Returns the number of the currently
      | active program.
      |
      */
    fn get_current_program(&mut self) -> i32 { 0 }
}

pub trait SetCurrentProgram {

    /**
      | Called by the host to change the current
      | program.
      |
      */
    fn set_current_program(&mut self, index: i32) { }
}

pub trait GetProgramName {

    /**
      | Must return the name of a given program.
      |
      */
    fn get_program_name(&mut self, index: i32) -> String;
}

pub trait ChangeProgramName {

    /**
      | Called by the host to rename a program.
      |
      */
    fn change_program_name(
        &mut self, 
        index:    i32,
        new_name: &str
    );
}
