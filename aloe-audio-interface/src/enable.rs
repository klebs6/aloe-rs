crate::ix!();

pub trait IsEnabled {

    /**
      | Returns true if the current bus is enabled.
      |
      */
    fn is_enabled(&self) -> bool;
}

pub trait AudioProcessorBusEnable {

    /**
      | Enable or disable this bus. This will
      | return false if the AudioProcessor
      | does not support disabling this bus.
      |
      */
    fn enable(&mut self, should_enable: bool) -> bool;
}

pub trait IsEnabledByDefault {

    /**
      | Returns if this bus is enabled by default.
      |
      */
    fn is_enabled_by_default(&self) -> bool;
}

pub trait IsDisabled {

    /**
      | Returns true if there are no channels
      | in the set.
      |
      */
    fn is_disabled(&self) -> bool;
}

pub trait EnableAllBuses {

    /**
      | Enables all buses
      |
      */
    fn enable_all_buses(&mut self) -> bool;
}

pub trait DisableNonMainBuses {

    /**
      | Disables all non-main buses (aux and
      | sidechains).
      |
      */
    fn disable_non_main_buses(&mut self) -> bool;
}
