crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidContentUriOutputStream<W: Write> {
    base:   W,
    stream: GlobalRef,
    pos:    i64, // default = 0
}

#[cfg(target_os="android")]
impl<W: Write> Drop for AndroidContentUriOutputStream<W> {

    fn drop(&mut self) {
        todo!();
        /* 
            stream.callVoidMethod (AndroidOutputStream.close);
         */
    }
}

#[cfg(target_os="android")]
impl<W: Write> AndroidContentUriOutputStream<W> {

    pub fn new(output_stream: LocalRef<jobject>) -> Self {
    
        todo!();
        /*
        : stream(outputStream),

        
        */
    }
    
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            stream.callVoidMethod (AndroidOutputStream.flush);
        */
    }
    
    pub fn set_position(&mut self, new_pos: i64) -> bool {
        
        todo!();
        /*
            return (newPos == pos);
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return pos;
        */
    }
    
    pub fn write(&mut self, 
        data_to_write:   *const c_void,
        number_of_bytes: usize) -> bool {
        
        todo!();
        /*
            if (numberOfBytes == 0)
                return true;

            JNIEnv* env = getEnv();

            jbyteArray javaArray = env->NewByteArray ((jsize) numberOfBytes);
            env->SetByteArrayRegion (javaArray, 0, (jsize) numberOfBytes, (const jbyte*) dataToWrite);

            stream.callVoidMethod (AndroidOutputStream.write, javaArray, 0, (jint) numberOfBytes);
            env->DeleteLocalRef (javaArray);

            pos += static_cast<int64> (numberOfBytes);
            return true;
        */
    }
}
