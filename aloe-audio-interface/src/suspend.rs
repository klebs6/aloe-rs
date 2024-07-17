crate::ix!();

pub trait SuspendProcessing {

    /**
      | Enables and disables the processing
      | callback.
      | 
      | If you need to do something time-consuming
      | on a thread and would like to make sure
      | the audio processing callback doesn't
      | happen until you've finished, use this
      | to disable the callback and re-enable
      | it again afterwards.
      | 
      | -----------
      | @code
      | 
      | void loadNewPatch()
      | {
      |     suspendProcessing (true);
      | 
      |     ..do something that takes ages..
      | 
      |     suspendProcessing (false);
      | }
      | 
      | If the host tries to make an audio callback
      | while processing is suspended, the
      | processor will return an empty buffer,
      | but won't block the audio thread like
      | it would do if you use the getCallbackLock()
      | critical section to synchronise access.
      | 
      | Any code that calls processBlock()
      | should call isSuspended() before doing
      | so, and if the processor is suspended,
      | it should avoid the call and emit silence
      | or whatever is appropriate.
      | 
      | @see getCallbackLock
      |
      */
    fn suspend_processing(&mut self, should_be_suspended: bool);
}

pub trait IsSuspended {

    /**
      | Returns true if processing is currently
      | suspended. @see suspendProcessing
      |
      */
    fn is_suspended(&self) -> bool;
}
