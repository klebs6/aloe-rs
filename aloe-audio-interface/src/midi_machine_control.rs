crate::ix!();

pub trait IsMidiMachineControlMessage {

    /**
      | Checks whether this is an MMC message.
      | If it is, you can use the getMidiMachineControlCommand()
      | to find out its type.
      |
      */
    fn is_midi_machine_control_message(&self) -> bool;
}

pub trait GetMidiMachineControlCommand {

    /**
      | For an MMC message, this returns its
      | type. Make sure it's actually an MMC
      | message with isMidiMachineControlMessage()
      | before calling this method.
      |
      */
    fn get_midi_machine_control_command(&self) -> dyn MidiMachineControlCommand;
}

pub trait MidiMachineControlCommand {

    /**
      | Creates an MMC message.
      |
      */
    fn midi_machine_control_command(&mut self, command: dyn MidiMachineControlCommand) -> dyn MidiMessageInterface;
}

pub trait IsMidiMachineControlGoto {

    /**
      | Checks whether this is an MMC "goto"
      | message. If it is, the parameters passed-in
      | are set to the time that the message contains.
      | @see midiMachineControlGoto
      |
      */
    fn is_midi_machine_control_goto(
        &self, 
        hours:   &mut i32,
        minutes: &mut i32,
        seconds: &mut i32,
        frames:  &mut i32

    ) -> bool;
}

pub trait MidiMachineControlGoto {

    /**
      | Creates an MMC "goto" message. This
      | messages tells the device to go to a specific
      | frame. @see isMidiMachineControlGoto
      |
      */
    fn midi_machine_control_goto(
        &mut self, 
        hours:   i32,
        minutes: i32,
        seconds: i32,
        frames:  i32

    ) -> dyn MidiMessageInterface;
}
