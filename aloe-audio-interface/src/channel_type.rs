crate::ix!();

pub trait GetChannelTypeName {

    /**
      | Returns the name of a given channel type.
      | For example, this method may return
      | "Surround Left".
      |
      */
    fn get_channel_type_name(&mut self, ty: AudioChannelType) -> String;
}

pub trait GetAbbreviatedChannelTypeName {

    /**
      | Returns the abbreviated name of a channel
      | type. For example, this method may return
      | "Ls".
      |
      */
    fn get_abbreviated_channel_type_name(&mut self, ty: AudioChannelType) -> String;
}

pub trait GetChannelTypeFromAbbreviation {

    /**
      | Returns the channel type from an abbreviated
      | name.
      |
      */
    fn get_channel_type_from_abbreviation(&mut self, abbr: &String) -> AudioChannelType;
}
