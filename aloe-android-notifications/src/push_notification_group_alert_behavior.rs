crate::ix!();

/**
  | Controls sound and vibration behaviour
  | for group notifications. Available
  | from Android API 26 or above.
  |
  */
pub enum PushNotificationGroupAlertBehaviour
{
    /**
      | both child notifications and group
      | notifications should produce sound
      | and vibration.
      |
      */
    alertAll,           

    /**
      | all child notifications in the group
      | should have no sound nor vibration,
      | even if corresponding notification
      | channel has sounds and vibrations enabled.
      |
      */
    AlertSummary,       

    /**
      | summary notifications in the group
      | should have no sound nor vibration,
      | even if corresponding notification
      | channel has sounds and vibrations enabled.
      |
      */
    AlertChildren,       
}

impl Default for PushNotificationGroupAlertBehaviour
{
    fn default() -> Self {
        todo!();
    }
}
