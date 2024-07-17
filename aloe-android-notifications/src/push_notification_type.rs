crate::ix!();

impl Default for PushNotificationType {

    fn default() -> Self {
        Self::unspecified
    }
}

/**
  | Metadata that can be used by the OS to
  | better handle the notification, depending
  | on its priority.
  |
  */
pub enum PushNotificationType
{
    /**
      | Category not set.
      |
      */
    unspecified,       

    /**
      | Alarm or timer.
      |
      */
    alarm,             

    /**
      | Incoming voice/video call or similar.
      |
      */
    call,              

    /**
      | Async message like email.
      |
      */
    email,             

    /**
      | Error in background operation or authentication
      | status.
      |
      */
    error,             

    /**
      | Calendar event.
      |
      */
    event,             

    /**
      | Incoming message (sms, instant message
      | etc.).
      |
      */
    message,           

    /**
      | PushNotificationProgress for a long-running background
      | operation.
      |
      */
    taskProgress,      

    /**
      | Promotion or advertisement.
      |
      */
    promo,             

    /**
      | Specific, single thing related recommendation.
      |
      */
    recommendation,    

    /**
      | User-scheduled reminder.
      |
      */
    reminder,          

    /**
      | Running background service.
      |
      */
    service,           

    /**
      | Social network or sharing update.
      |
      */
    social,            

    /**
      | Ongoing information about device or
      | contextual status.
      |
      */
    status,            

    /**
      | System or device status update.
      |
      */
    system,            

    /**
      | Media transport control for playback.
      |
      */
    transport,          
}
