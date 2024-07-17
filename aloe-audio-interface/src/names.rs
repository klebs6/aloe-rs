crate::ix!();

pub trait GetID {

    /**
      | Returns the group's ID.
      |
      */
    fn getid(&self) -> String;
}

pub trait GetAlternateDisplayNames {

    /**
      | Returns a list of alternative names
      | to use for this processor.
      | 
      | Some hosts truncate the name of your
      | AudioProcessor when there isn't enough
      | space in the GUI to show the full name.
      | Overriding this method, allows the
      | host to choose an alternative name (such
      | as an abbreviation) to better fit the
      | available space.
      |
      */
    fn get_alternate_display_names(&self) -> Vec<String>;
}

pub trait SetName {

    /**
      | Changes the name of the group. If you
      | do this after the group has been added
      | to an AudioProcessor, call updateHostDisplay()
      | to inform the host of the change. Not
      | all hosts support dynamic group name
      | changes.
      |
      */
    fn set_name(&mut self, new_name: String);
}

pub trait GetControllerName {

    /**
      | Returns the name of a controller type
      | number, or nullptr if unknown for this
      | controller number. @see getControllerNumber
      |
      */
    fn get_controller_name(&mut self, n: i32) -> *const u8;
}

pub trait GetFormatName {

    fn get_format_name(&self) -> &String;
}

pub trait GetOutputChannelNames {

    /** 
      | Returns the names of all the available
      | output channels on this device.
      | To find out which of these are currently
      | in use, call getActiveOutputChannels().
      */
    fn get_output_channel_names(&mut self) -> Vec<String>;
}

pub trait GetInputChannelNames {

    /** 
      | Returns the names of all the available
      | input channels on this device.  To find
      | out which of these are currently in use,
      | call getActiveInputChannels().
      */
    fn get_input_channel_names(&mut self) -> Vec<String>;
}
