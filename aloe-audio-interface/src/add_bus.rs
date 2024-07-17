crate::ix!();

pub trait AudioProcessorAddBus {

    /**
      | Dynamically request an additional
      | bus.
      | 
      | Request an additional bus from the audio
      | processor. If the audio processor does
      | not support adding additional buses
      | then this method will return false.
      | 
      | Most audio processors will not allow
      | you to dynamically add/remove audio
      | buses and will return false.
      | 
      | This method will invoke the canApplyBusCountChange
      | callback to probe if a bus can be added
      | and, if yes, will use the supplied bus
      | properties of the canApplyBusCountChange
      | callback to create a new bus.
      | 
      | @see canApplyBusCountChange, removeBus
      |
      */
    fn add_bus(&mut self, is_input: bool) -> bool;
}

pub trait RemoveBus {

    /**
      | Dynamically remove the latest added
      | bus.
      | 
      | Request the removal of the last bus from
      | the audio processor. If the audio processor
      | does not support removing buses then
      | this method will return false.
      | 
      | Most audio processors will not allow
      | you to dynamically add/remove audio
      | buses and will return false.
      | 
      | The default implementation will return
      | false.
      | 
      | This method will invoke the canApplyBusCountChange
      | callback to probe if a bus can currently
      | be removed and, if yes, will go ahead
      | and remove it.
      | 
      | @see addBus, canRemoveBus
      |
      */
    fn remove_bus(&mut self, is_input: bool) -> bool;
}
