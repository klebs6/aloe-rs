crate::ix!();

/**
  | This port contains a 32-bit float buffer
  | that can contain several frames of data.
  | 
  | Processing the data in a block improves
  | performance.
  | 
  | The size is framesPerBuffer * samplesPerFrame).
  |
  */
pub struct FlowGraphPortFloat<'a> {

    base:              FlowGraphPort<'a>,

    frames_per_buffer: i32, // default = 1

    /**
      | allocated in constructor
      |
      */
    buffer:            Box<&'a [f32]>,
}

impl<'a> FlowGraphPortFloat<'a> {
    
    pub fn get_frames_per_buffer(&self) -> i32 {
        
        todo!();
        /*
            return mFramesPerBuffer;
        */
    }

    /**
      | @return
      | 
      | buffer internal to the port or from a
      | connected port
      |
      */
    pub fn get_buffer(&mut self) -> *mut f32 {
        
        todo!();
        /*
            return mBuffer.get();
        */
    }

    pub fn new(
        parent:            &mut FlowGraphNode,
        samples_per_frame: i32,
        frames_per_buffer: Option<i32>

    ) -> Self {

        let frames_per_buffer: i32 =
            frames_per_buffer.unwrap_or(kDefaultBufferSize);
    
        todo!();
        /*
        : flow_graph_port(parent, samplesPerFrame),
        : frames_per_buffer(framesPerBuffer),
        : buffer(nullptr),

            size_t numFloats = static_cast<size_t>(framesPerBuffer * getSamplesPerFrame());
        mBuffer = std::make_unique<float[]>(numFloats);
        */
    }
}
