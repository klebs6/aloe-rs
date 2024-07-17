crate::ix!();

pub trait IsSustainPedalOn {

    /**
      | Returns true if this message is a 'sustain
      | pedal down' controller message.
      |
      */
    fn is_sustain_pedal_on(&self) -> bool;
}

pub trait IsSustainPedalOff {

    /**
      | Returns true if this message is a 'sustain
      | pedal up' controller message.
      |
      */
    fn is_sustain_pedal_off(&self) -> bool;
}
