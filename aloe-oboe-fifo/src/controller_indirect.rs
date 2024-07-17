crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoControllerIndirect.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoControllerIndirect.cpp]

/**
  | A FifoControllerBase with counters
  | external to the class.
  |
  */
pub struct FifoControllerIndirect {
    base:                  FifoControllerBase,
    read_counter_address:  *mut Atomic<u64>,
    write_counter_address: *mut Atomic<u64>,
}

impl FifoControllerIndirect {
    
    pub fn get_read_counter(&self) -> u64 {
        
        todo!();
        /*
            return mReadCounterAddress->load(std::memory_order_acquire);
        */
    }
    
    pub fn set_read_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mReadCounterAddress->store(n, std::memory_order_release);
        */
    }
    
    pub fn increment_read_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mReadCounterAddress->fetch_add(n, std::memory_order_acq_rel);
        */
    }
    
    pub fn get_write_counter(&self) -> u64 {
        
        todo!();
        /*
            return mWriteCounterAddress->load(std::memory_order_acquire);
        */
    }
    
    pub fn set_write_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mWriteCounterAddress->store(n, std::memory_order_release);
        */
    }
    
    pub fn increment_write_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mWriteCounterAddress->fetch_add(n, std::memory_order_acq_rel);
        */
    }
    
    pub fn new(
        num_frames:            u32,
        read_counter_address:  *mut Atomic<u64>,
        write_counter_address: *mut Atomic<u64>) -> Self {
    
        todo!();
        /*
        : fifo_controller_base(numFrames),
        : read_counter_address(readCounterAddress),
        : write_counter_address(writeCounterAddress),

        
        */
    }
}
