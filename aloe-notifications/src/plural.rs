crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_PushNotifications.h]

#[cfg(not(target_os="android"))]
pub struct PushNotifications {}

/**
  | Singleton class responsible for push
  | notifications functionality. Both
  | remote and local notifications are
  | supported. To get information about
  | notifications, register a listener
  | on your application startup. It is best
  | to register the listener as soon as possible,
  | because your application can be launched
  | from a push notification too.
  | 
  | To send a local notification create
  | an instance of Notification, fill the
  | necessary fields and call PushNotifications::sendLocalNotification().
  | When receiving local or remote notifications,
  | inspect the Notification's fields
  | for notification details.
  | 
  | Bear in mind that some fields will not
  | be available when receiving a remote
  | notification.
  | 
  | @tags{GUI}
  |
  */
#[cfg(target_os="android")]
pub struct PushNotifications {
    base:      DeletedAtShutdown,
    listeners: ListenerList<Rc<RefCell<dyn PushNotificationListener>>>,

    #[cfg(ALOE_PUSH_NOTIFICATIONS)]
    impl_: Box<Impl>,
}

#[cfg(target_os="android")]
aloe_implement_singleton!{
    PushNotifications
}

#[cfg(target_os="android")]
aloe_declare_singleton!{
    PushNotifications, false
}

#[cfg(target_os="android")]
impl Drop for PushNotifications {

    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_PushNotifications.cpp]
#[cfg(target_os="android")]
impl PushNotifications {

    pub fn new() -> Self {
    
        todo!();
        /*


            #if ALOE_PUSH_NOTIFICATIONS
        : impl (new Impl (*this))
      #endif
        */
    }
    
