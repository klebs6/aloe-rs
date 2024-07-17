crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_MemoryOutputStream.h]

/**
  | Writes data to an internal memory buffer,
  | which grows as required.
  | 
  | The data that was written into the stream
  | can then be accessed later as a contiguous
  | block of memory.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MemoryOutputStream {
    base:           OutputStream,
    block_to_use:   *const MemoryBlock, // default = nullptr
    internal_block: MemoryBlock,
    external_data:  *mut c_void, // default = nullptr
    position:       usize, // default = 0
    size:           usize, // default = 0
    available_size: usize, // default = 0
}

impl Shl<&MemoryOutputStream> for &mut OutputStream {

    type Output = OutputStream;

    /**
      | Copies all the data that has been written
      | to a MemoryOutputStream into another
      | stream.
      |
      */
    #[inline] fn shl(self, rhs: &MemoryOutputStream) -> Self::Output {
        todo!();
        /*
            auto dataSize = streamToRead.getDataSize();

        if (dataSize > 0)
            stream.write (streamToRead.getData(), dataSize);

        return stream;
        */
    }
}


impl Drop for MemoryOutputStream {

    /**
      | Destructor.
      | 
      | This will free any data that was written
      | to it.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        trimExternalBlockSize();
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_MemoryOutputStream.cpp]
impl MemoryOutputStream {

    /**
      | Returns the number of bytes of data that
      | have been written to the stream.
      | 
      | @see getData
      |
      */
    pub fn get_data_size(&self) -> usize {
        
        todo!();
        /*
            return size;
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return (int64) position;
        */
    }
    
    /**
      | Creates an empty memory stream, ready
      | to be written into.
      | 
      | -----------
      | @param initialSize
      | 
      | the initial amount of capacity to allocate
      | for writing into
      |
      */
    pub fn new_with_initial_size(initial_size: Option<usize>) -> Self {

        let initial_size: usize = initial_size.unwrap_or(256);
    
        todo!();
        /*
        : blockToUse (&internalBlock)

        internalBlock.setSize (initialSize, false);
        */
    }

    /**
      | Creates a memory stream for writing
      | into into a pre-existing MemoryBlock
      | object.
      | 
      | -----------
      | @note
      | 
      | the destination block will always be
      | larger than the amount of data that has
      | been written to the stream, because
      | the MemoryOutputStream keeps some
      | spare capacity at its end. To trim the
      | block's size down to fit the actual data,
      | call flush(), or delete the MemoryOutputStream.
      | 
      | -----------
      | @param memoryBlockToWriteTo
      | 
      | the block into which new data will be
      | written.
      | ----------
      | @param appendToExistingBlockContent
      | 
      | if this is true, the contents of the block
      | will be kept, and new data will be appended
      | to it. If false, the block will be cleared
      | before use
      |
      */
    pub fn new_into_preexisting_memory_block(
        memory_block_to_write_to:         &mut MemoryBlock,
        append_to_existing_block_content: bool) -> Self {
    
        todo!();
        /*
        : block_to_use(&memoryBlockToWriteTo),

            if (appendToExistingBlockContent)
            position = size = memoryBlockToWriteTo.getSize();
        */
    }

    /**
      | Creates a MemoryOutputStream that
      | will write into a user-supplied, fixed-size
      | block of memory.
      | 
      | When using this mode, the stream will
      | write directly into this memory area
      | until it's full, at which point write
      | operations will fail.
      |
      */
    pub fn new_into_dest_buffer(
        dest_buffer:      *mut c_void,
        dest_buffer_size: usize) -> Self {
    
        todo!();
        /*
        : external_data(destBuffer),
        : available_size(destBufferSize),

            jassert (externalData != nullptr); // This must be a valid pointer.
        */
    }

