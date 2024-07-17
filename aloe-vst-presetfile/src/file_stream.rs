crate::ix!();

/**
  | Stream implementation for a file using
  | stdio.
  |
  */
#[DECLARE_FUNKNOWN_METHODS]
pub struct FileStream {
    file: *mut libc::FILE,
}

impl IBStream for FileStream {

    #[PLUGIN_API]
    fn read(&mut self, 
        buffer:         *mut c_void,
        num_bytes:      i32,
        num_bytes_read: *mut i32) -> tresult {
        
        todo!();
        /*
            size_t result = fread (buffer, 1, static_cast<size_t> (numBytes), file);
        if (numBytesRead)
            *numBytesRead = (int32)result;
        return static_cast<int32> (result) == numBytes ? kResultOk : kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn write(&mut self, 
        buffer:            *mut c_void,
        num_bytes:         i32,
        num_bytes_written: *mut i32) -> tresult {
        
        todo!();
        /*
            size_t result = fwrite (buffer, 1, static_cast<size_t> (numBytes), file);
        if (numBytesWritten)
            *numBytesWritten = (int32)result;
        return static_cast<int32> (result) == numBytes ? kResultOk : kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn seek(&mut self, 
        pos:    i64,
        mode:   i32,
        result: *mut i64) -> tresult {
        
        todo!();
        /*
            if (fseek (file, (int32)pos, mode) == 0)
        {
            if (result)
                *result = ftell (file);
            return kResultOk;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn tell(&mut self, pos: *mut i64) -> tresult {
        
        todo!();
        /*
            if (pos)
            *pos = ftell (file);
        return kResultOk;
        */
    }
}

impl FUnknown for FileStream {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl Drop for FileStream {

    fn drop(&mut self) {
        todo!();
        /*
            fclose (file);
        FUNKNOWN_DTOR
        */
    }
}

implement_funknown_methods!{
    FileStream, 
    IBStream, 
    IBStream::iid
}

impl FileStream {

    /**
      | open a stream using stdio function
      |
      */
    #[PLUGIN_API]
    pub fn open(&mut self, 
        filename: *const u8,
        mode:     *const u8) -> *mut dyn IBStream {
        
        todo!();
        /*
            FILE* file = fopen (filename, mode);
        return file ? new FileStream (file) : nullptr;
        */
    }
    
    pub fn new(file: *mut libc::FILE) -> Self {
    
        todo!();
        /*
        : file(file),

            FUNKNOWN_CTOR
        */
    }
}
