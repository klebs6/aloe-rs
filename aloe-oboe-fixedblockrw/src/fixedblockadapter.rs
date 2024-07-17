crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FixedBlockAdapter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/FixedBlockAdapter.cpp]

/**
  | Interface for a class that needs fixed-size
  | blocks.
  |
  */
pub trait FixedBlockProcessor {

    /**
      | @param buffer
      | 
      | Pointer to first byte of data.
      | ----------
      | @param numBytes
      | 
      | This will be a fixed size specified in
      | FixedBlockAdapter::open().
      | 
      | -----------
      | @return
      | 
      | Number of bytes processed or a negative
      | error code.
      |
      */
    fn on_process_fixed_block(&mut self, 
        buffer:    *mut u8,
        num_bytes: i32) -> i32;
}

/**
  | Base class for a variable-to-fixed-size
  | block adapter.
  |
  */
pub struct FixedBlockAdapter<'a> {

    fixed_block_processor: &'a mut dyn FixedBlockProcessor,

    /**
      | Store data here while assembling buffers.
      |
      */
    storage:               Box<&'a [u8]>,

    /**
      | Size in bytes of the fixed size buffer.
      |
      */
    size:                  i32, // default = 0

    /**
      | Offset of the last byte read or written.
      |
      */
    position:              i32, // default = 0
}

impl<'a> FixedBlockAdapter<'a> {
    
    pub fn new(fixed_block_processor: &mut dyn FixedBlockProcessor) -> Self {
    
        todo!();
        /*
        : fixed_block_processor(fixedBlockProcessor),
        */
    }

    /**
      | Allocate internal resources needed
      | for buffering data.
      |
      */
    pub fn open(&mut self, bytes_per_fixed_block: i32) -> i32 {
        
        todo!();
        /*
            mSize = bytesPerFixedBlock;
        mStorage = std::make_unique<uint8_t[]>(bytesPerFixedBlock);
        mPosition = 0;
        return 0;
        */
    }
    
    /**
      | Free internal resources.
      |
      */
    pub fn close(&mut self) -> i32 {
        
        todo!();
        /*
            mStorage.reset(nullptr);
        mSize = 0;
        mPosition = 0;
        return 0;
        */
    }
}
