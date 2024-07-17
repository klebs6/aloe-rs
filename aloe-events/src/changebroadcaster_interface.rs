crate::ix!();

pub trait ChangeBroadcasterInterface 
: AddChangeListener
+ RemoveChangeListener
+ RemoveAllChangeListeners
+ SendChangeMessage
+ SendSynchronousChangeMessage
+ DispatchPendingMessages
+ CallListeners
{ }

pub trait AddChangeListener {

    /**
      | Registers a listener to receive change
      | callbacks from this broadcaster.
      | 
      | Trying to add a listener that's already
      | on the list will have no effect.
      |
      */
    fn add_change_listener(&mut self, listener: *mut dyn ChangeListener);
}

pub trait RemoveChangeListener {

    /**
      | Unregisters a listener from the list.
      | 
      | If the listener isn't on the list, this
      | won't have any effect.
      |
      */
    fn remove_change_listener(&mut self, listener: *mut dyn ChangeListener);
}

pub trait RemoveAllChangeListeners {

    /**
      | Removes all listeners from the list.
      |
      */
    fn remove_all_change_listeners(&mut self);
}

pub trait SendChangeMessage {

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
    fn send_change_message(&mut self);
}

pub trait SendSynchronousChangeMessage {

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
    fn send_synchronous_change_message(&mut self);
}

pub trait DispatchPendingMessages {

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
    fn dispatch_pending_messages(&mut self);
}

pub trait CallListeners {

    fn call_listeners(&mut self);
}
