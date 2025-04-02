crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/common/memorystream.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/common/memorystream.cpp]

pub const MEM_GROW_AMOUNT: TSize = 4096;

/**
  | Memory based Stream for IBStream
  | implementation (using malloc). \ingroup
  | sdkBase
  |
  */
#[DECLARE_FUNKNOWN_METHODS]
pub struct MemoryStream {

    /**
      | memory block
      |
      */
    memory:           *mut u8,

    /**
      | size of the memory block
      |
      */
    memory_size:      TSize,

    /**
      | size of the stream
      |
      */
    size:             TSize,

    /**
      | stream pointer
      |
      */
    cursor:           i64,

    /**
      | stream has allocated memory itself
      |
      */
    own_memory:       bool,

    /**
      | stream invalid
      |
      */
    allocation_error: bool,
}

implement_funknown_methods!{
    MemoryStream, 
    IBStream, 
    IBStream::iid
}

impl Drop for MemoryStream {

    fn drop(&mut self) {
        todo!();
        /*
            if (ownMemory && memory)
            ::free (memory);

        FUNKNOWN_DTOR
        */
    }
}

impl FUnknown for MemoryStream {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { 
        todo!() 
    }

    fn add_ref(&mut self) -> u32 { 
        todo!() 
    }

    fn release(&mut self) -> u32 { 
        todo!() 
    }
}

impl IBStream for MemoryStream {

    #[PLUGIN_API]
    fn read(&mut self, 
        buffer:         *mut c_void,
        num_bytes:      i32,
        num_bytes_read: *mut i32) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn write(&mut self, 
        buffer:            *mut c_void,
        num_bytes:         i32,
        num_bytes_written: *mut i32) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn seek(&mut self, 
        pos:    i64,
        mode:   i32,
        result: *mut i64) -> tresult {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    fn tell(&mut self, pos: *mut i64) -> tresult {
        
        todo!();
        /*
        
        */
    }
}

impl Default for MemoryStream {

    fn default() -> Self {
    
        todo!();
        /*
        : memory(nullptr),
        : memory_size(0),
        : size(0),
        : cursor(0),
        : own_memory(true),
        : allocation_error(false),

            FUNKNOWN_CTOR
        */
    }
}
    
impl MemoryStream {

    /**
      | reuse a given memory without getting
      | ownership
      |
      */
    pub fn new(
        data:   *mut c_void,
        length: TSize) -> Self {
    
        todo!();
        /*

        : memory ((char*)data)
        , memorySize (length)
        , size (length)
        , cursor (0)
        , ownMemory (false)
        , allocationError (false)
        FUNKNOWN_CTOR
        */
    }
    
    #[PLUGIN_API]
    pub fn read(&mut self, 
        data:           *mut c_void,
        num_bytes:      i32,
        num_bytes_read: *mut i32) -> tresult {
        
        todo!();
        /*
            if (memory == nullptr)
        {
            if (allocationError)
                return kOutOfMemory;
            numBytes = 0;
        }
        else
        {       
            // Does read exceed size ?
            if (cursor + numBytes > size)
            {
                int32 maxBytes = int32 (size - cursor);

                // Has length become zero or negative ?
                if (maxBytes <= 0) 
                {
                    cursor = size;
                    numBytes = 0;
                }
                else
                    numBytes = maxBytes;
            }
            
            if (numBytes)
            {
                memcpy (data, &memory[cursor], static_cast<size_t> (numBytes));
                cursor += numBytes;
            }
        }

        if (numBytesRead)
            *numBytesRead = numBytes;

        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn write(&mut self, 
        buffer:            *mut c_void,
        num_bytes:         i32,
        num_bytes_written: *mut i32) -> tresult {
        
        todo!();
        /*
            if (allocationError)
            return kOutOfMemory;
        if (buffer == nullptr)
            return kInvalidArgument;

        // Does write exceed size ?
        TSize requiredSize = cursor + numBytes;
        if (requiredSize > size) 
        {       
            if (requiredSize > memorySize)
                setSize (requiredSize);
            else
                size = requiredSize;
        }
        
        // Copy data
        if (memory && cursor >= 0 && numBytes > 0)
        {
            memcpy (&memory[cursor], buffer, static_cast<size_t> (numBytes));
            // Update cursor
            cursor += numBytes;
        }
        else
            numBytes = 0;

        if (numBytesWritten)
            *numBytesWritten = numBytes;

        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn seek(&mut self, 
        pos:    i64,
        mode:   i32,
        result: *mut i64) -> tresult {
        
        todo!();
        /*
            switch (mode) 
        {
            case kIBSeekSet:
                cursor = pos;
                break;
            case kIBSeekCur:
                cursor = cursor + pos;
                break;
            case kIBSeekEnd:
                cursor = size + pos;
                break;
        }

        if (ownMemory == false)
            if (cursor > memorySize)
                cursor = memorySize;

        if (result)
            *result = cursor;

        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn tell(&mut self, pos: *mut i64) -> tresult {
        
        todo!();
        /*
            if (!pos)
            return kInvalidArgument;

        *pos = cursor;
        return kResultTrue;
        */
    }
    
