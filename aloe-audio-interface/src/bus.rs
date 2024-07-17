crate::ix!();

pub trait GetBus {

    /**
      | Returns the audio bus with a given index
      | and direction.
      | 
      | If busIndex is invalid then this method
      | will return a nullptr.
      |
      */
    fn get_bus_mut(
        &mut self, 
        is_input:  bool,
        bus_index: i32) -> *mut dyn AudioProcessorBusInterface;

    /**
      | Returns the audio bus with a given index
      | and direction.
      | 
      | If busIndex is invalid then this method
      | will return a nullptr.
      |
      */
    fn get_bus(
        &self, 
        is_input:  bool,
        bus_index: i32
    ) -> *const dyn AudioProcessorBusInterface;
}

pub trait GetBusChannelIndexInProcessBlockBuffer {

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
    fn get_channel_index_in_process_block_buffer(
        &self, 
        is_input:      bool,
        bus_index:     i32,
        channel_index: i32

    ) -> i32;
}

pub trait GetOffsetInBusBufferForAbsoluteChannelIndex {

    /**
      | Returns the offset in a bus's buffer
      | from an absolute channel index.
      | 
      | This method returns the offset in a bus's
      | buffer given an absolute channel index.
      | 
      | It also provides the bus index. For example,
      | this method would return one for a processor
      | with two stereo buses when given the
      | absolute channel index.
      |
      */
    fn get_offset_in_bus_buffer_for_absolute_channel_index(
        &self, 
        is_input:               bool,
        absolute_channel_index: i32,
        bus_index:              &mut i32) -> i32;
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
        process_block_buffer: &mut AudioBuffer<FloatType>,
        is_input:             bool,
        bus_index:            i32

    ) -> AudioBuffer<FloatType>;
}

pub trait CheckBusesLayoutSupported {

    /**
      | Returns true if the Audio processor
      | is likely to support a given layout.
      | 
      | This can be called regardless if the
      | processor is currently running.
      |
      */
    fn check_buses_layout_supported(&self, _0: &dyn AudioProcessorBusesLayoutInterface) -> bool;
}
