crate::ix!();

/**
  | Stream representing a Read-Only subsection
  | of its source stream.
  |
  */
 #[DECLARE_FUNKNOWN_METHODS]
pub struct ReadOnlyBStream {
    source_stream: *mut dyn IBStream,
    source_offset: TSize,
    section_size:  TSize,
    seek_position: TSize,
}

implement_refcount!{ ReadOnlyBStream }

impl IBStream for ReadOnlyBStream {

    #[PLUGIN_API]
    fn read(&mut self, 
        buffer:         *mut c_void,
        num_bytes:      i32,
        num_bytes_read: *mut i32) -> tresult {
        
        todo!();
        /*
            if (numBytesRead)
            *numBytesRead = 0;

        if (!sourceStream)
            return kNotInitialized;

        int32 maxBytesToRead = (int32) (sectionSize - seekPosition);
        if (numBytes > maxBytesToRead)
            numBytes = maxBytesToRead;
        if (numBytes <= 0)
            return kResultOk;

        tresult result = sourceStream->seek (sourceOffset + seekPosition, kIBSeekSet);
        if (result != kResultOk)
            return result;

        int32 numRead = 0;
        result = sourceStream->read (buffer, numBytes, &numRead);

        if (numRead > 0)
            seekPosition += numRead;
        if (numBytesRead)
            *numBytesRead = numRead;

        return result;
        */
    }
    
    #[PLUGIN_API]
    fn write(&mut self, 
        buffer:            *mut c_void,
        num_bytes:         i32,
        num_bytes_written: *mut i32) -> tresult {
        
        todo!();
        /*
            if (numBytesWritten)
            *numBytesWritten = 0;

        return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    fn seek(&mut self, 
        pos:    i64,
        mode:   i32,
        result: *mut i64) -> tresult {
        
        todo!();
        /*
            switch (mode)
        {
            case kIBSeekSet: seekPosition = pos; break;

            case kIBSeekCur: seekPosition += pos; break;

            case kIBSeekEnd: seekPosition = sectionSize + pos; break;
        }

        if (seekPosition < 0)
            seekPosition = 0;
        if (seekPosition > sectionSize)
            seekPosition = sectionSize;

        if (result)
            *result = seekPosition;
        return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    fn tell(&mut self, pos: *mut i64) -> tresult {
        
        todo!();
        /*
            if (pos)
            *pos = seekPosition;
        return kResultOk;
        */
    }
}

impl FUnknown for ReadOnlyBStream {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl Drop for ReadOnlyBStream {

    fn drop(&mut self) {
        todo!();
        /*
            if (sourceStream)
            sourceStream->release ();

        FUNKNOWN_DTOR
        */
    }
}

impl ReadOnlyBStream {

    pub fn new(
        source_stream: *mut dyn IBStream,
        source_offset: TSize,
        section_size:  TSize) -> Self {
    
        todo!();
        /*
        : source_stream(sourceStream),
        : source_offset(sourceOffset),
        : section_size(sectionSize),
        : seek_position(0),

            FUNKNOWN_CTOR
        if (sourceStream)
            sourceStream->addRef ();
        */
    }
    
    #[PLUGIN_API]
    pub fn query_interface(&mut self, 
        iid: TUID,
        obj: *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            return sourceStream ? sourceStream->queryInterface (_iid, obj) : kResultFalse;
        */
    }
    
}
