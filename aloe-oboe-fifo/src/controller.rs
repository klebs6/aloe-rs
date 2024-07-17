crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoController.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoController.cpp]

/**
  | A FifoControllerBase with counters
  | contained in the class.
  |
  */
pub struct FifoController {
    base:          FifoControllerBase,
    read_counter:  Atomic<u64>,
    write_counter: Atomic<u64>,
}

impl FifoController {

    pub fn get_read_counter(&self) -> u64 {
        
        todo!();
        /*
            return mReadCounter.load(std::memory_order_acquire);
        */
    }
    
    pub fn set_read_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mReadCounter.store(n, std::memory_order_release);
        */
    }
    
    pub fn increment_read_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mReadCounter.fetch_add(n, std::memory_order_acq_rel);
        */
    }
    
    pub fn get_write_counter(&self) -> u64 {
        
        todo!();
        /*
            return mWriteCounter.load(std::memory_order_acquire);
        */
    }
    
    pub fn set_write_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mWriteCounter.store(n, std::memory_order_release);
        */
    }
    
    pub fn increment_write_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mWriteCounter.fetch_add(n, std::memory_order_acq_rel);
        */
    }
    
    pub fn new(num_frames: u32) -> Self {
    
        todo!();
        /*
        : fifo_controller_base(numFrames),

            setReadCounter(0);
        setWriteCounter(0);
        */
    }
}
