crate::ix!();

pub trait IsMidiChannelMetaEvent {

    /**
      | Returns true if this is a 'channel' meta-event.
      | 
      | -----------
      | @note
      | 
      | A channel meta-event specifies the
      | midi channel that should be used
      | 
      | for subsequent meta-events.
      | 
      | @see getMidiChannelMetaEventChannel
      |
      */
    fn is_midi_channel_meta_event(&self) -> bool;
}

pub trait GetMidiChannelMetaEventChannel {

    /**
      | Returns the channel number from a channel
      | meta-event.
      | 
      | -----------
      | @return
      | 
      | the channel, in the range 1 to 16.
      | 
      | @see isMidiChannelMetaEvent
      |
      */
    fn get_midi_channel_meta_event_channel(&self) -> i32;
}

pub trait MidiChannelMetaEvent {

    /**
      | Creates a midi channel meta-event.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      | 
      | @see isMidiChannelMetaEvent
      |
      */
    fn midi_channel_meta_event(&mut self, channel: i32) -> dyn MidiMessageInterface;
}
