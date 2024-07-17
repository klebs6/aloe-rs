crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/broadcasters/aloe_ActionListener.h]

/**
  | Interface class for delivery of events
  | that are sent by an ActionBroadcaster.
  | 
  | @see ActionBroadcaster, ChangeListener
  | 
  | @tags{Events}
  |
  */
pub trait ActionListener {

    /**
      | Overridden by your subclass to receive
      | the callback.
      | 
      | -----------
      | @param message
      | 
      | the string that was specified when the
      | event was triggered by a call to ActionBroadcaster::sendActionMessage()
      |
      */
    fn action_listener_callback(&mut self, message: &String);
}
