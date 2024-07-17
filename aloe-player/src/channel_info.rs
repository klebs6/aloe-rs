crate::ix!();

pub struct ChannelInfo<Value> {
    data:         *mut *mut Value,
    num_channels: i32,
}

impl<Value> Default for ChannelInfo<Value> {

    fn default() -> Self {
        Self {
            data:         null_mut(),
            num_channels: 0,
        }
    }
}

impl<Value> ChannelInfo<Value> {
    
    pub fn new(
        data_in:         *mut *mut Value,
        num_channels_in: i32) -> Self {
    
        todo!();
        /*
        : data(dataIn),
        : num_channels(numChannelsIn),

        
        */
    }
}
