crate::ix!();

pub trait GetCallbackLock {

    /**
      | This returns a critical section that
      | will automatically be locked while
      | the host is calling the processBlock()
      | method.
      | 
      | Use it from your UI or other threads to
      | lock access to variables that are used
      | by the process callback, but obviously
      | be careful not to keep it locked for too
      | long, because that could cause stuttering
      | playback. If you need to do something
      | that'll take a long time and need the
      | processing to stop while it happens,
      | use the suspendProcessing() method
      | instead.
      | 
      | @see suspendProcessing
      |
      */
    fn get_callback_lock(&self) -> &CriticalSection;
}
