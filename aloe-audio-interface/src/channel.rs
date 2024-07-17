crate::ix!();

pub trait GetChannel {

    /**
      | Returns the midi channel associated
      | with the message. @see isForChannel,
      | setChannel
      | 
      | -----------
      | @return
      | 
      | a value 1 to 16 if the message has a channel,
      | or 0 if it hasn't (e.g. if it's a sysex)
      |
      */
    fn get_channel(&self) -> i32;
}

pub trait IsForChannel {

    /**
      | Returns true if the message applies
      | to the given midi channel. @see getChannel,
      | setChannel
      | 
      | -----------
      | @param channelNumber
      | 
      | the channel number to look for, in the
      | range 1 to 16
      |
      */
    fn is_for_channel(&self, channel: i32) -> bool;
}

pub trait SetChannel {

    /**
      | Changes the message's midi channel.
      | This won't do anything for non-channel
      | messages like sysexes.
      | 
      | -----------
      | @param newChannelNumber
      | 
      | the channel number to change it to, in
      | the range 1 to 16
      |
      */
    fn set_channel(&mut self, channel: i32);
}
