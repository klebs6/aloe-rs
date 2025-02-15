crate::ix!();

pub type OptionalQueue = OptionalScopedPointer<ConvolutionMessageQueue>;

pub struct ConvolutionMessageQueueImpl {
    base: BackgroundMessageQueue,
}

/**
  | Used by the Convolution to dispatch
  | engine-update messages on a background
  | thread.
  | 
  | May be shared between multiple Convolution
  | instances.
  | 
  | @tags{DSP}
  |
  */
pub struct ConvolutionMessageQueue {
    impl_: Box<ConvolutionMessageQueueImpl>,
}

impl Default for ConvolutionMessageQueue {
    
    /**
      | Initialises the queue to a default size.
      | 
      | If your Convolution is updated very
      | frequently, or you are sharing this
      | queue between multiple Convolutions,
      | consider using the alternative constructor
      | taking an explicit size argument.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : convolution_message_queue(1000),
        */
    }
}

impl Drop for ConvolutionMessageQueue {

    fn drop(&mut self) {
        todo!();
        /* 
        impl->stopThread (-1);
 */
    }
}

impl ConvolutionMessageQueue {
    
    /**
      | Initialises the queue with the specified
      | number of entries.
      | 
      | In general, the number of required entries
      | scales with the number of Convolutions
      | sharing the same Queue, and the frequency
      | of updates to those Convolutions.
      |
      */
    pub fn new_with_len(entries: i32) -> Self {
    
        todo!();
        /*


            : impl (std::make_unique<Impl> (entries))
        impl->startThread();
        */
    }
}
