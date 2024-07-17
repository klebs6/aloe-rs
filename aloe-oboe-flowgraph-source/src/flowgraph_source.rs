crate::ix!();

/**
  | Base class for an edge node in a graph
  | that has no upstream nodes.
  | 
  | It outputs data but does not consume
  | data.
  | 
  | By default, it will read its data from
  | an external buffer.
  |
  */
pub struct FlowGraphSource<'a> {
    base:   FlowGraphNode<'a>,
    output: FlowGraphPortFloatOutput<'a>,
}

impl<'a> FlowGraphSource<'a> {

    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
            : output(*this, channelCount)
        */
    }
}
