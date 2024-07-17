crate::ix!();

#[cfg(target_os="android")]
pub struct StreamCloser {
    stream: GlobalRef,
}

#[cfg(target_os="android")]
impl Drop for StreamCloser {

    fn drop(&mut self) {
        todo!();
        /* 
                if (stream.get() != nullptr)
                    getEnv()->CallVoidMethod (stream, JavaCloseable.close);
             */
    }
}

#[cfg(target_os="android")]
impl StreamCloser {

    pub fn new(stream_to_use: &LocalRef<jobject>) -> Self {
    
        todo!();
        /*
        : stream(GlobalRef (streamToUse)),

        
        */
    }
}
