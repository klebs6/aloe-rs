crate::ix!();

pub fn calc_buffer_stream_buffer_size<R: Read>(
    requested_size: i32,
    source:         *mut R) -> i32 {
    
    todo!();
    /*
        // You need to supply a real stream when creating a BufferedInputStream
        jassert (source != nullptr);

        requestedSize = jmax (256, requestedSize);
        auto sourceSize = source->getTotalLength();

        if (sourceSize >= 0 && sourceSize < requestedSize)
            return jmax (32, (int) sourceSize);

        return requestedSize;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_BufferedInputStream.h]

/**
  | Wraps another input stream, and reads
  | from it using an intermediate buffer
  | 
  | If you're using an input stream such
  | as a file input stream, and making lots
  | of small read accesses to it, it's probably
  | sensible to wrap it in one of these, so
  | that the source stream gets accessed
  | in larger chunk sizes, meaning less
  | work for the underlying stream.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct BufferedInputStream<R: Read> {
    source:         OptionalScopedPointer<R>,
    buffer_size:    i32,
    position:       i64,
    last_read_pos:  i64, // default = 0
    buffer_start:   i64,
    buffer_overlap: i64, // default = 128
    buffer:         HeapBlock<u8>,
}

impl<R: Read> Read for BufferedInputStream<R> {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize,std::io::Error> {
        todo!();
    }
}

impl<R: Read> Drop for BufferedInputStream<R> {

    /**
      | Destructor.
      | 
      | This may also delete the source stream,
      | if that option was chosen when the buffered
      | stream was created.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*       */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_BufferedInputStream.cpp]
impl<R: Read> BufferedInputStream<R> {

    /**
      | Creates a BufferedInputStream from
      | an input source.
      | 
      | -----------
      | @param sourceStream
      | 
      | the source stream to read from
      | ----------
      | @param bufferSize
      | 
      | the size of reservoir to use to buffer
      | the source
      | ----------
      | @param deleteSourceWhenDestroyed
      | 
      | whether the sourceStream that is passed
      | in should be deleted by this object when
      | it is itself deleted.
      |
      */
    pub fn new_from_source_stream_and_size(
        source_stream:  *mut R,
        size:           i32,
        take_ownership: bool) -> Self {
    
        todo!();
        /*


            : source (sourceStream, takeOwnership),
         bufferSize (calcBufferStreamBufferSize (size, sourceStream)),
         position (sourceStream->getPosition()),
         bufferStart (position)

        buffer.malloc (bufferSize);
        */
    }
    
    /**
      | Creates a BufferedInputStream from
      | an input source.
      | 
      | -----------
      | @param sourceStream
      | 
      | the source stream to read from - the source
      | stream must not be deleted until this
      | object has been destroyed.
      | ----------
      | @param bufferSize
      | 
      | the size of reservoir to use to buffer
      | the source
      |
      */
    pub fn new_from_source_stream(
        source_stream: &mut R,
        size:          i32) -> Self {
    
        todo!();
        /*


            : BufferedInputStream (&sourceStream, size, false)
        */
    }

    /**
      | Returns the next byte that would be read
      | by a call to readByte()
      |
      */
    pub fn peek_byte(&mut self) -> u8 {
        
        todo!();
        /*
            if (! ensureBuffered())
            return 0;

        return position < lastReadPos ? buffer[(int) (position - bufferStart)] : 0;
        */
    }

    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return source->getTotalLength();
        */
    }

    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return position;
        */
    }

    pub fn set_position(&mut self, new_position: i64) -> bool {
        
        todo!();
        /*
            position = jmax ((int64) 0, newPosition);
        return true;
        */
    }

    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return position >= lastReadPos && source->isExhausted();
        */
    }

    pub fn ensure_buffered(&mut self) -> bool {
        
        todo!();
        /*
            auto bufferEndOverlap = lastReadPos - bufferOverlap;

        if (position < bufferStart || position >= bufferEndOverlap)
        {
            int bytesRead;

            if (position < lastReadPos
                 && position >= bufferEndOverlap
                 && position >= bufferStart)
            {
                auto bytesToKeep = (int) (lastReadPos - position);
                memmove (buffer, buffer + (int) (position - bufferStart), (size_t) bytesToKeep);

                bufferStart = position;
                bytesRead = source->read (buffer + bytesToKeep,
                                          (int) (bufferSize - bytesToKeep));

                if (bytesRead < 0)
                    return false;

                lastReadPos += bytesRead;
                bytesRead += bytesToKeep;
            }
            else
            {
                bufferStart = position;

                if (! source->setPosition (bufferStart))
                    return false;

                bytesRead = source->read (buffer, bufferSize);

                if (bytesRead < 0)
                    return false;

                lastReadPos = bufferStart + bytesRead;
            }

            while (bytesRead < bufferSize)
                buffer[bytesRead++] = 0;
        }

        return true;
        */
    }

    pub fn read(&mut self, 
        dest_buffer:       *mut c_void,
        max_bytes_to_read: i32) -> i32 {
        
        todo!();
        /*
            jassert (destBuffer != nullptr && maxBytesToRead >= 0);

        if (position >= bufferStart
             && position + maxBytesToRead <= lastReadPos)
        {
            memcpy (destBuffer, buffer + (int) (position - bufferStart), (size_t) maxBytesToRead);
            position += maxBytesToRead;
            return maxBytesToRead;
        }

        if (position < bufferStart || position >= lastReadPos)
            if (! ensureBuffered())
                return 0;

        int bytesRead = 0;

        while (maxBytesToRead > 0)
        {
            auto numToRead = jmin (maxBytesToRead, (int) (lastReadPos - position));

            if (numToRead > 0)
            {
                memcpy (destBuffer, buffer + (int) (position - bufferStart), (size_t) numToRead);
                maxBytesToRead -= numToRead;
                bytesRead += numToRead;
                position += numToRead;
                destBuffer = static_cast<char*> (destBuffer) + numToRead;
            }

            auto oldLastReadPos = lastReadPos;

            if (! ensureBuffered()
                 || oldLastReadPos == lastReadPos
                 || isExhausted())
                break;
        }

        return bytesRead;
        */
    }

    pub fn read_string(&mut self) -> String {
        
        todo!();
        /*
            if (position >= bufferStart
             && position < lastReadPos)
        {
            auto maxChars = (int) (lastReadPos - position);
            auto* src = buffer + (int) (position - bufferStart);

            for (int i = 0; i < maxChars; ++i)
            {
                if (src[i] == 0)
                {
                    position += i + 1;
                    return String::fromUTF8 (src, i);
                }
            }
        }

        return InputStream::readString();
        */
    }
}
