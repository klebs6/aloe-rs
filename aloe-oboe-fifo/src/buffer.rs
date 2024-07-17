crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoBuffer.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/fifo/FifoBuffer.cpp]

pub struct FifoBuffer {
    bytes_per_frame:       u32,
    storage:               *mut u8,

    /**
      | did this object allocate the storage?
      |
      */
    storage_owned:         bool,

    fifo:                  Box<FifoControllerBase>,
    frames_read_count:     u64,
    frames_underrun_count: u64,
}

impl Drop for FifoBuffer {

    fn drop(&mut self) {
        todo!();
        /* 
        if (mStorageOwned) {
            delete[] mStorage;
        }
         */
    }
}

impl FifoBuffer {

    pub fn get_full_frames_available(&mut self) -> u32 {
        
        todo!();
        /*
            return mFifo->getFullFramesAvailable();
        */
    }
    
    pub fn get_bytes_per_frame(&self) -> u32 {
        
        todo!();
        /*
            return mBytesPerFrame;
        */
    }
    
    pub fn get_read_counter(&self) -> u64 {
        
        todo!();
        /*
            return mFifo->getReadCounter();
        */
    }
    
    pub fn set_read_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mFifo->setReadCounter(n);
        */
    }
    
    pub fn get_write_counter(&mut self) -> u64 {
        
        todo!();
        /*
            return mFifo->getWriteCounter();
        */
    }
    
    pub fn set_write_counter(&mut self, n: u64)  {
        
        todo!();
        /*
            mFifo->setWriteCounter(n);
        */
    }
    
    pub fn new(
        bytes_per_frame:    u32,
        capacity_in_frames: u32) -> Self {
    
        todo!();
        /*

            : mBytesPerFrame(bytesPerFrame)
            , mStorage(nullptr)
            , mFramesReadCount(0)
            , mFramesUnderrunCount(0)

        mFifo = std::make_unique<FifoController>(capacityInFrames);
        // allocate buffer
        int32_t bytesPerBuffer = bytesPerFrame * capacityInFrames;
        mStorage = new uint8_t[bytesPerBuffer];
        mStorageOwned = true;
        */
    }
    
    pub fn new_with_addresses(
        bytes_per_frame:       u32,
        capacity_in_frames:    u32,
        read_counter_address:  *mut Atomic<u64>,
        write_counter_address: *mut Atomic<u64>,
        data_storage_address:  *mut u8) -> Self {
    
        todo!();
        /*

            : mBytesPerFrame(bytesPerFrame)
            , mStorage(dataStorageAddress)
            , mFramesReadCount(0)
            , mFramesUnderrunCount(0)

        mFifo = std::make_unique<FifoControllerIndirect>(capacityInFrames,
                                           readCounterAddress,
                                           writeCounterAddress);
        mStorage = dataStorageAddress;
        mStorageOwned = false;
        */
    }
    
    pub fn convert_frames_to_bytes(&mut self, frames: i32) -> i32 {
        
        todo!();
        /*
            return frames * mBytesPerFrame;
        */
    }
    
    /**
      | Read framesToRead or, if not enough,
      | then read as many as are available.
      | 
      | -----------
      | @param destination
      | 
      | @param framesToRead number of frames
      | requested
      | 
      | -----------
      | @return
      | 
      | number of frames actually read
      |
      */
    pub fn read(&mut self, 
        buffer:     *mut c_void,
        num_frames: i32) -> i32 {
        
        todo!();
        /*
            if (numFrames <= 0) {
            return 0;
        }
        // safe because numFrames is guaranteed positive
        uint32_t framesToRead = static_cast<uint32_t>(numFrames);
        uint32_t framesAvailable = mFifo->getFullFramesAvailable();
        framesToRead = std::min(framesToRead, framesAvailable);

        uint32_t readIndex = mFifo->getReadIndex(); // ranges 0 to capacity
        uint8_t *destination = reinterpret_cast<uint8_t *>(buffer);
        uint8_t *source = &mStorage[convertFramesToBytes(readIndex)];
        if ((readIndex + framesToRead) > mFifo->getFrameCapacity()) {
            // read in two parts, first part here is at the end of the mStorage buffer
            int32_t frames1 = static_cast<int32_t>(mFifo->getFrameCapacity() - readIndex);
            int32_t numBytes = convertFramesToBytes(frames1);
            if (numBytes < 0) {
                return static_cast<int32_t>(OboeResult::ErrorOutOfRange);
            }
            memcpy(destination, source, static_cast<size_t>(numBytes));
            destination += numBytes;
            // read second part, which is at the beginning of mStorage
            source = &mStorage[0];
            int32_t frames2 = static_cast<uint32_t>(framesToRead - frames1);
            numBytes = convertFramesToBytes(frames2);
            if (numBytes < 0) {
                return static_cast<int32_t>(OboeResult::ErrorOutOfRange);
            }
            memcpy(destination, source, static_cast<size_t>(numBytes));
        } else {
            // just read in one shot
            int32_t numBytes = convertFramesToBytes(framesToRead);
            if (numBytes < 0) {
                return static_cast<int32_t>(OboeResult::ErrorOutOfRange);
            }
            memcpy(destination, source, static_cast<size_t>(numBytes));
        }
        mFifo->advanceReadIndex(framesToRead);

        return framesToRead;
        */
    }
    
    pub fn write(&mut self, 
        buffer:     *const c_void,
        num_frames: i32) -> i32 {
        
        todo!();
        /*
            if (numFrames <= 0) {
            return 0;
        }
        // Guaranteed positive.
        uint32_t framesToWrite = static_cast<uint32_t>(numFrames);
        uint32_t framesAvailable = mFifo->getEmptyFramesAvailable();
        framesToWrite = std::min(framesToWrite, framesAvailable);

        uint32_t writeIndex = mFifo->getWriteIndex();
        int byteIndex = convertFramesToBytes(writeIndex);
        const uint8_t *source = reinterpret_cast<const uint8_t *>(buffer);
        uint8_t *destination = &mStorage[byteIndex];
        if ((writeIndex + framesToWrite) > mFifo->getFrameCapacity()) {
            // write in two parts, first part here
            int32_t frames1 = static_cast<uint32_t>(mFifo->getFrameCapacity() - writeIndex);
            int32_t numBytes = convertFramesToBytes(frames1);
            if (numBytes < 0) {
                return static_cast<int32_t>(OboeResult::ErrorOutOfRange);
            }
            memcpy(destination, source, static_cast<size_t>(numBytes));
            // read second part
            source += convertFramesToBytes(frames1);
            destination = &mStorage[0];
            int frames2 = static_cast<uint32_t>(framesToWrite - frames1);
            numBytes = convertFramesToBytes(frames2);
            if (numBytes < 0) {
                return static_cast<int32_t>(OboeResult::ErrorOutOfRange);
            }
            memcpy(destination, source, static_cast<size_t>(numBytes));
        } else {
            // just write in one shot
            int32_t numBytes = convertFramesToBytes(framesToWrite);
            if (numBytes < 0) {
                return static_cast<int32_t>(OboeResult::ErrorOutOfRange);
            }
            memcpy(destination, source, static_cast<size_t>(numBytes));
        }
        mFifo->advanceWriteIndex(framesToWrite);

        return framesToWrite;
        */
    }
    
    /**
      | Calls read(). If all of the frames cannot
      | be read then the remainder of the buffer
      | is set to zero.
      | 
      | -----------
      | @param destination
      | 
      | @param framesToRead number of frames
      | requested
      | 
      | -----------
      | @return
      | 
      | number of frames actually read
      |
      */
    pub fn read_now(&mut self, 
        buffer:     *mut c_void,
        num_frames: i32) -> i32 {
        
        todo!();
        /*
            int32_t framesRead = read(buffer, numFrames);
        if (framesRead < 0) {
            return framesRead;
        }
        int32_t framesLeft = numFrames - framesRead;
        mFramesReadCount += framesRead;
        mFramesUnderrunCount += framesLeft;
        // Zero out any samples we could not set.
        if (framesLeft > 0) {
            uint8_t *destination = reinterpret_cast<uint8_t *>(buffer);
            destination += convertFramesToBytes(framesRead); // point to first byte not set
            int32_t bytesToZero = convertFramesToBytes(framesLeft);
            memset(destination, 0, static_cast<size_t>(bytesToZero));
        }

        return framesRead;
        */
    }
    
    pub fn get_buffer_capacity_in_frames(&self) -> u32 {
        
        todo!();
        /*
            return mFifo->getFrameCapacity();
        */
    }
}
