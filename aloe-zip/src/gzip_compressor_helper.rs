crate::ix!();

#[no_copy]
pub struct GZIPCompressorHelper {

    stream:           z_stream,
    comp_level:       i32,
    is_first_deflate: bool, // default = true
    stream_is_valid:  bool, // default = false
    finished:         bool, // default = false
    buffer:           [Bytef; 32768],
}

pub const GZIP_COMPRESSOR_HELPER_STRATEGY: usize = 0;

impl Drop for GZIPCompressorHelper {

    fn drop(&mut self) {
        todo!();
        /* 
                if (streamIsValid)
                    zlib::deflateEnd (&stream);
             */
    }
}

impl GZIPCompressorHelper {

    pub fn new(
        compression_level: i32,
        window_bits:       i32) -> Self {
    
        todo!();
        /*


            : compLevel ((compressionLevel < 0 || compressionLevel > 9) ? -1 : compressionLevel)
                using namespace zlib;
                zerostruct (stream);

                streamIsValid = (deflateInit2 (&stream, compLevel, Z_DEFLATED,
                                               windowBits != 0 ? windowBits : MAX_WBITS,
                                               8, strategy) == Z_OK);
        */
    }
    
    pub fn write(
        &mut self, 
        data:      *const u8,
        data_size: usize,
        out:       &mut dyn Write

    ) -> bool {
        
        todo!();
        /*
            // When you call flush() on a gzip stream, the stream is closed, and you can
                // no longer continue to write data to it!
                jassert (! finished);

                while (dataSize > 0)
                    if (! doNextBlock (data, dataSize, out, Z_NO_FLUSH))
                        return false;

                return true;
        */
    }
    
    pub fn finish(&mut self, out: &mut dyn Write)  {
        
        todo!();
        /*
            const uint8* data = nullptr;
                size_t dataSize = 0;

                while (! finished)
                    doNextBlock (data, dataSize, out, Z_FINISH);
        */
    }
    
    pub fn do_next_block(
        &mut self, 
        data:       &mut *const u8,
        data_size:  &mut usize,
        out:        &mut dyn Write,
        flush_mode: i32

    ) -> bool {
        
        todo!();
        /*
            using namespace zlib;

                if (streamIsValid)
                {
                    stream.next_in   = const_cast<uint8*> (data);
                    stream.next_out  = buffer;
                    stream.avail_in  = (z_uInt) dataSize;
                    stream.avail_out = (z_uInt) sizeof (buffer);

                    auto result = isFirstDeflate ? deflateParams (&stream, compLevel, strategy)
                                                 : deflate (&stream, flushMode);
                    isFirstDeflate = false;

                    switch (result)
                    {
                        case Z_STREAM_END:
                            finished = true;
                            ALOE_FALLTHROUGH
                        case Z_OK:
                        {
                            data += dataSize - stream.avail_in;
                            dataSize = stream.avail_in;
                            auto bytesDone = (ssize_t) sizeof (buffer) - (ssize_t) stream.avail_out;
                            return bytesDone <= 0 || out.write (buffer, (size_t) bytesDone);
                        }

                        default:
                            break;
                    }
                }

                return false;
        */
    }
}
