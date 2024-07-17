crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_NotificationType.h]

/**
  | These enums are used in various classes
  | to indicate whether a notification
  | event should be sent out.
  |
  */
pub enum NotificationType
{
    /**
      | No notification message should be sent.
      |
      */
    dontSendNotification = 0,   

    /**
      | Requests a notification message, either
      | synchronous or not.
      |
      */
    sendNotification = 1,       

    /**
      | Requests a synchronous notification.
      |
      */
    sendNotificationSync,       

    /**
      | Requests an asynchronous notification.
      |
      */
    sendNotificationAsync,      
}
