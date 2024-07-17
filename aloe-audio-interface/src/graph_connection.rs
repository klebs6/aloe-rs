crate::ix!();

/**
  | Represents a connection between two
  | channels of two nodes in an AudioProcessorGraph.
  | 
  | To create a connection, use AudioProcessorGraph::addConnection().
  |
  */
#[derive(Default)]
pub struct AudioProcessorGraphConnection {

    /**
      | The channel and node which is the input
      | source for this connection.
      |
      */
    source:      AudioProcessorGraphNodeAndChannel,

    /**
      | The channel and node which is the input
      | source for this connection.
      |
      */
    destination: AudioProcessorGraphNodeAndChannel,
}

impl PartialEq<AudioProcessorGraphConnection> for AudioProcessorGraphConnection {
    
    #[inline] fn eq(&self, other: &AudioProcessorGraphConnection) -> bool {
        todo!();
        /*
            return source == other.source && destination == other.destination;
        */
    }
}

impl Eq for AudioProcessorGraphConnection {}

impl Ord for AudioProcessorGraphConnection {
    
    #[inline] fn cmp(&self, other: &AudioProcessorGraphConnection) -> std::cmp::Ordering {
        todo!();
        /*
            if (source.nodeID != other.source.nodeID)
            return source.nodeID < other.source.nodeID;

        if (destination.nodeID != other.destination.nodeID)
            return destination.nodeID < other.destination.nodeID;

        if (source.channelIndex != other.source.channelIndex)
            return source.channelIndex < other.source.channelIndex;

        return destination.channelIndex < other.destination.channelIndex;
        */
    }
}

impl PartialOrd<AudioProcessorGraphConnection> for AudioProcessorGraphConnection {
    #[inline] fn partial_cmp(&self, other: &AudioProcessorGraphConnection) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl AudioProcessorGraphConnection {

    pub fn new(
        src: AudioProcessorGraphNodeAndChannel,
        dst: AudioProcessorGraphNodeAndChannel) -> Self {
    
        todo!();
        /*
        : source(src),
        : destination(dst),

        
        */
    }
}

