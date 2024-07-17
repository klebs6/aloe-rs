crate::ix!();

pub const GZIP_DECOMP_BUFFER_SIZE: usize = 32768;

/**
  | internal helper object that holds the
  | zlib structures so they don't have to
  | be included publicly.
  |
  */
#[no_copy]
pub struct GZIPDecompressHelper {
    finished:         bool, // default = true
    needs_dictionary: bool, // default = false
    error:            bool, // default = true
    stream_is_valid:  bool, // default = false
    stream:           z_stream,
    data:             *mut u8, // default = nullptr
    data_size:        usize, // default = 0
}

impl Drop for GZIPDecompressHelper {

    fn drop(&mut self) {
        todo!();
        /* 
                if (streamIsValid)
                    zlib::inflateEnd (&stream);
             */
    }
}

impl GZIPDecompressHelper {

    pub fn new(f: GZipDecompressorInputStreamFormat) -> Self {
    
        todo!();
        /*


            using namespace zlib;
                zerostruct (stream);
                streamIsValid = (inflateInit2 (&stream, getBitsForGZipDecompressorInputStreamFormat (f)) == Z_OK);
                finished = error = ! streamIsValid;
        */
    }
    
    pub fn needs_input(&self) -> bool {
        
        todo!();
        /*
            return dataSize <= 0;
        */
    }
    
    pub fn set_input(&mut self, 
        data: *mut u8,
        size: usize)  {
        
        todo!();
        /*
            data = data_;
                dataSize = size;
        */
    }
    
    pub fn do_next_block(&mut self, 
        dest:      *mut u8,
        dest_size: u32) -> i32 {
        
        todo!();
        /*
            using namespace zlib;

                if (streamIsValid && data != nullptr && ! finished)
                {
                    stream.next_in  = data;
                    stream.next_out = dest;
                    stream.avail_in  = (z_uInt) dataSize;
                    stream.avail_out = (z_uInt) destSize;

                    switch (inflate (&stream, Z_PARTIAL_FLUSH))
                    {
                    case Z_STREAM_END:
                        finished = true;
                        ALOE_FALLTHROUGH
                    case Z_OK:
                        data += dataSize - stream.avail_in;
                        dataSize = (z_uInt) stream.avail_in;
                        return (int) (destSize - stream.avail_out);

                    case Z_NEED_DICT:
                        needsDictionary = true;
                        data += dataSize - stream.avail_in;
                        dataSize = (size_t) stream.avail_in;
                        break;

                    case Z_DATA_ERROR:
                    case Z_MEM_ERROR:
                        error = true;
                        ALOE_FALLTHROUGH
                    default:
                        break;
                    }
                }

                return 0;
        */
    }
    
    pub fn get_bits_for_format(f: GZipDecompressorInputStreamFormat) -> i32 {
        
        todo!();
        /*
            switch (f)
                {
                    case zlibGZipDecompressorInputStreamFormat:     return  MAX_WBITS;
                    case deflateGZipDecompressorInputStreamFormat:  return -MAX_WBITS;
                    case gzipGZipDecompressorInputStreamFormat:     return  MAX_WBITS | 16;
                    default:             jassertfalse; break;
                }

                return MAX_WBITS;
        */
    }
}
