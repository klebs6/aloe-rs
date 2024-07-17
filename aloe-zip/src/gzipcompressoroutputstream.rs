crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/aloe_GZIPCompressorOutputStream.h]

/**
  | These are preset values that can be used
  | for the constructor's windowBits parameter.
  | 
  | For more info about this, see the zlib
  | documentation for its windowBits parameter.
  |
  */
pub enum GZipDecompressorOutputStreamWindowBitsValues
{
    windowBitsRaw = -15,
    windowBitsGZIP = 15 + 16
}

/**
  | A stream which uses zlib to compress
  | the data written into it.
  | 
  | Important note: When you call flush()
  | on a GZIPCompressorOutputStream,
  | the gzip data is closed - this means that
  | no more data can be written to it, and
  | any subsequent attempts to call write()
  | will cause an assertion.
  | 
  | @see GZIPDecompressorInputStream
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct GZIPCompressorOutputStream {
    dest_stream: OptionalScopedPointer<Box<dyn Write>>,
    helper:      Box<GZIPCompressorHelper>,
}

impl Write for GZIPCompressorOutputStream {

    fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error> { 
        todo!() 
    }

    fn flush(&mut self) -> Result<(), std::io::Error> { 
        todo!() 
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/aloe_GZIPCompressorOutputStream.cpp]
impl Drop for GZIPCompressorOutputStream {

    fn drop(&mut self) {
        todo!();
        /* 
        flush();
        */
    }
}

impl GZIPCompressorOutputStream {

    /**
      | Creates a compression stream.
      | 
      | -----------
      | @param destStream
      | 
      | the stream into which the compressed
      | data will be written
      | ----------
      | @param compressionLevel
      | 
      | how much to compress the data, between
      | 0 and 9, where 0 is non-compressed storage,
      | 1 is the fastest/lowest compression,
      | and 9 is the slowest/highest compression.
      | Any value outside this range indicates
      | that a default compression level should
      | be used.
      | ----------
      | @param windowBits
      | 
      | this is used internally to change the
      | window size used by zlib - leave it as
      | 0 unless you specifically need to set
      | its value for some reason
      |
      */
    pub fn new(
        dest_stream:       &mut dyn Write,
        compression_level: Option<i32>,
        window_bits:       Option<i32>

    ) -> Self {

        let compression_level: i32 =
            compression_level.unwrap_or(-1);

        let window_bits: i32 = window_bits.unwrap_or(0);

        todo!();
        /*
        : gzip_compressor_output_stream(&s, compressionLevel, false, windowBits),
        
        */
    }

    /**
      | Creates a compression stream.
      | -----------
      | @param destStream
      | 
      | the stream into which the compressed
      | data will be written. Ownership of this
      | object depends on the value of deleteDestStreamWhenDestroyed
      | ----------
      | @param compressionLevel
      | 
      | how much to compress the data, between
      | 0 and 9, where 0 is non-compressed storage,
      | 1 is the fastest/lowest compression,
      | and 9 is the slowest/highest compression.
      | Any value outside this range indicates
      | that a default compression level should
      | be used.
      | ----------
      | @param deleteDestStreamWhenDestroyed
      | 
      | whether or not the GZIPCompressorOutputStream
      | will delete the destStream object when
      | it is destroyed
      | ----------
      | @param windowBits
      | 
      | this is used internally to change the
      | window size used by zlib - leave it as
      | 0 unless you specifically need to set
      | its value for some reason
      |
      */
    pub fn new_possibly_delete_dest_stream_when_destroyed(
        dest_stream:                       *mut dyn Write,
        compression_level:                 Option<i32>,
        delete_dest_stream_when_destroyed: Option<bool>,
        window_bits:                       Option<i32>

    ) -> Self {

        let compression_level: i32 =
            compression_level.unwrap_or(-1);

        let delete_dest_stream_when_destroyed: bool =
            delete_dest_stream_when_destroyed.unwrap_or(false);

        let window_bits: i32 = window_bits.unwrap_or(0);
    
        todo!();
        /*


            : destStream (out, deleteDestStream),
         helper (new GZIPCompressorHelper (compressionLevel, windowBits))

        jassert (out != nullptr);
        */
    }
    

    /**
      | Flushes and closes the stream.
      | 
      | -----------
      | @note
      | 
      | unlike most streams, when you call flush()
      | on a GZIPCompressorOutputStream,
      | the stream is closed - this means that
      | no more data can be written to it, and
      | any subsequent attempts to call write()
      | will cause an assertion.
      |
      */
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            helper->finish (*destStream);
        destStream->flush();
        */
    }
    
    pub fn write(&mut self, 
        dest_buffer: *const c_void,
        how_many:    usize) -> bool {
        
        todo!();
        /*
            jassert (destBuffer != nullptr && (ssize_t) howMany >= 0);

        return helper->write (static_cast<const uint8*> (destBuffer), howMany, *destStream);
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return destStream->getPosition();
        */
    }
    
    pub fn set_position(&mut self, new_position: i64) -> bool {
        
        todo!();
        /*
            jassertfalse; // can't do it!
        return false;
        */
    }
}
