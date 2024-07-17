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
pub struct FlowGraphSourceBuffered<'a> {
    base:           FlowGraphSource<'a>,

    data:           *const c_void, // default = nullptr

    /**
      | number of frames in mData
      |
      */
    size_in_frames: i32, // default = 0

    /**
      | index of next frame to be processed
      |
      */
    frame_index:    i32, // default = 0
}

impl<'a> FlowGraphSourceBuffered<'a> {

    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
        : flow_graph_source(channelCount),

        
        */
    }

    /**
      | Specify buffer that the node will read
      | from.
      | 
      | -----------
      | @param data
      | 
      | TODO Consider using std::shared_ptr.
      | 
      | @param numFrames
      |
      */
    pub fn set_data(&mut self, 
        data:       *const c_void,
        num_frames: i32)  {
        
        todo!();
        /*
            mData = data;
            mSizeInFrames = numFrames;
            mFrameIndex = 0;
        */
    }
}
