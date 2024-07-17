crate::ix!();

pub trait AllControllersOff {

    /**
      | Creates an all-controllers-off message.
      | 
      | -----------
      | @param channel
      | 
      | the midi channel, in the range 1 to 16
      |
      */
    fn all_controllers_off(&mut self, channel: i32) -> dyn MidiMessageInterface;
}
