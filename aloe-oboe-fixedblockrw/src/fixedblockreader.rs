crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FixedBlockReader.h]

/**
  | Read from a fixed-size block to a variable
  | sized block.
  | 
  | This can be used to convert a pull data
  | flow from fixed sized buffers to variable
  | sized buffers.
  | 
  | An example would be an audio output callback
  | that reads from the app.
  |
  */
pub struct FixedBlockReader<'a> {

    base: FixedBlockAdapter<'a>,

    /**
      | Number of valid bytes in mStorage.
      |
      */
    valid: i32, // default = 0
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FixedBlockReader.cpp]
impl<'a> FixedBlockReader<'a> {

    pub fn new(fixed_block_processor: &mut dyn FixedBlockProcessor) -> Self {
    
        todo!();
        /*
        : fixed_block_adapter(fixedBlockProcessor),

            mPosition = mSize;
        */
    }
    
    pub fn open(&mut self, bytes_per_fixed_block: i32) -> i32 {
        
        todo!();
        /*
            int32_t result = FixedBlockAdapter::open(bytesPerFixedBlock);
        mPosition = 0;
        mValid = 0;
        return result;
        */
    }
    
    pub fn read_from_storage(&mut self, 
        buffer:    *mut u8,
        num_bytes: i32) -> i32 {
        
        todo!();
        /*
            int32_t bytesToRead = numBytes;
        int32_t dataAvailable = mValid - mPosition;
        if (bytesToRead > dataAvailable) {
            bytesToRead = dataAvailable;
        }
        memcpy(buffer, mStorage.get() + mPosition, bytesToRead);
        mPosition += bytesToRead;
        return bytesToRead;
        */
    }
    
    /**
      | Read into a variable sized block.
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
      | Number of bytes read or a negative error
      | code.
      |
      */
    pub fn read(&mut self, 
        buffer:    *mut u8,
        num_bytes: i32) -> i32 {
        
        todo!();
        /*
            int32_t bytesRead;
        int32_t bytesLeft = numBytes;
        while(bytesLeft > 0) {
            if (mPosition < mValid) {
                // Use up bytes currently in storage.
                bytesRead = readFromStorage(buffer, bytesLeft);
                buffer += bytesRead;
                bytesLeft -= bytesRead;
            } else if (bytesLeft >= mSize) {
                // Nothing in storage. Read through if enough for a complete block.
                bytesRead = mFixedBlockProcessor.onProcessFixedBlock(buffer, mSize);
                if (bytesRead < 0) return bytesRead;
                buffer += bytesRead;
                bytesLeft -= bytesRead;
            } else {
                // Just need a partial block so we have to reload storage.
                bytesRead = mFixedBlockProcessor.onProcessFixedBlock(mStorage.get(), mSize);
                if (bytesRead < 0) return bytesRead;
                mPosition = 0;
                mValid = bytesRead;
                if (bytesRead == 0) break;
            }
        }
        return numBytes - bytesLeft;
        */
    }
}
