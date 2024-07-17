crate::ix!();

/**
  | Register a listener (ideally on application
  | startup) to receive information about
  | notifications received and any callbacks
  | to async functions called.
  |
  */
pub trait PushNotificationListener
{

    /**
      | This callback will be called after you
      | call requestSettingsUsed() or requestPermissionsWithSettings().
      | 
      | -----------
      | @note
      | 
      | settings are currently only used on
      | iOS. When called on other platforms,
      | PushNotificationSettings with no categories and all
      | allow flags set to true will be received
      | in
      | 
      | PushNotificationListener::notificationSettingsReceived().
      |
      */
    fn notification_settings_received(&mut self, settings: &PushNotificationSettings)  {
        
        todo!();
        /*
            ignoreUnused (settings);
        */
    }

    /**
      | Called when the list of pending notifications,
      | requested by calling getPendingLocalNotifications()
      | is returned. iOS 10 or above only.
      |
      */
    fn pending_local_notifications_list_received(&mut self, notifications: &[PushNotification])  {
        
        todo!();
        /*
            ignoreUnused (notifications);
        */
    }


    /**
      | This can be called in multiple different
      | situations, depending on the OS and
      | the situation.
      | 
      | On pre iOS 10 device it will be called
      | when a user presses on a notification
      | or when a notification was received
      | when the app was in the foreground already.
      | On iOS 10 it will be called when a user
      | presses on a notification
      | 
      | -----------
      | @note
      | 
      | On Android, if remote notification
      | was received while the app was in the
      | background and then user pressed on
      | it, the notification object received
      | in this callback will contain only "properties"
      | member set. Hence, if you want to know
      | what was the notification title, content
      | etc, you need to set them as additional
      | properties, so that you will be able
      | to restore them from "properties" dictionary.
      | 
      | Note you can receive this callback on
      | startup, if the application was launched
      | from a notification.
      |
      */
    fn handle_notification(&mut self, 
        is_local_notification: bool,
        notification:          &PushNotification)  {
        
        todo!();
        /*
            ignoreUnused (isLocalNotification); ignoreUnused (notification);
        */
    }

    /**
      | This can be called when a user performs
      | some action on the notification such
      | as pressing on an action button or responding
      | with a text input.
      | 
      | -----------
      | @note
      | 
      | pressing on a notification area, i.e.
      | not on an action button is not considered
      | to be an action, and hence receivedNotification()
      | will be called in that case.
      | 
      | Note you can receive this callback on
      | startup, if the application was launched
      | from a notification's action.
      | 
      | -----------
      | @param isLocalNotification
      | 
      | If the notification is local
      | ----------
      | @param notification
      | 
      | The notification
      | ----------
      | @param actionIdentifier
      | 
      | A String identifying the action
      | ----------
      | @param optionalResponse
      | 
      | Text response a user inputs for notifications
      | with a text input.
      | 
      | Empty for notifications without a text
      | input option.
      |
      */
    fn handle_notification_action(&mut self, 
        is_local_notification: bool,
        notification:          &PushNotification,
        action_identifier:     &String,
        optional_response:     &String)  {
        
        todo!();
        /*
            ignoreUnused (isLocalNotification);
            ignoreUnused (notification);
            ignoreUnused (actionIdentifier);
            ignoreUnused (optionalResponse);
        */
    }

    /**
      | For iOS10 and Android, this can be also
      | called when a user dismissed the notification
      | before responding to it.
      |
      */
    fn local_notification_dismissed_by_user(&mut self, notification: &PushNotification)  {
        
        todo!();
        /*
            ignoreUnused (notification);
        */
    }

    /**
      | Called after getDeliveredNotifications()
      | request is fulfilled. Returns notifications
      | that are visible in the notification
      | area on the device and that are still
      | waiting for a user action/response.
      | 
      | On iOS, iOS version 10 or higher is required.
      | On Android, API level 18 or higher is
      | required.
      | 
      | For unsupported platforms, an empty
      | array will be returned.
      |
      */
    fn delivered_notifications_list_received(&mut self, notifications: &[PushNotification])  {
        
        todo!();
        /*
            ignoreUnused (notifications);
        */
    }

    /**
      | Called whenever a token gets refreshed.
      | You should monitor any token updates,
      | because only the last token that is assigned
      | to device is valid and can be used.
      |
      */
    fn device_token_refreshed(&mut self, token: &String)  {
        
        todo!();
        /*
            ignoreUnused (token);
        */
    }

    /**
      | Called when Firebase Cloud Messaging
      | server deletes pending messages. This
      | can happen when
      | 
      | 1) too many messages were sent to the
      | server (hint: use collapsible messages).
      | 
      | 2) the devices hasn't been online in
      | a long time (refer to Firebase documentation
      | for the maximum time a message can be
      | stored on FCM before expiring).
      |
      */
    fn remote_notifications_deleted(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Called when an upstream message sent
      | with PushNotifications::sendUpstreamMessage()
      | has been sent successfully.
      | 
      | Bear in mind that in may take several
      | minutes or more to receive this callback.
      |
      */
    fn upstream_message_sent(&mut self, message_id: &String)  {
        
        todo!();
        /*
            ignoreUnused (messageId);
        */
    }

    /**
      | Called when there was an error sending
      | an upstream message with
      | 
      | PushNotifications::sendUpstreamMessage().
      | 
      | Bear in mind that in may take several
      | minutes or more to receive this callback.
      |
      */
    fn upstream_message_sending_error(&mut self, 
        message_id: &String,
        error:      &String)  {
        
        todo!();
        /*
            ignoreUnused (messageId); ignoreUnused (error);
        */
    }
}
