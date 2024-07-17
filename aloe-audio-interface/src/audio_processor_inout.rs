crate::ix!();

#[derive(Default)]
pub struct AudioProcessorInOutChannelPair {
    in_channels:  i16, // default = 0
    out_channels: i16, // default = 0
}

impl PartialEq<AudioProcessorInOutChannelPair> for AudioProcessorInOutChannelPair {
    
    #[inline] fn eq(&self, other: &AudioProcessorInOutChannelPair) -> bool {
        todo!();
        /*
            return other.inChannels == inChannels && other.outChannels == outChannels;
        */
    }
}

impl Eq for AudioProcessorInOutChannelPair {}

impl AudioProcessorInOutChannelPair {

    pub fn new(
        in_ch:  i16,
        out_ch: i16) -> Self {
    
        todo!();
        /*
        : in_channels(inCh),
        : out_channels(outCh),

        
        */
    }
    
    pub fn new_from_channel_config(config: &[i16; 2]) -> Self {
    
        todo!();
        /*

            : inChannels (config[0]), outChannels (config[1])
        */
    }
}
