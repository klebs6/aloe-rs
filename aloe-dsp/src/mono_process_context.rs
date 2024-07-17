crate::ix!();

pub struct MonoProcessContext<ProcessContext> {
    base:    ProcessContext,
    channel: usize,
}

impl<ProcessContext: HasAudioBlockType> MonoProcessContext<ProcessContext> {

    pub fn new(
        multi_channel_context: &ProcessContext,
        channel_to_use:        usize) -> Self {
    
        todo!();
        /*
        : process_context(multiChannelContext),
        : channel(channelToUse),

        
        */
    }
    
    pub fn get_input_block(&self) -> <ProcessContext as HasAudioBlockType>::AudioBlockType {
        
        todo!();
        /*
            return ProcessContext::getInputBlock() .getSingleChannelBlock (channel);
        */
    }
    
    pub fn get_output_block(&self) -> <ProcessContext as HasAudioBlockType>::AudioBlockType {
        
        todo!();
        /*
            return ProcessContext::getOutputBlock().getSingleChannelBlock (channel);
        */
    }
}
