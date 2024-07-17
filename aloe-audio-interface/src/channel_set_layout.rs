crate::ix!();

pub trait IsDiscreteLayout {

    /**
      | Returns if this is a channel layout made-up
      | of discrete channels.
      |
      */
    fn is_discrete_layout(&self) -> bool;
}
