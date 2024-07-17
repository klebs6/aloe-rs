crate::ix!();

/**
  | Base class for a node that has an input
  | and an output with the same number of
  | channels.
  | 
  | This may include traditional filters,
  | eg. FIR, but also include any processing
  | node that converts input to output.
  |
  */
pub struct FlowGraphFilter<'a> {
    base:   FlowGraphNode<'a>,
    input:  FlowGraphPortFloatInput<'a>,
    output: FlowGraphPortFloatOutput<'a>,
}

impl<'a> FlowGraphFilter<'a> {

    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
            : input(*this, channelCount)
                , output(*this, channelCount)
        */
    }
}
