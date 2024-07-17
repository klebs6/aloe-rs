crate::ix!();

/**
  | Stream implementation for a memory
  | buffer.
  |
  */
#[DECLARE_FUNKNOWN_METHODS]
pub struct BufferStream {
    buffer: Buffer,
}

impl IBStream for BufferStream {

    #[PLUGIN_API]
    fn read(&mut self, 
        buffer:         *mut c_void,
        num_bytes:      i32,
        num_bytes_read: *mut i32) -> tresult {
        
        todo!();
        /*
            uint32 size = mBuffer.get (buffer, static_cast<uint32> (numBytes));
        if (numBytesRead)
            *numBytesRead = static_cast<int32> (size);

        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn write(&mut self, 
        buffer:            *mut c_void,
        num_bytes:         i32,
        num_bytes_written: *mut i32) -> tresult {
        
        todo!();
        /*
            bool res = mBuffer.put (buffer, static_cast<uint32> (numBytes));
        if (numBytesWritten)
            *numBytesWritten = res ? numBytes : 0;

        return res ? kResultTrue : kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn seek(&mut self, 
        pos:    i64,
        mode:   i32,
        result: *mut i64) -> tresult {
        
        todo!();
        /*
            bool res = false;
        switch (mode)
        {
            //--- -----------------
            case IBStream::kIBSeekSet:
            {
                int64 tmp = pos;
                if (tmp < 0)
                    tmp = 0;
                res = mBuffer.setFillSize (static_cast<uint32> (tmp));
            }
            break;

            //--- -----------------
            case IBStream::kIBSeekCur:
            {
                int64 tmp = mBuffer.getFillSize () + pos;
                if (tmp < 0)
                    tmp = 0;
                res = mBuffer.setFillSize (static_cast<uint32> (tmp));
            }
            break;

            //--- -----------------
            case IBStream::kIBSeekEnd:
            {
                int64 tmp = mBuffer.getSize () - pos;
                if (tmp < 0)
                    tmp = 0;
                res = mBuffer.setFillSize (static_cast<uint32> (tmp));
            }
            break;
        }
        if (res && result)
            *result = mBuffer.getFillSize ();

        return res ? kResultTrue : kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn tell(&mut self, pos: *mut i64) -> tresult {
        
        todo!();
        /*
            if (pos)
            *pos = mBuffer.getFillSize ();
        return pos ? kResultTrue : kResultFalse;
        */
    }
}

impl FUnknown for BufferStream {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

implement_funknown_methods!{
    BufferStream, 
    IBStream, 
    IBStream::iid
}

impl Drop for BufferStream {

    fn drop(&mut self) {
        todo!();
        /*
            FUNKNOWN_DTOR
        */
    }
}

impl Default for BufferStream {

    fn default() -> Self {
    
        todo!();
        /*


            FUNKNOWN_CTOR
        */
    }
}
