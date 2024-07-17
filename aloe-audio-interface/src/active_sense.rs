crate::ix!();

pub trait IsActiveSense {

    /**
      | Returns true if this is an active-sense
      | message.
      |
      */
    fn is_active_sense(&self) -> bool;
}
