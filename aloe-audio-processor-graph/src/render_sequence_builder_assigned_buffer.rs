crate::ix!();

pub struct RenderSequenceBuilderAssignedBuffer {
    channel: AudioProcessorGraphNodeAndChannel,
}

impl RenderSequenceBuilderAssignedBuffer {

    pub fn create_read_only_empty() -> RenderSequenceBuilderAssignedBuffer {
        
        todo!();
        /*
            return { { zeroNodeID(), 0 } };
        */
    }
    
    pub fn create_free() -> RenderSequenceBuilderAssignedBuffer {
        
        todo!();
        /*
            return { { freeNodeID(), 0 } };
        */
    }
    
    pub fn is_read_only_empty(&self) -> bool {
        
        todo!();
        /*
            return channel.nodeID == zeroNodeID();
        */
    }
    
    pub fn is_free(&self) -> bool {
        
        todo!();
        /*
            return channel.nodeID == freeNodeID();
        */
    }
    
    pub fn is_assigned(&self) -> bool {
        
        todo!();
        /*
            return ! (isReadOnlyEmpty() || isFree());
        */
    }
    
    pub fn set_free(&mut self)  {
        
        todo!();
        /*
            channel = { freeNodeID(), 0 };
        */
    }
    
    pub fn set_assigned_to_non_existent_node(&mut self)  {
        
        todo!();
        /*
            channel = { anonNodeID(), 0 };
        */
    }
    
    pub fn anon_nodeid() -> RenderSequenceBuilderAudioProcessorGraphNodeID {
        
        todo!();
        /*
            return RenderSequenceBuilderAudioProcessorGraphNodeID (0x7ffffffd);
        */
    }
    
    pub fn zero_nodeid() -> RenderSequenceBuilderAudioProcessorGraphNodeID {
        
        todo!();
        /*
            return RenderSequenceBuilderAudioProcessorGraphNodeID (0x7ffffffe);
        */
    }
    
    pub fn free_nodeid() -> RenderSequenceBuilderAudioProcessorGraphNodeID {
        
        todo!();
        /*
            return RenderSequenceBuilderAudioProcessorGraphNodeID (0x7fffffff);
        */
    }
}