    pub fn add_listener(&mut self, l: *mut dyn PushNotificationListener)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    pub fn remove_listener(&mut self, l: *mut dyn PushNotificationListener)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    /**
      | Initialises push notifications on
      | current device with the settings provided.
      | 
      | Call this on your application startup
      | and on iOS the first time the application
      | starts, a user will be presented with
      | a permission request dialog to give
      | push notifications permission.
      | 
      | Once a user responds, Listener::notificationSettingsReceived()
      | will be called so that you can check what
      | permissions where actually granted.
      | The listener callback will be called
      | on each subsequent startup too (provided
      | you called requestPermissionsWithSettings()
      | on previous application run). This
      | way you can check what are current push
      | notifications permissions.
      | 
      | -----------
      | @note
      | 
      | settings are currently only used on
      | iOS. When calling on other platforms,
      | Settings with no categories and all
      | allow* flags set to true will be received
      | in
      | 
      | Listener::notificationSettingsReceived().
      | 
      | You can also call requestSettingsUsed()
      | to explicitly ask for current settings.
      |
      */
    pub fn request_permissions_with_settings(&mut self, settings: &PushNotificationSettings)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS && (ALOE_IOS || ALOE_MAC)
        impl->requestPermissionsWithSettings (settings);
      #else
        ignoreUnused (settings);
        listeners.call ([] (Listener& l) { l.notificationSettingsReceived ({}); });
      #endif
        */
    }
    
    /**
      | Sends an asynchronous request to retrieve
      | current settings that are currently
      | in use.
      | 
      | These can be exactly the same as used
      | in requestPermissionsWithSettings(),
      | but depending on user's subsequent
      | changes in OS settings, the actual current
      | settings may be different (e.g. user
      | might have later decided to disable
      | sounds).
      | 
      | -----------
      | @note
      | 
      | settings are currently only used on
      | iOS and partially on OSX.
      | 
      | On OSX, only allow* flags are used and
      | they refer to remote notifications
      | only. For local notifications, refer
      | to System Preferences.
      | 
      | When calling this function on other
      | platforms, Settings with no categories
      | and all allow* flags set to true will
      | be received in Listener::notificationSettingsReceived().
      |
      */
    pub fn request_settings_used(&mut self)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS && (ALOE_IOS || ALOE_MAC)
        impl->requestSettingsUsed();
      #else
        listeners.call ([] (Listener& l) { l.notificationSettingsReceived ({}); });
      #endif
        */
    }
    
    /**
      | Checks whether notifications are enabled
      | for given application.
      | 
      | On iOS and OSX this will always return
      | true, use requestSettingsUsed() instead.
      |
      */
    pub fn are_notifications_enabled(&self) -> bool {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        return impl->areNotificationsEnabled();
      #else
        return false;
      #endif
        */
    }
    
    /**
      | Sends a request for a list of notifications
      | delivered. Such notifications are
      | visible in the notification area on
      | the device and they are still waiting
      | for user action/response.
      | 
      | When the request is finished Listener::deliveredNotificationsListReceived()
      | will be called.
      | 
      | On iOS, iOS version 10 or higher is required.
      | On Android, API level 18 or higher is
      | required.
      | 
      | For unsupported platforms, Listener::deliveredNotificationsListReceived()
      | will return an empty array.
      |
      */
    pub fn get_delivered_notifications(&self)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->getDeliveredNotifications();
      #endif
        */
    }
    
    /**
      | Removes all notifications that were
      | delivered.
      |
      */
    pub fn remove_all_delivered_notifications(&mut self)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->removeAllDeliveredNotifications();
      #endif
        */
    }
    
    /**
      | Retrieves current device token. Note,
      | it is not a good idea to cache this token
      | because it may change in the meantime.
      | Always call this method to get the current
      | token value.
      |
      */
    pub fn get_device_token(&self) -> String {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        return impl->getDeviceToken();
      #else
        return {};
      #endif
        */
    }
    
    /**
      | Android API level 26 or higher only:
      | configures notification channel groups
      | and channels to be used in the app. These
      | have to be setup before notifications
      | can be sent on Android API level 26 or
      | higher.
      |
      */
    pub fn setup_channels(&mut self, 
        groups:   &[PushNotificationChannelGroup],
        channels: &[PushNotificationChannel])  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->setupChannels (groups, channels);
      #else
        ignoreUnused (groups, channels);
      #endif
        */
    }
    
    /**
      | iOS only: sends an asynchronous request
      | to retrieve a list of notifications
      | that were scheduled and not yet delivered.
      | 
      | When the list is retrieved, Listener::pendingLocalNotificationsListReceived()
      | will be called.
      |
      */
    pub fn get_pending_local_notifications(&self)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->getPendingLocalNotifications();
      #endif
        */
    }
    
    /**
      | Unschedules all pending local notifications.
      | iOS only.
      |
      */
    pub fn remove_all_pending_local_notifications(&mut self)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->removeAllPendingLocalNotifications();
      #endif
        */
    }
    
    /**
      | Android only: allows to subscribe to
      | messages from a specific topic.
      | 
      | So you could for instance subscribe
      | this device to all "sports" topic messages
      | to receive any remote notifications
      | that have "sports" topic set.
      | 
      | Refer to Firebase documentation for
      | how to send topic messages.
      |
      */
    pub fn subscribe_to_topic(&mut self, topic: &String)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->subscribeToTopic (topic);
      #else
        ignoreUnused (topic);
      #endif
        */
    }
    
    /**
      | Android only: allows to remove a topic
      | subscription that was previously added
      | with subscribeToTopic().
      |
      */
    pub fn unsubscribe_from_topic(&mut self, topic: &String)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->unsubscribeFromTopic (topic);
      #else
        ignoreUnused (topic);
      #endif
        */
    }
    
    /**
      | On iOS as well as on Android, sends a local
      | notification.
      | 
      | On Android and iOS 10 or above, this will
      | refresh an existing notification if
      | the same identifier is used as in a notification
      | that was already sent and not yet responded
      | by a user.
      |
      */
    pub fn send_local_notification(&mut self, n: &PushNotification)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->sendLocalNotification (n);
      #else
        ignoreUnused (n);
      #endif
        */
    }
    
    /**
      | Removes a previously delivered notification.
      | This can be useful for instance when
      | the information in the notification
      | becomes obsolete.
      |
      */
    pub fn remove_delivered_notification(&mut self, identifier: &String)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->removeDeliveredNotification (identifier);
      #else
        ignoreUnused (identifier);
      #endif
        */
    }
    
    /**
      | Unschedules a pending local notification
      | with a given identifier. Available
      | from iOS 10.
      |
      */
    pub fn remove_pending_local_notification(&mut self, identifier: &String)  {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->removePendingLocalNotification (identifier);
      #else
        ignoreUnused (identifier);
      #endif
        */
    }
    
    /**
      | Android only: sends an upstream message
      | to your app server. The server must implement
      | 
      | XMPP Connection Server protocol (refer
      | to Firebase documentation).
      | 
      | -----------
      | @note
      | 
      | there may be a limit of maximum collapse
      | keys used at the same time and beyond
      | the limit (refer to
      | 
      | Firebase documentation) it is not guaranteed
      | which keys will be in use by the server.
      | 
      | -----------
      | @param serverSenderId
      | 
      | Represents the sender. Consult your
      | Firebase project settings to retrieve
      | the sender id.
      | ----------
      | @param collapseKey
      | 
      | Remote messages with the same collapse
      | key that were not yet delivered will
      | be collapsed into one, with the newest
      | message replacing all the previous
      | ones.
      | ----------
      | @param messageId
      | 
      | A unique message ID. Used in error callbacks
      | and debugging.
      | ----------
      | @param messageType
      | 
      | Message type.
      | ----------
      | @param timeToLive
      | 
      | TTL in seconds. If 0, the message sending
      | will be attempted immediately and it
      | will be dropped if the device is not connected.
      | Otherwise, the message will be queued
      | for the period specified.
      | ----------
      | @param additionalData
      | 
      | Collection of key-value pairs to be
      | used as an additional data for the message.
      |
      */
    pub fn send_upstream_message(
        &mut self, 
        server_sender_id: &String,
        collapse_key:     &String,
        message_id:       &String,
        message_type:     &String,
        time_to_live:     i32,
        additional_data:  &Vec<(String,String)>

    ) {
        
        todo!();
        /*
            #if ALOE_PUSH_NOTIFICATIONS
        impl->sendUpstreamMessage (serverSenderId,
                                    collapseKey,
                                    messageId,
                                    messageType,
                                    timeToLive,
                                    additionalData);
      #else
        ignoreUnused (serverSenderId, collapseKey, messageId, messageType);
        ignoreUnused (timeToLive, additionalData);
      #endif
        */
    }
}

