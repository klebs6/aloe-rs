crate::ix!();

/**
  | Controls timestamp visibility and
  | format.
  |
  */
pub enum PushNotificationTimestampVisibility
{
    /**
      | Do not show timestamp.
      |
      */
    off,                    

    /**
      | Show normal timestamp.
      |
      */
    normal,                 

    /**
      | Show chronometer as a stopwatch. Available
      | from Android API 16 or above.
      |
      */
    chronometer,            

    /**
      | Set the chronometer to count down instead
      | of counting up. Available from Android
      | API 24 or above.
      |
      */
    countDownChronometer,    
}

impl Default for PushNotificationTimestampVisibility {

    fn default() -> Self {
        todo!();
    }
}
