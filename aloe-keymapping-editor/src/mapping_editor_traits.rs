crate::ix!();

pub trait KeyMappingEditorComponentInterface: 
ShouldCommandBeIncluded
+ IsCommandReadOnly
+ GetDescriptionForKeyPress {}

pub trait ShouldCommandBeIncluded {

    /**
      | Can be overridden if some commands need
      | to be excluded from the list.
      | 
      | By default this will use the KeyPressMappingSet's
      | shouldCommandBeVisibleInEditor()
      | method to decide what to return, but
      | you can override it to handle special
      | cases.
      |
      */
    fn should_command_be_included(&mut self, commandid: CommandID) -> bool;
}

pub trait IsCommandReadOnly {

    /**
      | Can be overridden to indicate that some
      | commands are shown as read-only.
      | 
      | By default this will use the KeyPressMappingSet's
      | shouldCommandBeReadOnlyInEditor()
      | method to decide what to return, but
      | you can override it to handle special
      | cases.
      |
      */
    fn is_command_read_only(&mut self, commandid: CommandID) -> bool;
}

pub trait GetDescriptionForKeyPress {

    /**
      | This can be overridden to let you change
      | the format of the string used to describe
      | a keypress.
      | 
      | This is handy if you're using non-standard
      | KeyPress objects, e.g. for custom keys
      | that are triggered by something else
      | externally. If you override the method,
      | be sure to let the base class's method
      | handle keys you're not interested in.
      |
      */
    fn get_description_for_key_press(&mut self, key: &KeyPress) -> String;
}
