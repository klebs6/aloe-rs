crate::ix!();

/**
  | Each node in the graph has a UID of this
  | type.
  |
  */
#[derive(Default)]
pub struct AudioProcessorGraphNodeID
{
    uid: u32, // default = 0
}

impl PartialEq<AudioProcessorGraphNodeID> for AudioProcessorGraphNodeID {
    
    #[inline] fn eq(&self, other: &AudioProcessorGraphNodeID) -> bool {
        todo!();
        /*
            return uid == other.uid;
        */
    }
}

impl Eq for AudioProcessorGraphNodeID {}

impl Ord for AudioProcessorGraphNodeID {
    
    #[inline] fn cmp(&self, other: &AudioProcessorGraphNodeID) -> std::cmp::Ordering {
        todo!();
        /*
            return uid <  other.uid;
        */
    }
}

impl PartialOrd<AudioProcessorGraphNodeID> for AudioProcessorGraphNodeID {
    #[inline] fn partial_cmp(&self, other: &AudioProcessorGraphNodeID) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl AudioProcessorGraphNodeID {
    
    pub fn new(i: u32) -> Self {
    
        todo!();
        /*
        : uid(i),

        
        */
    }
}
