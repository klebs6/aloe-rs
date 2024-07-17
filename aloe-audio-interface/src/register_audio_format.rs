crate::ix!();

pub trait RegisterFormat {

    /**
      | Adds a format to the manager's list of
      | available file types.
      | 
      | The object passed-in will be deleted
      | by this object, so don't keep a pointer
      | to it!
      | 
      | If makeThisTheDefaultFormat is true,
      | then the getDefaultFormat() method
      | will return this one when called.
      |
      */
    fn register_format(
        &mut self, 
        new_format:                   *mut dyn AudioFormatInterface,
        make_this_the_default_format: bool
    );
}

pub trait RegisterBasicFormats {

    /**
      | Handy method to make it easy to register
      | the formats that come with Aloe.
      | 
      | This will add WAV and AIFF to the list,
      | along with any other formats enabled
      | in either the Proaloer or your application's
      | preprocessor definitions.
      |
      */
    fn register_basic_formats(&mut self);
}

pub trait ClearFormats {

    /**
      | Clears the list of known formats.
      |
      */
    fn clear_formats(&mut self);
}
