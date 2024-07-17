crate::ix!();

pub trait FillInPluginDescription {

    /**
      | Fills-in the appropriate parts of this
      | plugin description object.
      |
      */
    fn fill_in_plugin_description(&self, _0: &mut PluginDescription);
}

pub trait FromAbbreviatedString {

    /**
      | Returns an dyn AudioChannelSetInterface from a string
      | returned by getSpeakerArrangementAsString
      | @see getSpeakerArrangementAsString
      */
    fn from_abbreviated_string(&mut self, str_: &String) -> dyn AudioChannelSetInterface;
}

pub trait GetDescription {

    fn get_description(&self) -> String;
}

pub trait GetPluginDescription {

    /**
      | Returns a PluginDescription for this
      | plugin.
      | 
      | This is just a convenience method to
      | avoid calling fillInPluginDescription.
      |
      */
    fn get_plugin_description(&self) -> PluginDescription;
}
