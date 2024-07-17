crate::ix!();

pub trait DiscreteChannels {

    /**
      | Creates a set of untyped discrete channels.
      |
      */
    fn discrete_channels(&mut self, num_channels: i32) -> dyn AudioChannelSetInterface;
}