    /**
      | returns the current memory size
      |
      */
    pub fn get_size(&self) -> TSize {
        
        todo!();
        /*
            return size;
        */
    }
    
    /**
      | set the memory size, a realloc will occur
      | if memory already used
      |
      */
    pub fn set_size(&mut self, s: TSize)  {
        
        todo!();
        /*
            if (s <= 0)
        {
            if (ownMemory && memory)
                free (memory);

            memory = nullptr;
            memorySize = 0;
            size = 0;
            cursor = 0;
            return;
        }

        TSize newMemorySize = (((Max (memorySize, s) - 1) / kMemGrowAmount) + 1) * kMemGrowAmount;
        if (newMemorySize == memorySize)
        {
            size = s;
            return;
        }

        if (memory && ownMemory == false)
        {
            allocationError = true;
            return; 
        }

        ownMemory = true;
        char* newMemory = nullptr;

        if (memory)
        {
            newMemory = (char*)realloc (memory, (size_t)newMemorySize);
            if (newMemory == nullptr && newMemorySize > 0)
            {
                newMemory = (char*)malloc ((size_t)newMemorySize);
                if (newMemory)
                {
                    memcpy (newMemory, memory, (size_t)Min (newMemorySize, memorySize));           
                    free (memory);
                }       
            }
        }
        else
            newMemory = (char*)malloc ((size_t)newMemorySize);

        if (newMemory == nullptr)
        {
            if (newMemorySize > 0)
                allocationError = true;

            memory = nullptr;
            memorySize = 0;
            size = 0;
            cursor = 0;
        }
        else
        {
            memory = newMemory;
            memorySize = newMemorySize;
            size = s;
        }
        */
    }
    
    /**
      | returns the memory pointer
      |
      */
    pub fn get_data(&self) -> *mut u8 {
        
        todo!();
        /*
            return memory;
        */
    }
    
    /**
      | returns the memory pointer and give
      | up ownership
      |
      */
    pub fn detach_data(&mut self) -> *mut u8 {
        
        todo!();
        /*
            if (ownMemory)
        {
            char* result = memory;
            memory = nullptr;
            memorySize = 0;
            size = 0;
            cursor = 0;
            return result;
        }
        return nullptr;
        */
    }
    
    /**
      | realloc to the current use memory size
      | if needed
      |
      */
    pub fn truncate(&mut self) -> bool {
        
        todo!();
        /*
            if (ownMemory == false)
            return false;

        if (memorySize == size)
            return true;

        memorySize = size;
        
        if (memorySize == 0)
        {
            if (memory)
            {
                free (memory);
                memory = nullptr;
            }
        }
        else
        {
            if (memory)
            {
                char* newMemory = (char*)realloc (memory, (size_t)memorySize);
                if (newMemory)
                    memory = newMemory;
            }
        }
        return true;
        */
    }
    
    /**
      | truncate memory at current cursor position
      |
      */
    pub fn truncate_to_cursor(&mut self) -> bool {
        
        todo!();
        /*
            size = cursor;
        return truncate ();
        */
    }
}
