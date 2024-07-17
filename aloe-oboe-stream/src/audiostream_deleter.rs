crate::ix!();

/**
  | This struct is a stateless functor which
  | closes an AudioStream prior to its deletion.
  | 
  | This means it can be used to safely delete
  | a smart pointer referring to an open
  | stream.
  |
  */
pub struct StreamDeleterFunctor {

}

impl StreamDeleterFunctor {

    pub fn invoke(&mut self, audio_stream: *mut AudioStream)  {
        
        todo!();
        /*
            if (audioStream) {
                    audioStream->close();
                }
                delete audioStream;
        */
    }
}
