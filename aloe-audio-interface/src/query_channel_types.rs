crate::ix!();

pub trait GetChannelTypes {

    /**
      | Returns an array of all the types in this
      | channel set.
      |
      */
    fn get_channel_types(&self) -> Vec<AudioChannelType>;
}

pub trait GetSize {

    /**
      | Returns the number of channels in the
      | set.
      |
      */
    fn size(&self) -> i32;
}

pub trait GetTypeOfChannel {

    /**
      | Returns the type of one of the channels
      | in the set, by index.
      |
      */
    fn get_type_of_channel(&self, index: i32) -> AudioChannelType;
}

pub trait GetChannelIndexForType {

    /**
      | Returns the index for a particular channel-type.
      | Will return -1 if the this set does not contain
      | a channel of this type.
      */
    fn get_channel_index_for_type(&self, ty: AudioChannelType) -> i32;
}
