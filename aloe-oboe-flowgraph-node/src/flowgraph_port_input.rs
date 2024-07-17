crate::ix!();

/**
  | An input port for streaming audio data.
  | 
  | You can set a value that will be used for
  | processing.
  | 
  | If you connect an output port to this
  | port then its value will be used instead.
  |
  */
pub struct FlowGraphPortFloatInput<'a> {
    base:      FlowGraphPortFloat<'a>,
    connected: *mut FlowGraphPortFloatOutput<'a>, // default = nullptr
}

impl<'a> FlowGraphPortFloatInput<'a> {

    pub fn new(
        parent:            &mut FlowGraphNode,
        samples_per_frame: i32) -> Self {
    
        todo!();
        /*
        : flow_graph_port_float(parent, samplesPerFrame),

            // Add to parent so it can pull data from each input.
            parent.addInputPort(*this);
        */
    }

    /**
      | Write every value of the float buffer.
      | 
      | This value will be ignored if an output
      | port is connected to this port.
      |
      */
    pub fn set_value(&mut self, value: f32)  {
        
        todo!();
        /*
            int numFloats = kDefaultBufferSize * getSamplesPerFrame();
            float *buffer = getBuffer();
            for (int i = 0; i < numFloats; i++) {
                *buffer++ = value;
            }
        */
    }

    /**
      | Connect to the output of another module.
      | 
      | An input port can only have one connection.
      | 
      | An output port can have multiple connections.
      | 
      | This not thread safe.
      |
      */
    pub fn connect(&mut self, port: *mut FlowGraphPortFloatOutput)  {
        
        todo!();
        /*
            assert(getSamplesPerFrame() == port->getSamplesPerFrame());
            mConnected = port;
        */
    }
    
    pub fn disconnect_port(&mut self, port: *mut FlowGraphPortFloatOutput)  {
        
        todo!();
        /*
            assert(mConnected == port);
            (void) port;
            mConnected = nullptr;
        */
    }
    
    pub fn disconnect(&mut self)  {
        
        todo!();
        /*
            mConnected = nullptr;
        */
    }

    /**
      | Pull data from any output port that is
      | connected.
      |
      */
    pub fn pull_data(&mut self, 
        call_count: i64,
        num_frames: i32) -> i32 {
        
        todo!();
        /*
            return (mConnected == nullptr)
                ? std::min(getFramesPerBuffer(), numFrames)
                : mConnected->pullData(callCount, numFrames);
        */
    }
    
    pub fn pull_reset(&mut self)  {
        
        todo!();
        /*
            if (mConnected != nullptr) mConnected->pullReset();
        */
    }
    
    /**
      | If connected to an output port then this
      | will return that output ports buffers.
      | 
      | If not connected then it returns the
      | input ports own buffer which can be loaded
      | using setValue().
      |
      */
    pub fn get_buffer(&mut self) -> *mut f32 {
        
        todo!();
        /*
            if (mConnected == nullptr) {
            return FlowGraphPortFloat::getBuffer(); // loaded using setValue()
        } else {
            return mConnected->getBuffer();
        }
        */
    }
}
