crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FixedBlockWriter.h]

/**
  | This can be used to convert a push data
  | flow from variable sized buffers to
  | fixed sized buffers.
  | 
  | An example would be an audio input callback.
  |
  */
pub struct FixedBlockWriter<'a> {
    base: FixedBlockAdapter<'a>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FixedBlockWriter.cpp]
impl<'a> FixedBlockWriter<'a> {

    pub fn new(fixed_block_processor: &mut dyn FixedBlockProcessor) -> Self {
    
        todo!();
        /*
        : fixed_block_adapter(fixedBlockProcessor),
        */
    }
    
    pub fn write_to_storage(&mut self, 
        buffer:    *mut u8,
        num_bytes: i32) -> i32 {
        
        todo!();
        /*
            int32_t bytesToStore = numBytes;
        int32_t roomAvailable = mSize - mPosition;
        if (bytesToStore > roomAvailable) {
            bytesToStore = roomAvailable;
        }
        memcpy(mStorage.get() + mPosition, buffer, bytesToStore);
        mPosition += bytesToStore;
        return bytesToStore;
        */
    }
    
    /**
      | Write from a variable sized block.
      | 
      | -----------
      | @note
      | 
      | if the fixed-sized blocks must be aligned,
      | then the variable-sized blocks must
      | have the same alignment. For example,
      | if the fixed-size blocks must be a multiple
      | of 8, then the variable-sized blocks
      | must also be a multiple of 8.
      | 
      | -----------
      | @param buffer
      | 
      | @param numBytes
      | 
      | -----------
      | @return
      | 
      | Number of bytes written or a negative
      | error code.
      |
      */
    pub fn write(&mut self, 
        buffer:    *mut u8,
        num_bytes: i32) -> i32 {
        
        todo!();
        /*
            int32_t bytesLeft = numBytes;

        // If we already have data in storage then add to it.
        if (mPosition > 0) {
            int32_t bytesWritten = writeToStorage(buffer, bytesLeft);
            buffer += bytesWritten;
            bytesLeft -= bytesWritten;
            // If storage full then flush it out
            if (mPosition == mSize) {
                bytesWritten = mFixedBlockProcessor.onProcessFixedBlock(mStorage.get(), mSize);
                if (bytesWritten < 0) return bytesWritten;
                mPosition = 0;
                if (bytesWritten < mSize) {
                    // Only some of the data was written! This should not happen.
                    return -1;
                }
            }
        }

        // Write through if enough for a complete block.
        while(bytesLeft > mSize) {
            int32_t bytesWritten = mFixedBlockProcessor.onProcessFixedBlock(buffer, mSize);
            if (bytesWritten < 0) return bytesWritten;
            buffer += bytesWritten;
            bytesLeft -= bytesWritten;
        }

        // Save any remaining partial blocks for next time.
        if (bytesLeft > 0) {
            int32_t bytesWritten = writeToStorage(buffer, bytesLeft);
            bytesLeft -= bytesWritten;
        }

        return numBytes - bytesLeft;
        */
    }
}
