crate::ix!();

pub trait IsSostenutoPedalOn {

    /**
      | Returns true if this message is a 'sostenuto
      | pedal down' controller message.
      |
      */
    fn is_sostenuto_pedal_on(&self) -> bool;
}

pub trait IsSostenutoPedalOff {

    /**
      | Returns true if this message is a 'sostenuto
      | pedal up' controller message.
      |
      */
    fn is_sostenuto_pedal_off(&self) -> bool;
}
