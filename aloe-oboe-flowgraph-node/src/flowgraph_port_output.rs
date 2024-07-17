crate::ix!();

/**
  | The results of a node's processing are
  | stored in the buffers of the output ports.
  |
  */
pub struct FlowGraphPortFloatOutput<'a> {
    base: FlowGraphPortFloat<'a>,
}

impl<'a> FlowGraphPortFloatOutput<'a> {

    pub fn new(
        parent:            &mut FlowGraphNode<'a>,
        samples_per_frame: i32) -> Self {
    
        todo!();
        /*
        : flow_graph_port_float(parent, samplesPerFrame),
        */
    }

    /**
      | Call the parent module's onProcess()
      | method.
      | 
      | That may pull data from its inputs and
      | recursively process the entire graph.
      | 
      | -----------
      | @return
      | 
      | number of frames actually pulled
      |
      */
    pub fn pull_data(&mut self, 
        call_count: i64,
        num_frames: i32) -> i32 {
        
        todo!();
        /*
            numFrames = std::min(getFramesPerBuffer(), numFrames);
        return mContainingNode.pullData(numFrames, callCount);
        */
    }
    
    pub fn pull_reset(&mut self)  {
        
        todo!();
        /*
            mContainingNode.pullReset();
        */
    }

    /**
      | Connect to the input of another module.
      | 
      | An input port can only have one connection.
      | 
      | An output port can have multiple connections.
      | 
      | If you connect a second output port to
      | an input port then it overwrites the
      | previous connection.
      | 
      | This not thread safe. Do not modify the
      | graph topology from another thread
      | while running.
      | 
      | Also do not delete a module while it is
      | connected to another port if the graph
      | is running.
      |
      */
    pub fn connect(&mut self, port: *mut FlowGraphPortFloatInput)  {
        
        todo!();
        /*
            port->connect(this);
        */
    }
    
    /**
      | Disconnect from the input of another
      | module.
      | 
      | This not thread safe.
      |
      */
    pub fn disconnect(&mut self, port: *mut FlowGraphPortFloatInput)  {
        
        todo!();
        /*
            port->disconnect(this);
        */
    }
}
