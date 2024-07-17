crate::ix!();

pub trait FlowGraphPortInterface {

    fn pull_data(&mut self, 
        frame_position: i64,
        num_frames:     i32) -> i32;

    fn pull_reset(&mut self);
}

/**
  | This is a connector that allows data
  | to flow between modules.
  | 
  | The ports are the primary means of interacting
  | with a module. So they are generally
  | declared as public.
  |
  */
pub struct FlowGraphPort<'a> {
    containing_node:   &'a mut FlowGraphNode<'a>,
    samples_per_frame: i32, // default = 1
}

impl<'a> FlowGraphPort<'a> {

    pub fn new(
        parent:            &mut FlowGraphNode,
        samples_per_frame: i32) -> Self {
    
        todo!();
        /*
        : containing_node(parent),
        : samples_per_frame(samplesPerFrame),
        */
    }
    
    pub fn get_samples_per_frame(&self) -> i32 {
        
        todo!();
        /*
            return mSamplesPerFrame;
        */
    }
}