    /**
      | If the stream is writing to a user-supplied
      | MemoryBlock, this will trim any excess
      | capacity off the block, so that its length
      | matches the amount of actual data that
      | has been written so far.
      |
      */
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            trimExternalBlockSize();
        */
    }

    pub fn trim_external_block_size(&mut self)  {
        
        todo!();
        /*
            if (blockToUse != &internalBlock && blockToUse != nullptr)
            blockToUse->setSize (size, false);
        */
    }

    /**
      | Increases the internal storage capacity
      | to be able to contain at least the specified
      | amount of data without needing to be
      | resized.
      |
      */
    pub fn preallocate(&mut self, bytes_to_preallocate: usize)  {
        
        todo!();
        /*
            if (blockToUse != nullptr)
            blockToUse->ensureSize (bytesToPreallocate + 1);
        */
    }

    /**
      | Resets the stream, clearing any data
      | that has been written to it so far.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            position = 0;
        size = 0;
        */
    }

    pub fn prepare_to_write(&mut self, num_bytes: usize) -> *mut u8 {
        
        todo!();
        /*
            jassert ((ssize_t) numBytes >= 0);
        auto storageNeeded = position + numBytes;

        char* data;

        if (blockToUse != nullptr)
        {
            if (storageNeeded >= blockToUse->getSize())
                blockToUse->ensureSize ((storageNeeded + jmin (storageNeeded / 2, (size_t) (1024 * 1024)) + 32) & ~31u);

            data = static_cast<char*> (blockToUse->getData());
        }
        else
        {
            if (storageNeeded > availableSize)
                return nullptr;

            data = static_cast<char*> (externalData);
        }

        auto* writePointer = data + position;
        position += numBytes;
        size = jmax (size, position);
        return writePointer;
        */
    }

    pub fn write(&mut self, 
        buffer:   *const c_void,
        how_many: usize) -> bool {
        
        todo!();
        /*
            if (howMany == 0)
            return true;

        jassert (buffer != nullptr);

        if (auto* dest = prepareToWrite (howMany))
        {
            memcpy (dest, buffer, howMany);
            return true;
        }

        return false;
        */
    }

    pub fn write_repeated_byte(&mut self, 
        byte:     u8,
        how_many: usize) -> bool {
        
        todo!();
        /*
            if (howMany == 0)
            return true;

        if (auto* dest = prepareToWrite (howMany))
        {
            memset (dest, byte, howMany);
            return true;
        }

        return false;
        */
    }

    /**
      | Appends the utf-8 bytes for a unicode
      | character
      |
      */
    pub fn append_utf8char(&mut self, c: wchar_t) -> bool {
        
        todo!();
        /*
            if (auto* dest = prepareToWrite (CharPointer_UTF8::getBytesRequiredFor (c)))
        {
            CharPointer_UTF8 (dest).write (c);
            return true;
        }

        return false;
        */
    }

    /**
      | Returns a copy of the stream's data as
      | a memory block.
      |
      */
    pub fn get_memory_block(&self) -> MemoryBlock {
        
        todo!();
        /*
            return MemoryBlock (getData(), getDataSize());
        */
    }

    /**
      | Returns a pointer to the data that has
      | been written to the stream.
      | 
      | @see getDataSize
      |
      */
    pub fn get_data(&self)  {
        
        todo!();
        /*
            if (blockToUse == nullptr)
            return externalData;

        if (blockToUse->getSize() > size)
            static_cast<char*> (blockToUse->getData()) [size] = 0;

        return blockToUse->getData();
        */
    }

    pub fn set_position(&mut self, new_position: i64) -> bool {
        
        todo!();
        /*
            if (newPosition <= (int64) size)
        {
            // ok to seek backwards
            position = jlimit ((size_t) 0, size, (size_t) newPosition);
            return true;
        }

        // can't move beyond the end of the stream..
        return false;
        */
    }

    pub fn write_from_input_stream<R: Read>(
        &mut self, 
        source:                 &mut R,
        max_num_bytes_to_write: i64) -> i64 {
        
        todo!();
        /*
            // before writing from an input, see if we can preallocate to make it more efficient..
        int64 availableData = source.getTotalLength() - source.getPosition();

        if (availableData > 0)
        {
            if (maxNumBytesToWrite > availableData || maxNumBytesToWrite < 0)
                maxNumBytesToWrite = availableData;

            if (blockToUse != nullptr)
                preallocate (blockToUse->getSize() + (size_t) maxNumBytesToWrite);
        }

        return OutputStream::writeFromInputStream (source, maxNumBytesToWrite);
        */
    }

    /**
      | Returns a String created from the (UTF8)
      | data that has been written to the stream.
      |
      */
    pub fn toutf8(&self) -> String {
        
        todo!();
        /*
            auto* d = static_cast<const char*> (getData());
        return String (CharPointer_UTF8 (d), CharPointer_UTF8 (d + getDataSize()));
        */
    }

    /**
      | Attempts to detect the encoding of the
      | data and convert it to a string.
      | 
      | @see String::createStringFromData
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return String::createStringFromData (getData(), (int) getDataSize());
        */
    }
}
