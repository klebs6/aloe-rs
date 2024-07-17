crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_MemoryInputStream.h]

/**
  | Allows a block of data to be accessed
  | as a stream.
  | 
  | This can either be used to refer to a shared
  | block of memory, or can make its own internal
  | copy of the data when the MemoryInputStream
  | is created.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
#[no_copy]
pub struct MemoryInputStream {
    data:          *const c_void,
    data_size:     usize,
    position:      usize, // default = 0
    internal_copy: MemoryBlock,
}

impl Read for MemoryInputStream {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        todo!();
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_MemoryInputStream.cpp]
impl MemoryInputStream {

    /**
      | Returns a pointer to the source data
      | block from which this stream is reading.
      |
      */
    pub fn get_data(&self)  {
        
        todo!();
        /*
            return data;
        */
    }

    /**
      | Returns the number of bytes of source
      | data in the block from which this stream
      | is reading.
      |
      */
    pub fn get_data_size(&self) -> usize {
        
        todo!();
        /*
            return dataSize;
        */
    }
    
    /**
      | Creates a MemoryInputStream.
      | 
      | -----------
      | @param sourceData
      | 
      | the block of data to use as the stream's
      | source
      | ----------
      | @param sourceDataSize
      | 
      | the number of bytes in the source data
      | block
      | ----------
      | @param keepInternalCopyOfData
      | 
      | if false, the stream will just keep a
      | pointer to the source data, so this data
      | shouldn't be changed for the lifetime
      | of the stream; if this parameter is true,
      | the stream will make its own copy of the
      | data and use that.
      |
      */
    pub fn new_from_source_data_and_size(
        source_data:      *const c_void,
        source_data_size: usize,
        keep_copy:        bool) -> Self {
    
        todo!();
        /*


            : data (sourceData),
          dataSize (sourceDataSize)

        if (keepCopy)
        {
            internalCopy = MemoryBlock (sourceData, sourceDataSize);
            data = internalCopy.getData();
        }
        */
    }

    /**
      | Creates a MemoryInputStream.
      | 
      | -----------
      | @param data
      | 
      | a block of data to use as the stream's
      | source
      | ----------
      | @param keepInternalCopyOfData
      | 
      | if false, the stream will just keep a
      | reference to the source data, so this
      | data shouldn't be changed for the lifetime
      | of the stream; if this parameter is true,
      | the stream will make its own copy of the
      | data and use that.
      |
      */
    pub fn new_from_source_data(
        source_data: &MemoryBlock,
        keep_copy:   bool) -> Self {
    
        todo!();
        /*


            : data (sourceData.getData()),
          dataSize (sourceData.getSize())

        if (keepCopy)
        {
            internalCopy = sourceData;
            data = internalCopy.getData();
        }
        */
    }

    /**
      | Creates a stream by moving from a MemoryBlock.
      |
      */
    pub fn new_from_memory_block(source: MemoryBlock) -> Self {
    
        todo!();
        /*


            : internalCopy (std::move (source))

        data = internalCopy.getData();
        dataSize = internalCopy.getSize();
        */
    }

    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return (int64) dataSize;
        */
    }

    pub fn read(&mut self, 
        buffer:   *mut c_void,
        how_many: i32) -> i32 {
        
        todo!();
        /*
            jassert (buffer != nullptr && howMany >= 0);

        if (howMany <= 0 || position >= dataSize)
            return 0;

        auto num = jmin ((size_t) howMany, dataSize - position);

        if (num > 0)
        {
            memcpy (buffer, addBytesToPointer (data, position), num);
            position += num;
        }

        return (int) num;
        */
    }

    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return position >= dataSize;
        */
    }

    pub fn set_position(&mut self, pos: i64) -> bool {
        
        todo!();
        /*
            position = (size_t) jlimit ((int64) 0, (int64) dataSize, pos);
        return true;
        */
    }

    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return (int64) position;
        */
    }

    pub fn skip_next_bytes(&mut self, num_bytes_to_skip: i64)  {
        
        todo!();
        /*
            if (numBytesToSkip > 0)
            setPosition (getPosition() + numBytesToSkip);
        */
    }
}
