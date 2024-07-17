crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/broadcasters/aloe_ActionBroadcaster.h]

#[no_copy]
pub struct ActionMessage {
    broadcaster: WeakReference<ActionBroadcaster>,
    message:     String,
    listener:    *const dyn ActionListener,
}

impl MessageBaseInterface for ActionMessage {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
            if (auto b = broadcaster.get())
                    if (b->actionListeners.contains (listener))
                        listener->actionListenerCallback (message);
        */
    }
}

impl ActionMessage {

    pub fn new(
        ab:           *const ActionBroadcaster,
        message_text: &String,
        l:            *mut dyn ActionListener) -> Self {
    
        todo!();
        /*


            : broadcaster (const_cast<ActionBroadcaster*> (ab)),
                message (messageText),
                listener (l)
        */
    }
}

/**
  | Manages a list of ActionListeners,
  | and can send them messages.
  | 
  | To quickly add methods to your class
  | that can add/remove action listeners
  | and broadcast to them, you can derive
  | from this.
  | 
  | @see ActionListener, ChangeListener
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[weak_referenceable]
pub struct ActionBroadcaster {
    action_listeners:     SortedSet<*mut dyn ActionListener>,
    action_listener_lock: CriticalSection,
}

impl Default for ActionBroadcaster {

    fn default() -> Self {
    
        todo!();
        /*
        // are you trying to create this object before or after aloe has been initialised??
        ALOE_ASSERT_MESSAGE_MANAGER_EXISTS
        */
    }
}

impl Drop for ActionBroadcaster {

    fn drop(&mut self) {
        todo!();
        /* 
        // all event-based objects must be deleted BEFORE aloe is shut down!
        ALOE_ASSERT_MESSAGE_MANAGER_EXISTS
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/broadcasters/aloe_ActionBroadcaster.cpp]

impl ActionBroadcaster {

    /**
      | Adds a listener to the list.
      | 
      | Trying to add a listener that's already
      | on the list will have no effect.
      |
      */
    pub fn add_action_listener(&mut self, listener: *mut dyn ActionListener)  {
        
        todo!();
        /*
            const ScopedLock sl (actionListenerLock);

        if (listener != nullptr)
            actionListeners.add (listener);
        */
    }

    /**
      | Removes a listener from the list.
      | 
      | If the listener isn't on the list, this
      | won't have any effect.
      |
      */
    pub fn remove_action_listener(&mut self, listener: *mut dyn ActionListener)  {
        
        todo!();
        /*
            const ScopedLock sl (actionListenerLock);
        actionListeners.removeValue (listener);
        */
    }

    /**
      | Removes all listeners from the list.
      |
      */
    pub fn remove_all_action_listeners(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (actionListenerLock);
        actionListeners.clear();
        */
    }

    /**
      | Broadcasts a message to all the registered
      | listeners. @see ActionListener::actionListenerCallback
      |
      */
    pub fn send_action_message(&self, message: &String)  {
        
        todo!();
        /*
            const ScopedLock sl (actionListenerLock);

        for (int i = actionListeners.size(); --i >= 0;)
            (new ActionMessage (this, message, actionListeners.getUnchecked(i)))->post();
        */
    }
}
