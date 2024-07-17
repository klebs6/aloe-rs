crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/aloe_GZIPDecompressorInputStream.h]
pub enum GZipDecompressorInputStreamFormat
{
    Zlib = 0,
    Deflate,
    Gzip
}

/**
  | This stream will decompress a source-stream
  | using zlib.
  | 
  | Tip: if you're reading lots of small
  | items from one of these streams, you
  | can increase the performance enormously
  | by passing it through a BufferedInputStream,
  | so that it has to read larger blocks less
  | often.
  | 
  | @see GZIPCompressorOutputStream
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct GZIPDecompressorInputStream {
    source_stream:              OptionalScopedPointer<Box<dyn Read>>,
    uncompressed_stream_length: i64,
    format:                     GZipDecompressorInputStreamFormat,
    is_eof:                     bool, // default = false
    active_buffer_size:         i32, // default = 0
    original_source_pos:        i64,
    current_pos:                i64, // default = 0
    buffer:                     HeapBlock<u8>,
    helper:                     Box<GZIPDecompressHelper>,
}

impl Read for GZIPDecompressorInputStream {

    fn read(&mut self, _: &mut [u8]) -> Result<usize, std::io::Error> { 
        todo!() 
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/zip/aloe_GZIPDecompressorInputStream.cpp]
impl GZIPDecompressorInputStream {

    /**
      | Creates a decompressor stream.
      | 
      | -----------
      | @param sourceStream
      | 
      | the stream to read from
      | ----------
      | @param deleteSourceWhenDestroyed
      | 
      | whether or not to delete the source stream
      | when this object is destroyed
      | ----------
      | @param sourceGZipDecompressorInputStreamFormat
      | 
      | can be used to select which of the supported
      | formats the data is expected to be in
      | ----------
      | @param uncompressedStreamLength
      | 
      | if the creator knows the length that
      | the uncompressed stream will be, then
      | it can supply this value, which will
      | be returned by getTotalLength()
      |
      */
    pub fn new(
        source_stream:                *mut dyn Read,
        delete_source_when_destroyed: bool,
        source_format:                Option<GZipDecompressorInputStreamFormat>,
        uncompressed_stream_length:   Option<i64>

    ) -> Self {

        let source_format 
            = source_format.unwrap_or(GZipDecompressorInputStreamFormat::Zlib);

        let uncompressed_stream_length 
            = uncompressed_stream_length.unwrap_or(-1);

        todo!();
        /*

            : sourceStream (source, deleteSourceWhenDestroyed),
        uncompressedStreamLength (uncompressedLength),
        format (f),
        originalSourcePos (source->getPosition()),
        buffer ((size_t) GZIPDecompressHelper::gzipDecompBufferSize),
        helper (new GZIPDecompressHelper (f))
        */
    }

    /**
      | Creates a decompressor stream.
      | 
      | -----------
      | @param sourceStream
      | 
      | the stream to read from - the source stream
      | must not be deleted until this object
      | has been destroyed
      |
      */
    pub fn new_from_stream(source: &mut dyn Read) -> Self {
    
        todo!();
        /*


            : sourceStream (&source, false),
        uncompressedStreamLength (-1),
        format (zlibGZipDecompressorInputStreamFormat),
        originalSourcePos (source.getPosition()),
        buffer ((size_t) GZIPDecompressHelper::gzipDecompBufferSize),
        helper (new GZIPDecompressHelper (zlibGZipDecompressorInputStreamFormat))
        */
    }
    
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return uncompressedStreamLength;
        */
    }
    
    pub fn read(&mut self, 
        dest_buffer: *mut c_void,
        how_many:    i32) -> i32 {
        
        todo!();
        /*
            jassert (destBuffer != nullptr && howMany >= 0);

        if (howMany > 0 && ! isEof)
        {
            int numRead = 0;
            auto d = static_cast<uint8*> (destBuffer);

            while (! helper->error)
            {
                auto n = helper->doNextBlock (d, (unsigned int) howMany);
                currentPos += n;

                if (n == 0)
                {
                    if (helper->finished || helper->needsDictionary)
                    {
                        isEof = true;
                        return numRead;
                    }

                    if (helper->needsInput())
                    {
                        activeBufferSize = sourceStream->read (buffer, (int) GZIPDecompressHelper::gzipDecompBufferSize);

                        if (activeBufferSize > 0)
                        {
                            helper->setInput (buffer, (size_t) activeBufferSize);
                        }
                        else
                        {
                            isEof = true;
                            return numRead;
                        }
                    }
                }
                else
                {
                    numRead += n;
                    howMany -= n;
                    d += n;

                    if (howMany <= 0)
                        return numRead;
                }
            }
        }

        return 0;
        */
    }
    
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return helper->error || helper->finished || isEof;
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return currentPos;
        */
    }
    
    pub fn set_position(&mut self, new_pos: i64) -> bool {
        
        todo!();
        /*
            if (newPos < currentPos)
        {
            // to go backwards, reset the stream and start again..
            isEof = false;
            activeBufferSize = 0;
            currentPos = 0;
            helper.reset (new GZIPDecompressHelper (format));

            sourceStream->setPosition (originalSourcePos);
        }

        skipNextBytes (newPos - currentPos);
        return true;
        */
    }
}
