crate::ix!();

pub trait AudioProcessorBusInterface 
: BusIsInput 
+ GetBusIndex 
+ BusIsMain 

/*
   | The bus's name.
   |
   */
+ GetName 
+ GetDefaultLayout 
+ GetCurrentLayout 
+ GetLastEnabledLayout 
+ SetCurrentLayout 
+ SetCurrentLayoutWithoutEnabling 
+ GetNumberOfChannels 
+ SetNumberOfChannels 
+ IsLayoutSupported 
+ IsNumberOfChannelsSupported 
+ SupportedLayoutWithChannels 
+ GetMaxSupportedChannels 
+ GetBusesLayoutForLayoutChangeOfBus 
+ IsEnabled 
+ AudioProcessorBusEnable 
+ IsEnabledByDefault 
+ GetChannelIndexInProcessBlockBuffer 
//+ GetBusBuffer 
+ GetBusDirectionAndIndex 
+ UpdateChannelCount 
{ }

pub trait GetChannelIndexInProcessBlockBuffer {

    /**
      | Returns the position of a bus's channels
      | within the processBlock buffer.
      | 
      | This can be called in processBlock to
      | figure out which channel of the master
      | AudioBuffer maps onto a specific bus's
      | channel.
      |
      */
    fn get_channel_index_in_process_block_buffer(&self, channel_index: i32) -> i32;
}

pub trait GetBusBuffer<FloatType: num::Float> {

    /**
      | Returns an AudioBuffer containing
      | a set of channel pointers for a specific
      | bus.
      | 
      | This can be called in processBlock to
      | get a buffer containing a sub-group
      | of the master
      | 
      | AudioBuffer which contains all the
      | plugin channels.
      |
      */
    fn get_bus_buffer(
        &self, 
        process_block_buffer: &mut AudioBuffer<FloatType>
    ) -> AudioBuffer<FloatType>;
}
