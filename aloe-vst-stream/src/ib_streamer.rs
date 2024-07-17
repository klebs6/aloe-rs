crate::ix!();

/**
  | Wrapper class for typed reading/writing
  | from or to IBStream.
  | 
  | Can be used framework-independent
  | in plug-ins.
  |
  */
pub struct IBStreamer {
    base:   FStreamer,
    stream: *mut dyn IBStream,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/fstreamer.cpp]
impl IBStreamer {

    /**
      | Returns the associated IBStream.
      |
      */
    pub fn get_stream(&mut self) -> *mut dyn IBStream {
        
        todo!();
        /*
            return stream;
        */
    }

    /* ------------- FStreamer overrides:  ------------- */

    /**
      | Constructor for a given IBSTream and
      | a byteOrder.
      |
      */
    pub fn new(
        stream:     *mut dyn IBStream,
        byte_order: Option<i16>

    ) -> Self {

        let byte_order: i16 = byte_order.unwrap_or(byteorder![] as i16);
    
        todo!();
        /*
        : streamer(_byteOrder),
        : stream(stream),

        
        */
    }
    
    /**
      | Read one buffer of size.
      |
      */
    #[SMTG_OVERRIDE]
    pub fn read_raw(&mut self, 
        buffer: *mut c_void,
        size:   TSize) -> TSize {
        
        todo!();
        /*
            int32 numBytesRead = 0;
        stream->read (buffer, (int32)size, &numBytesRead);
        return numBytesRead;
        */
    }
    
    /**
      | Write one buffer of size.
      |
      */
    #[SMTG_OVERRIDE]
    pub fn write_raw(&mut self, 
        buffer: *const c_void,
        size:   TSize) -> TSize {
        
        todo!();
        /*
            int32 numBytesWritten = 0;
        stream->write ((void*)buffer, (int32)size, &numBytesWritten);
        return numBytesWritten;
        */
    }
    
    /**
      | Set file position for stream.
      |
      */
    #[SMTG_OVERRIDE]
    pub fn seek(&mut self, 
        pos:  i64,
        mode: FSeekMode) -> i64 {
        
        todo!();
        /*
            int64 result = -1;
        stream->seek (pos, mode, &result);
        return result;
        */
    }
    
    /**
      | Return current file position.
      |
      */
    #[SMTG_OVERRIDE]
    pub fn tell(&mut self) -> i64 {
        
        todo!();
        /*
            int64 pos = 0;
        stream->tell (&pos);
        return pos;
        */
    }
}
