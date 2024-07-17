crate::ix!();

impl AUInputElementInterface for AUInputElement {

    /* -------------- AUElement override  -------------- */

    fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    fn needs_buffer_space(&self) -> bool {
        
        todo!();
        /*
            return IsCallback();
        */
    }
}
