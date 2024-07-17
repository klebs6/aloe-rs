crate::ix!();

pub trait IsSoftPedalOn {

    /**
      | Returns true if this message is a 'soft
      | pedal down' controller message.
      |
      */
    fn is_soft_pedal_on(&self) -> bool;
}

pub trait IsSoftPedalOff {

    /**
      | Returns true if this message is a 'soft
      | pedal up' controller message.
      |
      */
    fn is_soft_pedal_off(&self) -> bool;
}
