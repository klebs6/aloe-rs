crate::ix!();

pub trait SetPlayHead {

    /**
      | Tells the processor to use this playhead
      | object.
      | 
      | The processor will not take ownership
      | of the object, so the caller must delete
      | it when it is no longer being used.
      |
      */
    fn set_play_head(&mut self, new_play_head: *mut dyn AudioPlayHeadInterface);
}

pub trait GetPlayHead {

    /**
      | Returns the current AudioPlayHead
      | object that should be used to find out
      | the state and position of the playhead.
      | 
      | You can ONLY call this from your processBlock()
      | method! Calling it at other times will
      | produce undefined behaviour, as the
      | host may not have any context in which
      | a time would make sense, and some hosts
      | will almost certainly have multithreading
      | issues if it's not called on the audio
      | thread.
      | 
      | The AudioPlayHead object that is returned
      | can be used to get the details about the
      | time of the start of the block currently
      | being processed. But do not store this
      | pointer or use it outside of the current
      | audio callback, because the host may
      | delete or re-use it.
      | 
      | If the host can't or won't provide any
      | time info, this will return nullptr.
      |
      */
    fn get_play_head(&self) -> *mut dyn AudioPlayHeadInterface;
}


