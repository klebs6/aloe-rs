crate::ix!();

/**
  | Structure used for AudioProcessor
  | Callbacks
  |
  */
pub struct AudioProcessorBusProperties
{
    /**
      | The name of the bus
      |
      */
    bus_name:                String,

    /**
      | The default layout of the bus
      |
      */
    default_layout:          AudioChannelSet,

    /**
      | Is this bus activated by default?
      |
      */
    is_activated_by_default: bool,
}

pub trait AudioProcessorBusesPropertiesInterface: 
BusPropertiesAddBus 
+ WithInput 
+ WithOutput 
{ }

pub trait BusPropertiesAddBus {

    fn add_bus(
        &mut self, 
        is_input:                bool,
        name:                    &String,
        default_layout:          &AudioChannelSet,
        is_activated_by_default: bool
    );
}

pub trait WithInput {

    fn with_input(
        &self, 
        name:                    &String,
        default_layout:          &AudioChannelSet,
        is_activated_by_default: bool
    ) -> dyn AudioProcessorBusesPropertiesInterface;
}

pub trait WithOutput {

    fn with_output(
        &self, 
        name:                    &String,
        default_layout:          &AudioChannelSet,
        is_activated_by_default: bool
    ) -> dyn AudioProcessorBusesPropertiesInterface;
}

pub trait LayoutListToArray {

    fn layout_lists_to_array<const numLayouts: usize>(configuration: &[[i16; numLayouts]; 2]) -> Vec<AudioProcessorInOutChannelPair> where Self: Sized;

    fn layout_list_to_array(configuration: &[ [i16; 2] ]) -> Vec<AudioProcessorInOutChannelPair> where Self: Sized;
}

pub trait BusesPropertiesFromLayoutArray {

    fn buses_properties_from_layout_array(_0: &[AudioProcessorInOutChannelPair]) 
        -> Box<dyn AudioProcessorBusesPropertiesInterface> where Self: Sized;
}
