crate::ix!();

pub trait AddChannel {

    /**
      | Adds a channel to the set.
      |
      */
    fn add_channel(&mut self, new_channel: AudioChannelType);
}

pub trait RemoveChannel {

    /**
      | Removes a channel from the set.
      |
      */
    fn remove_channel(&mut self, new_channel: AudioChannelType);
}
