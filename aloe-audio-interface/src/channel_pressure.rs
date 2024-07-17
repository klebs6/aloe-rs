crate::ix!();

pub trait IsChannelPressure {

    /**
      | Returns true if the message is a channel-pressure
      | change event.
      | 
      | -----------
      | @note
      | 
      | This is like aftertouch, but common
      | to the whole channel rather than a specific
      | 
      | note. Use getChannelPressureValue()
      | to find out the pressure, and getChannel()
      | 
      | to find out the channel.
      | 
      | @see channelPressureChange
      |
      */
    fn is_channel_pressure(&self) -> bool;
}

pub trait GetChannelPressureValue {

    /**
      | Returns the pressure from a channel
      | pressure change message.
      | 
      | -----------
      | @return
      | 
      | the pressure, in the range 0 to 127
      | 
      | @see isChannelPressure, channelPressureChange
      |
      */
    fn get_channel_pressure_value(&self) -> i32;
}

pub trait ChannelPressureChange {

    /**
      | Creates a channel-pressure change
      | event.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel: 1 to 16
      | ----------
      | @param pressure
      | 
      | the pressure, 0 to 127
      | 
      | @see isChannelPressure
      |
      */
    fn channel_pressure_change(
        &mut self, 
        channel:  i32,
        pressure: i32
    ) -> dyn MidiMessageInterface;
}
