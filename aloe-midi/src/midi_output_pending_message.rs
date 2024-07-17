crate::ix!();

pub struct MidiOutputPendingMessage
{
    message: MidiMessage,
    next:    *mut MidiOutputPendingMessage,
}

impl MidiOutputPendingMessage {
    
    pub fn new(
        data:       *const c_void,
        len:        i32,
        time_stamp: f64) -> Self {
    
        todo!();
        /*
        : message(data, len, timeStamp),
        */
    }
}
