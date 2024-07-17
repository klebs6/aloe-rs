crate::ix!();

/**
  | Metadata used as a hint to the OS about
  | the priority of the notification.
  |
  */
pub enum PushNotificationPriority
{
    veryLow  = -2,
    low      = -1,
    medium   =  0,
    high     =  1,
    veryHigh =  2
}

impl Default for PushNotificationPriority {

    fn default() -> Self {
        todo!();
    }
}
