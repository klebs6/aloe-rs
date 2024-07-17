crate::ix!();

pub trait FlowGraphSinkInterface {

    fn read(
        &mut self, 
        data:       *mut c_void,
        num_frames: i32

    ) -> i32;
}

/**
  | Base class for an edge node in a graph
  | that has no downstream nodes.
  | 
  | It consumes data but does not output
  | data.
  | 
  | This graph will be executed when data
  | is read() from this node by pulling data
  | from upstream nodes.
  |
  */
pub struct FlowGraphSink<'a> {
    base:  FlowGraphNode<'a>,
    input: FlowGraphPortFloatInput<'a>,
}

impl<'a> FlowGraphSink<'a> {

    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
            : input(*this, channelCount)
        */
    }

    /**
      | Dummy processor. The work happens in
      | the read() method.
      | 
      | -----------
      | @param numFrames
      | 
      | @return number of frames actually processed
      |
      */
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            return numFrames;
        */
    }

    /**
      | Pull data through the graph using this
      | nodes last callCount.
      | 
      | -----------
      | @param numFrames
      | 
      | @return
      |
      */
    pub fn pull_data(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            return FlowGraphNode::pullData(numFrames, getLastCallCount() + 1);
        */
    }
}
