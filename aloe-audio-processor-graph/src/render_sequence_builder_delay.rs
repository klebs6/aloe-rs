crate::ix!();

pub type RenderSequenceBuilderAudioProcessorGraphNode<'a>   
= AudioProcessorGraphNode<'a>;

pub type RenderSequenceBuilderAudioProcessorGraphNodeID 
= AudioProcessorGraphNodeID;

pub const RENDER_SEQUENCE_BUILDER_READ_ONLY_EMPTY_BUFFER_INDEX: usize = 0;

pub struct RenderSequenceBuilderDelay
{
    nodeid: RenderSequenceBuilderAudioProcessorGraphNodeID,
    delay:  i32,
}
