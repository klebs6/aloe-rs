crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_ApplicationCommandID.h]

/**
  | A set of general-purpose application
  | command IDs.
  | 
  | Because these commands are likely to
  | be used in most apps, they're defined
  | here to help different apps to use the
  | same numeric values for them.
  | 
  | Of course you don't have to use these,
  | but some of them are used internally
  | by Aloe - e.g. the quit ID is recognised
  | as a command by the ALOEApplication
  | class.
  | 
  | @see ApplicationCommandInfo, ApplicationCommandManager,
  | ApplicationCommandTarget, KeyPressMappingSet
  |
  */
pub enum StandardApplicationCommandsIds {

    /**
      | This command ID should be used to send
      | a "Quit the App" command.
      | 
      | This command is recognised by the ALOEApplication
      | class, so if it is invoked and no other
      | ApplicationCommandTarget handles
      | the event first, the ALOEApplication
      | object will catch it and call ALOEApplicationBase::systemRequestedQuit().
      |
      */
    Quit,

    /**
      | The command ID that should be used to
      | send a "Delete" command.
      |
      */
    Del,

    /**
      | The command ID that should be used to
      | send a "Cut" command.
      |
      */
    Cut,

    /**
      | The command ID that should be used to
      | send a "Copy to clipboard" command.
      |
      */
    Copy,

    /**
      | The command ID that should be used to
      | send a "Paste from clipboard" command.
      |
      */
    Paste,

    /**
      | The command ID that should be used to
      | send a "Select all" command.
      |
      */
    SelectAll,

    /**
      | The command ID that should be used to
      | send a "Deselect all" command.
      |
      */
    DeselectAll,

    /**
      | The command ID that should be used to
      | send a "undo" command.
      |
      */
    Undo,

    /**
      | The command ID that should be used to
      | send a "redo" command.
      |
      */
    Redo,
}

impl StandardApplicationCommandsIds {

    pub fn value(&self) -> usize {

        use StandardApplicationCommandsIds::*;

        match self {
            Quit        => 0x1001,
            Del         => 0x1002,
            Cut         => 0x1003,
            Copy        => 0x1004,
            Paste       => 0x1005,
            SelectAll   => 0x1006,
            DeselectAll => 0x1007,
            Undo        => 0x1008,
            Redo        => 0x1009,
        }
    }
}
