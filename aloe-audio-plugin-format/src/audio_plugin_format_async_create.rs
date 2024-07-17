crate::ix!();

pub struct AudioPluginFormatAsyncCreateMessage {
    base:            Message,
    desc:            PluginDescription,
    sample_rate:     f64,
    buffer_size:     i32,
    callback_to_use: PluginCreationCallback,
}

impl AudioPluginFormatAsyncCreateMessage {
    
    pub fn new(
        d:    &PluginDescription,
        sr:   f64,
        size: i32,
        call: PluginCreationCallback) -> Self {
    
        todo!();
        /*
        : desc(d),
        : sample_rate(sr),
        : buffer_size(size),
        : callback_to_use(std::move (call)),

        
        */
    }
}

