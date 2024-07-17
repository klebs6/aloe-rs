crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/broadcasters/aloe_ChangeBroadcaster.h]
pub struct ChangeBroadcasterCallback<'a> {
    base:  AsyncUpdater<'a>,
    owner: *mut ChangeBroadcaster<'a>,
}

impl<'a> Default for ChangeBroadcasterCallback<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        : owner(nullptr),
        */
    }
}

impl<'a> ChangeBroadcasterCallback<'a> {
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            jassert (owner != nullptr);
        owner->callListeners();
        */
    }
}

/**
  | Holds a list of ChangeListeners, and
  | sends messages to them when instructed.
  | 
  | @see ChangeListener
  | 
  | @tags{Events}
  |
  */
#[no_copy]
pub struct ChangeBroadcaster<'a> {
    broadcast_callback: ChangeBroadcasterCallback<'a>,
    change_listeners:   ListenerList<Box<dyn ChangeListener>>,
    any_listeners:      AtomicBool, // { false };
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/broadcasters/aloe_ChangeBroadcaster.cpp]
impl<'a> Default for ChangeBroadcaster<'a> {

    fn default() -> Self {
    
        todo!();
        /*
            broadcastCallback.owner = this;
        */
    }
}

impl<'a> AddChangeListener for ChangeBroadcaster<'a> {

    /**
      | Registers a listener to receive change
      | callbacks from this broadcaster.
      | 
      | Trying to add a listener that's already
      | on the list will have no effect.
      |
      */
    fn add_change_listener(&mut self, listener: *mut dyn ChangeListener)  {
        
        todo!();
        /*
            // Listeners can only be safely added when the event thread is locked
        // You can  use a MessageManagerLock if you need to call this from another thread.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        changeListeners.add (listener);
        anyListeners = true;
        */
    }
}
    
impl<'a> RemoveChangeListener for ChangeBroadcaster<'a> {

    /**
      | Unregisters a listener from the list.
      | 
      | If the listener isn't on the list, this
      | won't have any effect.
      |
      */
    fn remove_change_listener(&mut self, listener: *mut dyn ChangeListener)  {
        
        todo!();
        /*
            // Listeners can only be safely removed when the event thread is locked
        // You can  use a MessageManagerLock if you need to call this from another thread.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        changeListeners.remove (listener);
        anyListeners = changeListeners.size() > 0;
        */
    }
}
    
impl<'a> RemoveAllChangeListeners for ChangeBroadcaster<'a> {

    /**
      | Removes all listeners from the list.
      |
      */
    fn remove_all_change_listeners(&mut self)  {
        
        todo!();
        /*
            // Listeners can only be safely removed when the event thread is locked
        // You can  use a MessageManagerLock if you need to call this from another thread.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        changeListeners.clear();
        anyListeners = false;
        */
    }
}

impl<'a> SendChangeMessage for ChangeBroadcaster<'a> {

    /**
      | Causes an asynchronous change message
      | to be sent to all the registered listeners.
      | 
      | The message will be delivered asynchronously
      | by the main message thread, so this method
      | will return immediately. To call the
      | listeners synchronously use sendSynchronousChangeMessage().
      |
      */
    fn send_change_message(&mut self)  {
        
        todo!();
        /*
            if (anyListeners)
            broadcastCallback.triggerAsyncUpdate();
        */
    }
}

impl<'a> SendSynchronousChangeMessage for ChangeBroadcaster<'a> {

    /**
      | Sends a synchronous change message
      | to all the registered listeners.
      | 
      | This will immediately call all the listeners
      | that are registered. For thread-safety
      | reasons, you must only call this method
      | on the main message thread.
      | 
      | @see dispatchPendingMessages
      |
      */
    fn send_synchronous_change_message(&mut self)  {
        
        todo!();
        /*
            // This can only be called by the event thread.
        ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

        broadcastCallback.cancelPendingUpdate();
        callListeners();
        */
    }
}

impl<'a> DispatchPendingMessages for ChangeBroadcaster<'a> {

    /**
      | If a change message has been sent but
      | not yet dispatched, this will call sendSynchronousChangeMessage()
      | to make the callback immediately.
      | 
      | For thread-safety reasons, you must
      | only call this method on the main message
      | thread.
      |
      */
    fn dispatch_pending_messages(&mut self)  {
        
        todo!();
        /*
            broadcastCallback.handleUpdateNowIfNeeded();
        */
    }
}

impl<'a> CallListeners for ChangeBroadcaster<'a> {

    fn call_listeners(&mut self)  {
        
        todo!();
        /*
            changeListeners.call ([this] (ChangeListener& l) { l.changeListenerCallback (this); });
        */
    }
}
