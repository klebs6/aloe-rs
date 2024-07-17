crate::ix!();

pub trait NumChannelsChanged {

    /**
      | This method is called when the total
      | number of input or output channels is
      | changed.
      |
      */
    fn num_channels_changed(&mut self);
}

pub trait NumBusesChanged {

    /**
      | This method is called when the number
      | of buses is changed.
      |
      */
    fn num_buses_changed(&mut self);
}

pub trait ProcessorLayoutsChanged {

    /**
      | This method is called when the layout
      | of the audio processor changes.
      |
      */
    fn processor_layouts_changed(&mut self);
}
