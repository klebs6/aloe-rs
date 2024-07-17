/*!
  | FlowGraph.h
  | 
  | Processing node and ports that can be
  | used in a simple data flow graph.
  | 
  | This was designed to work with audio
  | but could be used for other types of data.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowGraphNode.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowGraphNode.cpp]

/*
  | TODO Move these classes into separate files.
  |
  | TODO Review use of raw pointers for
  |      connect(). Maybe use smart pointers but
  |      need to avoid run-time deallocation in
  |      audio thread.
  */

/**
  | Set this to 1 if using it inside the Android
  | framework.
  |
  | This code is kept here so that it can be moved
  | easily between Oboe and AAudio.
  */
#[cfg(not(FLOWGRAPH_ANDROID_INTERNAL))]
pub const FLOWGRAPH_ANDROID_INTERNAL: usize = 0;

/**
  | Set this to a name that will prevent AAudio
  | from calling into Oboe.
  |
  | AAudio and Oboe both use a version of this
  | flowgraph package.
  |
  | There was a problem in the unit tests where
  | AAudio would call a constructor in AAudio and
  | then call a destructor in Oboe! That caused
  | memory corruption.
  |
  | For more details, see Issue #930.
  */
#[cfg(not(FLOWGRAPH_OUTER_NAMESPACE))]
macro_rules! flowgraph_outer_namespace {
    () => {
        /*
                oboe
        */
    }
}

/**
  | Default block size that can be overridden when
  | the FlowGraphPortFloat is created.
  |
  | If it is too small then we will have too much
  | overhead from switching between nodes.
  |
  | If it is too high then we will thrash the
  | caches.
  */
pub const kDefaultBufferSize: i32 = 8; // arbitrary

pub const kInitialCallCount: i64 = -1;

pub trait FlowgraphNodeInterface {

    /**
      | Read from the input ports, generate
      | multiple frames of data then write the
      | results to the output ports.
      | 
      | -----------
      | @param numFrames
      | 
      | maximum number of frames requested
      | for processing @return number of frames
      | actually processed
      |
      */
    fn on_process(&mut self, num_frames: i32) -> i32;

}

/**
  | Base class for all nodes in the flowgraph.
  |
  */
pub struct FlowGraphNode<'a> {
    last_call_count:           i64, // default = kInitialCallCount
    input_ports:               Vec<Arc<FlowGraphPort<'a>>>,
    data_pulled_automatically: bool, // default = true
    block_recursion:           bool, // default = false
    last_frame_count:          i32, // default = 0
}

impl<'a> FlowGraphNode<'a> {

    pub fn add_input_port(&mut self, port: &mut FlowGraphPort)  {
        
        todo!();
        /*
            mInputPorts.push_back(port);
        */
    }
    
    pub fn is_data_pulled_automatically(&self) -> bool {
        
        todo!();
        /*
            return mDataPulledAutomatically;
        */
    }

    /**
      | Set true if you want the data pulled through
      | the graph automatically.
      | 
      | This is the default.
      | 
      | Set false if you want to pull the data
      | from the input ports in the onProcess()
      | method.
      | 
      | You might do this, for example, in a sample
      | rate converting node.
      | 
      | @param automatic
      |
      */
    pub fn set_data_pulled_automatically(&mut self, automatic: bool)  {
        
        todo!();
        /*
            mDataPulledAutomatically = automatic;
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowGraph";
        */
    }
    
    pub fn get_last_call_count(&mut self) -> i64 {
        
        todo!();
        /*
            return mLastCallCount;
        */
    }

    /**
      | If the callCount is at or after the previous
      | callCount then call pullData on all
      | of the upstreamNodes.
      | 
      | Then call onProcess().
      | 
      | This prevents infinite recursion in
      | case of cyclic graphs.
      | 
      | It also prevents nodes upstream from
      | a branch from being executed twice.
      | 
      | -----------
      | @param callCount
      | 
      | @param numFrames
      | 
      | -----------
      | @return
      | 
      | number of frames valid
      |
      */
    pub fn pull_data(&mut self, 
        num_frames: i32,
        call_count: i64) -> i32 {
        
        todo!();
        /*
            int32_t frameCount = numFrames;
        // Prevent recursion and multiple execution of nodes.
        if (callCount > mLastCallCount) {
            mLastCallCount = callCount;
            if (mDataPulledAutomatically) {
                // Pull from all the upstream nodes.
                for (auto &port : mInputPorts) {
                    // TODO fix bug of leaving unused data in some ports if using multiple AudioSource
                    frameCount = port.get().pullData(callCount, frameCount);
                }
            }
            if (frameCount > 0) {
                frameCount = onProcess(frameCount);
            }
            mLastFrameCount = frameCount;
        } else {
            frameCount = mLastFrameCount;
        }
        return frameCount;
        */
    }
    
    /**
      | Recursively reset all the nodes in the
      | graph, starting from a Sink.
      | 
      | This must not be called at the same time
      | as pullData!
      |
      */
    pub fn pull_reset(&mut self)  {
        
        todo!();
        /*
            if (!mBlockRecursion) {
            mBlockRecursion = true; // for cyclic graphs
            // Pull reset from all the upstream nodes.
            for (auto &port : mInputPorts) {
                port.get().pullReset();
            }
            mBlockRecursion = false;
            reset();
        }
        */
    }
    
    /**
      | Reset framePosition counters.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            mLastFrameCount = 0;
        mLastCallCount = kInitialCallCount;
        */
    }
}
