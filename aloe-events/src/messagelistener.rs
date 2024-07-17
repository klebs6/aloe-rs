crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_MessageListener.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_MessageListener.cpp]

impl Message {

    pub fn message_callback(&mut self)  {
        
        todo!();
        /*
            if (auto* r = recipient.get())
            r->handleMessage (*this);
        */
    }
}

/**
  | MessageListener subclasses can post
  | and receive Message objects.
  | 
  | @see Message, MessageManager, ActionListener,
  | ChangeListener
  | 
  | @tags{Events}
  |
  */
pub struct MessageListener {
    master_reference: WeakReferenceMaster<MessageListener>,
}

pub trait MessageListenerInterface {

    /**
      | This is the callback method that receives
      | incoming messages.
      | 
      | This is called by the MessageManager
      | from its dispatch loop.
      | 
      | @see postMessage
      |
      */
    fn handle_message(&mut self, message: &Message);
}

impl Drop for MessageListener {

    fn drop(&mut self) {
        todo!();
        /* 
        masterReference.clear();
        */
    }
}

impl Default for MessageListener {

    fn default() -> Self {
    
        todo!();
        /*
        // Are you trying to create a messagelistener before or after aloe has been initialised??
        ALOE_ASSERT_MESSAGE_MANAGER_EXISTS
        */
    }
}

impl MessageListener {
    
    /**
      | Sends a message to the message queue,
      | for asynchronous delivery to this listener
      | later on.
      | 
      | This method can be called safely by any
      | thread.
      | 
      | -----------
      | @param message
      | 
      | the message object to send - this will
      | be deleted automatically by the message
      | queue, so make sure it's allocated on
      | the heap, not the stack! @see handleMessage
      |
      */
    fn post_message(&self, message: *mut Message)  {
        
        todo!();
        /*
            message->recipient = const_cast<MessageListener*> (this);
        message->post();
        */
    }
}
