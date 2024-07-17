crate::ix!();

/**
  | Represents an input or output channel
  | of a node in an AudioProcessorGraph.
  |
  */
pub struct AudioProcessorGraphNodeAndChannel {
    nodeid:        AudioProcessorGraphNodeID,
    channel_index: i32,
}

impl Default for AudioProcessorGraphNodeAndChannel {

    fn default() -> Self {
        todo!();
    }
}

impl PartialEq<AudioProcessorGraphNodeAndChannel> for AudioProcessorGraphNodeAndChannel {
    
    #[inline] fn eq(&self, other: &AudioProcessorGraphNodeAndChannel) -> bool {
        todo!();
        /*
            return nodeID == other.nodeID && channelIndex == other.channelIndex;
        */
    }
}

impl Eq for AudioProcessorGraphNodeAndChannel {}

impl AudioProcessorGraphNodeAndChannel {

    pub fn ismidi(&self) -> bool {
        
        todo!();
        /*
            return channelIndex == midiChannelIndex;
        */
    }
}
