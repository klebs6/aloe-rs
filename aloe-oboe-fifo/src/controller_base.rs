crate::ix!();

pub trait FifoControllerBaseInterface {
    fn get_read_counter(&self) -> u64;
    fn set_read_counter(&mut self, n: u64);
    fn increment_read_counter(&mut self, n: u64);
    fn get_write_counter(&self) -> u64;
    fn set_write_counter(&mut self, n: u64);
    fn increment_write_counter(&mut self, n: u64);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoControllerBase.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoControllerBase.cpp]

/**
  | Manage the read/write indices of a circular
  | buffer.
  | 
  | The caller is responsible for reading
  | and writing the actual data.
  | 
  | -----------
  | @note
  | 
  | the span of available frames may not
  | be contiguous. They may wrap around
  | from the end to the beginning of the buffer.
  | In that case the data must be read or written
  | in at least two blocks of frames.
  |
  */
pub struct FifoControllerBase {
    total_frames: u32,
}

impl FifoControllerBase {

    pub fn get_frame_capacity(&self) -> u32 {
        
        todo!();
        /*
            return mTotalFrames;
        */
    }
    
    /**
      | @param totalFrames
      | 
      | capacity of the circular buffer in frames.
      |
      */
    pub fn new(capacity_in_frames: u32) -> Self {
    
        todo!();
        /*
        : total_frames(capacityInFrames),

            // Avoid ridiculously large buffers and the arithmetic wraparound issues that can follow.
        assert(capacityInFrames <= (UINT32_MAX / 4));
        */
    }
    
    /**
      | The frames available to read will be
      | calculated from the read and write counters.
      | 
      | The result will be clipped to the capacity
      | of the buffer.
      | 
      | If the buffer has underflowed then this
      | will return zero.
      | 
      | -----------
      | @return
      | 
      | number of valid frames available to
      | read.
      |
      */
    pub fn get_full_frames_available(&self) -> u32 {
        
        todo!();
        /*
            uint64_t writeCounter =  getWriteCounter();
        uint64_t readCounter = getReadCounter();
        if (readCounter > writeCounter) {
            return 0;
        }
        uint64_t delta = writeCounter - readCounter;
        if (delta >= mTotalFrames) {
            return mTotalFrames;
        }
        // delta is now guaranteed to fit within the range of a uint32_t
        return static_cast<uint32_t>(delta);
        */
    }
    
    /**
      | The index in a circular buffer of the
      | next frame to read.
      |
      */
    pub fn get_read_index(&self) -> u32 {
        
        todo!();
        /*
            // % works with non-power of two sizes
        return static_cast<uint32_t>(getReadCounter() % mTotalFrames);
        */
    }
    
    /**
      | @param numFrames
      | 
      | number of frames to advance the read
      | index
      |
      */
    pub fn advance_read_index(&mut self, num_frames: u32)  {
        
        todo!();
        /*
            incrementReadCounter(numFrames);
        */
    }
    
    /**
      | @return
      | 
      | maximum number of frames that can be
      | written without exceeding the threshold.
      |
      */
    pub fn get_empty_frames_available(&self) -> u32 {
        
        todo!();
        /*
            return static_cast<uint32_t>(mTotalFrames - getFullFramesAvailable());
        */
    }
    
    /**
      | The index in a circular buffer of the
      | next frame to write.
      |
      */
    pub fn get_write_index(&self) -> u32 {
        
        todo!();
        /*
            // % works with non-power of two sizes
        return static_cast<uint32_t>(getWriteCounter() % mTotalFrames);
        */
    }
    
    /**
      | @param numFrames
      | 
      | number of frames to advance the write
      | index
      |
      */
    pub fn advance_write_index(&mut self, num_frames: u32)  {
        
        todo!();
        /*
            incrementWriteCounter(numFrames);
        */
    }
}
