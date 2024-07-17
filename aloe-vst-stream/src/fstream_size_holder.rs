crate::ix!();

/**
  | FStreamSizeHolder Declaration remembers
  | size of stream chunk for backward compatibility.
  | 
  | <b>Example:</b>
  | 
  | -----------
  | @code
  | 
  | externalize (a)
  | {
  |     FStreamSizeHolder sizeHolder;
  |     sizeHolder.beginWrite ();   // sets start mark, writes dummy size
  |     a << ....
  |     sizeHolder.endWrite ();     // jumps to start mark, updates size, jumps back here
  | }
  | 
  | internalize (a)
  | {
  |     FStreamSizeHolder sizeHolder;
  |     sizeHolder.beginRead ();    // reads size, mark
  |     a >> ....
  |     sizeHolder.endRead ();      // jumps forward if new version has larger size
  | }
  |
  */
pub struct FStreamSizeHolder<'a> {
    stream:   &'a mut FStreamer,
    size_pos: i64,
}

impl<'a> FStreamSizeHolder<'a> {

    pub fn new(s: &mut FStreamer) -> Self {
    
        todo!();
        /*
        : stream(s),
        : size_pos(-1),
        */
    }
    
    /**
      | remembers position and writes 0
      |
      */
    pub fn begin_write(&mut self)  {
        
        todo!();
        /*
            sizePos = stream.tell ();
        stream.writeInt32 (0L);
        */
    }
    
    /**
      | writes and returns size (since the start
      | marker)
      |
      */
    pub fn end_write(&mut self) -> i32 {
        
        todo!();
        /*
            if (sizePos < 0)
            return 0;
        
        int64 currentPos = stream.tell ();

        stream.seek (sizePos, kSeekSet);
        int32 size = int32 (currentPos - sizePos - sizeof (int32));
        stream.writeInt32 (size);
        
        stream.seek (currentPos, kSeekSet);
        return size;
        */
    }
    
    /**
      | returns size
      |
      */
    pub fn begin_read(&mut self) -> i32 {
        
        todo!();
        /*
            sizePos = stream.tell ();
        int32 size = 0;
        stream.readInt32 (size);
        sizePos += size + sizeof (int32);
        return size;
        */
    }
    
    /**
      | jump to end of chunk
      |
      */
    pub fn end_read(&mut self)  {
        
        todo!();
        /*
            if (sizePos >= 0)
            stream.seek (sizePos, kSeekSet);
        */
    }
}
