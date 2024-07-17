crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/ibstream.h]

pub enum IStreamSeekMode
{
    /**
      | set absolute seek position
      |
      */
    kIBSeekSet = 0, 

    /**
      | set seek position relative to current
      | position
      |
      */
    kIBSeekCur,     

    /**
      | set seek position relative to stream
      | end
      |
      */
    kIBSeekEnd      
}

/**
  | Base class for streams. \ingroup pluginBase
  | 
  | - read/write binary data from/to stream
  | 
  | - get/set stream read-write position
  | (read and write position is the same)
  |
  */
pub trait IBStream: FUnknown {

    /**
      | Reads binary data from stream.
      | 
      | -----------
      | @param buffer
      | 
      | : destination buffer
      | ----------
      | @param numBytes
      | 
      | : amount of bytes to be read
      | ----------
      | @param numBytesRead
      | 
      | : result - how many bytes have been read
      | from stream (set to 0 if this is of no interest)
      |
      */
    #[PLUGIN_API]
    fn read(&mut self, 
            buffer:         *mut c_void,
            num_bytes:      i32,
            num_bytes_read: *mut i32) -> tresult;

    /**
      | Writes binary data to stream.
      | 
      | -----------
      | @param buffer
      | 
      | : source buffer
      | ----------
      | @param numBytes
      | 
      | : amount of bytes to write
      | ----------
      | @param numBytesWritten
      | 
      | : result - how many bytes have been written
      | to stream (set to 0 if this is of no interest)
      |
      */
    #[PLUGIN_API]
    fn write(&mut self, 
            buffer:            *mut c_void,
            num_bytes:         i32,
            num_bytes_written: *mut i32) -> tresult;

    /**
      | Sets stream read-write position.
      | 
      | -----------
      | @param pos
      | 
      | : new stream position (dependent on
      | mode)
      | ----------
      | @param mode
      | 
      | : value of enum IStreamSeekMode
      | ----------
      | @param result
      | 
      | : new seek position (set to 0 if this is
      | of no interest)
      |
      */
    #[PLUGIN_API]
    fn seek(&mut self, 
            pos:    i64,
            mode:   i32,
            result: *mut i64) -> tresult;

    /**
      | Gets current stream read-write position.
      | 
      | -----------
      | @param pos
      | 
      | : is assigned the current position if
      | function succeeds
      |
      */
    #[PLUGIN_API]
    fn tell(&mut self, pos: *mut i64) -> tresult;
}

lazy_static!{
    /*
    static const FUID ib_stream_iid;
    */
}

declare_class_iid!{
    IBStream, 
    0xC3BF6EA2, 
    0x30994752, 
    0x9B6BF990, 
    0x1EE33E9B
}
