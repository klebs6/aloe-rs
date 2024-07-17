crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_Message.h]

/**
  | The base class for objects that can be
  | sent to a MessageListener.
  | 
  | If you want to send a message that carries
  | some kind of custom data, just create
  | a subclass of Message with some appropriate
  | member variables to hold your data.
  | 
  | Always create a new instance of a Message
  | object on the heap, as it will be deleted
  | automatically after the message has
  | been delivered.
  | 
  | @see MessageListener, MessageManager,
  | ActionListener, ChangeListener
  | 
  | @tags{Events}
  |
  */
#[no_copy]
pub struct Message {

    /*
      | Avoid the leak-detector because for
      | plugins, the host can unload our DLL with
      | undelivered messages still in the system
      | event queue. These aren't harmful, but can
      | cause annoying assertions.
      */

    recipient: WeakReference<MessageListener>,
}

impl MessageBaseInterface for Message {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl Default for Message {
    
    /**
      | Creates an uninitialised message.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        */
    }
}

pub type MessagePtr = ReferenceCountedObjectPtr<Message>;
