crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_SubregionStream.h]

/**
  | Wraps another input stream, and reads
  | from a specific part of it.
  | 
  | This lets you take a subsection of a stream
  | and present it as an entire stream in
  | its own right.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct SubregionStream<R: Read> {
    source:                          OptionalScopedPointer<R>,
    start_position_in_source_stream: i64,
    length_of_source_stream:         i64,
}

impl<R: Read> Read for SubregionStream<R> {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        todo!();
    }
}

impl<R: Read> Drop for SubregionStream<R> {

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

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_SubregionStream.cpp]
impl<R: Read> SubregionStream<R> {
    
    /**
      | Creates a SubregionStream from an input
      | source.
      | 
      | -----------
      | @param sourceStream
      | 
      | the source stream to read from
      | ----------
      | @param startPositionInSourceStream
      | 
      | this is the position in the source stream
      | that corresponds to position 0 in this
      | stream
      | ----------
      | @param lengthOfSourceStream
      | 
      | this specifies the maximum number of
      | bytes from the source stream that will
      | be passed through by this stream. When
      | the position of this stream exceeds
      | lengthOfSourceStream, it will cause
      | an end-of-stream. If the length passed
      | in here is greater than the length of
      | the source stream (as returned by getTotalLength()),
      | then the smaller value will be used.
      | Passing a negative value for this parameter
      | means it will keep reading until the
      | source's end-of-stream.
      | ----------
      | @param deleteSourceWhenDestroyed
      | 
      | whether the sourceStream that is passed
      | in should be deleted by this object when
      | it is itself deleted.
      |
      */
    pub fn new(
        source_stream:                *mut R,
        start:                        i64,
        length:                       i64,
        delete_source_when_destroyed: bool) -> Self {
    
        todo!();
        /*
        : source(sourceStream, deleteSourceWhenDestroyed),
        : start_position_in_source_stream(start),
        : length_of_source_stream(length),

            SubregionStream::setPosition (0);
        */
    }
    
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            auto srcLen = source->getTotalLength() - startPositionInSourceStream;

        return lengthOfSourceStream >= 0 ? jmin (lengthOfSourceStream, srcLen)
                                         : srcLen;
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return source->getPosition() - startPositionInSourceStream;
        */
    }
    
    pub fn set_position(&mut self, new_position: i64) -> bool {
        
        todo!();
        /*
            return source->setPosition (jmax ((int64) 0, newPosition + startPositionInSourceStream));
        */
    }
    
    pub fn read(&mut self, 
        dest_buffer:       *mut c_void,
        max_bytes_to_read: i32) -> i32 {
        
        todo!();
        /*
            jassert (destBuffer != nullptr && maxBytesToRead >= 0);

        if (lengthOfSourceStream < 0)
            return source->read (destBuffer, maxBytesToRead);

        maxBytesToRead = (int) jmin ((int64) maxBytesToRead, lengthOfSourceStream - getPosition());

        if (maxBytesToRead <= 0)
            return 0;

        return source->read (destBuffer, maxBytesToRead);
        */
    }
    
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            if (lengthOfSourceStream >= 0 && getPosition() >= lengthOfSourceStream)
            return true;

        return source->isExhausted();
        */
    }
}
