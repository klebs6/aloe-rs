crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_mac_PushNotifications.cpp]
pub struct PushNotificationImpl<'a> {
    base:                      PushNotificationsDelegate,
    owner:                     &'a mut PushNotifications,
    is_earlier_than_lion:      bool, //= std::floor (NSFoundationVersionNumber) < std::floor (NSFoundationVersionNumber10_7);
    is_at_least_mountain_lion: bool, //= std::floor (NSFoundationVersionNumber) >= NSFoundationVersionNumber10_7;
    is_earlier_than_mavericks: bool, //= std::floor (NSFoundationVersionNumber) < NSFoundationVersionNumber10_9;
    is_earlier_than_yosemite:  bool, //= std::floor (NSFoundationVersionNumber) <= NSFoundationVersionNumber10_9;
    initialised:               bool, // default = false
    device_token:              String,
    settings:                  PushNotificationSettings,
}

impl<'a> PushNotificationImpl<'a> {

    pub fn new(p: &mut PushNotifications) -> Self {
    
        todo!();
        /*
        : owner(p),
        */
    }
    
    pub fn request_permissions_with_settings(&mut self, settings_to_use: &PushNotificationSettings)  {
        
        todo!();
        /*
            if (isEarlierThanLion)
                return;

            settings = settingsToUse;

            NSRemoteNotificationType types = NSUInteger ((bool) settings.allowBadge);

            if (isAtLeastMountainLion)
                types |= (NSUInteger) ((bool) settings.allowSound << 1 | (bool) settings.allowAlert << 2);

            [[NSApplication sharedApplication] registerForRemoteNotificationTypes: types];
        */
    }
    
    pub fn request_settings_used(&mut self)  {
        
        todo!();
        /*
            if (isEarlierThanLion)
            {
                // no settings available
                owner.listeners.call ([] (Listener& l) { l.notificationSettingsReceived ({}); });
                return;
            }

            settings.allowBadge = [NSApplication sharedApplication].enabledRemoteNotificationTypes & NSRemoteNotificationTypeBadge;

            if (isAtLeastMountainLion)
            {
                settings.allowSound = [NSApplication sharedApplication].enabledRemoteNotificationTypes & NSRemoteNotificationTypeSound;
                settings.allowAlert = [NSApplication sharedApplication].enabledRemoteNotificationTypes & NSRemoteNotificationTypeAlert;
            }

            owner.listeners.call ([&] (Listener& l) { l.notificationSettingsReceived (settings); });
        */
    }
    
    pub fn are_notifications_enabled(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn send_local_notification(&mut self, n: &PushNotification)  {
        
        todo!();
        /*
            auto* notification = PushNotificationsDelegateDetailsOsx::aloeNotificationToNSUserNotification (n, isEarlierThanMavericks, isEarlierThanYosemite);

            [[NSUserNotificationCenter defaultUserNotificationCenter] scheduleNotification: notification];
        */
    }
    
    pub fn get_delivered_notifications(&self)  {
        
        todo!();
        /*
            Vec<PushNotifications::PushNotification> notifs;

            for (NSUserNotification* n in [NSUserNotificationCenter defaultUserNotificationCenter].deliveredNotifications)
                notifs.add (PushNotificationsDelegateDetailsOsx::nsUserNotificationToAloeNotification (n, isEarlierThanMavericks, isEarlierThanYosemite));

            owner.listeners.call ([&] (Listener& l) { l.deliveredNotificationsListReceived (notifs); });
        */
    }
    
    pub fn remove_all_delivered_notifications(&mut self)  {
        
        todo!();
        /*
            [[NSUserNotificationCenter defaultUserNotificationCenter] removeAllDeliveredNotifications];
        */
    }
    
    pub fn remove_delivered_notification(&mut self, identifier: &String)  {
        
        todo!();
        /*
            PushNotifications::PushNotification n;
            n.identifier = identifier;

            auto nsNotification = PushNotificationsDelegateDetailsOsx::aloeNotificationToNSUserNotification (n, isEarlierThanMavericks, isEarlierThanYosemite);

            [[NSUserNotificationCenter defaultUserNotificationCenter] removeDeliveredNotification: nsNotification];
        */
    }
    
    pub fn setup_channels(&mut self, 
        groups:   &[PushNotificationChannelGroup],
        channels: &[PushNotificationChannel])  {
        
        todo!();
        /*
            ignoreUnused (groups, channels);
        */
    }
    
    pub fn get_pending_local_notifications(&self)  {
        
        todo!();
        /*
            Vec<PushNotifications::PushNotification> notifs;

            for (NSUserNotification* n in [NSUserNotificationCenter defaultUserNotificationCenter].scheduledNotifications)
                notifs.add (PushNotificationsDelegateDetailsOsx::nsUserNotificationToAloeNotification (n, isEarlierThanMavericks, isEarlierThanYosemite));

            owner.listeners.call ([&] (Listener& l) { l.pendingLocalNotificationsListReceived (notifs); });
        */
    }
    
    pub fn remove_pending_local_notification(&mut self, identifier: &String)  {
        
        todo!();
        /*
            PushNotifications::PushNotification n;
            n.identifier = identifier;

            auto nsNotification = PushNotificationsDelegateDetailsOsx::aloeNotificationToNSUserNotification (n, isEarlierThanMavericks, isEarlierThanYosemite);

            [[NSUserNotificationCenter defaultUserNotificationCenter] removeScheduledNotification: nsNotification];
        */
    }
    
    pub fn remove_all_pending_local_notifications(&mut self)  {
        
        todo!();
        /*
            for (NSUserNotification* n in [NSUserNotificationCenter defaultUserNotificationCenter].scheduledNotifications)
                [[NSUserNotificationCenter defaultUserNotificationCenter] removeScheduledNotification: n];
        */
    }
    
    pub fn get_device_token(&mut self) -> String {
        
        todo!();
        /*
            // You need to call requestPermissionsWithSettings() first.
            jassert (initialised);

            return deviceToken;
        */
    }

    /**
      PushNotificationsDelegate
      */
    pub fn registered_for_remote_notifications(&mut self, device_token_to_use: *mut NSData)  {
        
        todo!();
        /*
            deviceToken = [deviceTokenToUse]() -> String
            {
                auto length = deviceTokenToUse.length;

                if (auto* buffer = (const unsigned char*) deviceTokenToUse.bytes)
                {
                    NSMutableString* hexString = [NSMutableString stringWithCapacity: (length * 2)];

                    for (NSUInteger i = 0; i < length; ++i)
                        [hexString appendFormat:@"%02x", buffer[i]];

                    return nsStringToAloe ([hexString copy]);
                }

                return {};
            }();

            initialised = true;

            owner.listeners.call ([&] (Listener& l) { l.deviceTokenRefreshed (deviceToken); });
        */
    }
    
    pub fn failed_to_register_for_remote_notifications(&mut self, error: *mut NSError)  {
        
        todo!();
        /*
            ignoreUnused (error);
            deviceToken.clear();
        */
    }
    
    pub fn did_receive_remote_notification<K,V>(&mut self, user_info: *mut NSDictionary<K,V>)  {
        
        todo!();
        /*
            auto n = PushNotificationsDelegateDetailsOsx::nsDictionaryToAloeNotification (userInfo);
            owner.listeners.call ([&] (Listener& l) { l.handleNotification (true, n); });
        */
    }
    
    pub fn did_deliver_notification(&mut self, notification: *mut NSUserNotification)  {
        
        todo!();
        /*
            ignoreUnused (notification);
        */
    }
    
    pub fn did_activate_notification(&mut self, notification: *mut NSUserNotification)  {
        
        todo!();
        /*
            auto n = PushNotificationsDelegateDetailsOsx::nsUserNotificationToAloeNotification (notification, isEarlierThanMavericks, isEarlierThanYosemite);

            if (notification.activationType == NSUserNotificationActivationTypeContentsClicked)
            {
                owner.listeners.call ([&] (Listener& l) { l.handleNotification (notification.remote, n); });
            }
            else
            {
                auto actionIdentifier = (! isEarlierThanYosemite && notification.additionalActivationAction != nil)
                                            ? nsStringToAloe (notification.additionalActivationAction.identifier)
                                            : nsStringToAloe (notification.actionButtonTitle);

                auto reply = notification.activationType == NSUserNotificationActivationTypeReplied
                                ? nsStringToAloe ([notification.response string])
                                : String();

                owner.listeners.call ([&] (Listener& l) { l.handleNotificationAction (notification.remote, n, actionIdentifier, reply); });
            }
        */
    }
    
    pub fn should_present_notification(&mut self, _0: *mut NSUserNotification) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn subscribe_to_topic(&mut self, topic: &String)  {
        
        todo!();
        /*
            ignoreUnused (topic);
        */
    }
    
    pub fn unsubscribe_from_topic(&mut self, topic: &String)  {
        
        todo!();
        /*
            ignoreUnused (topic);
        */
    }
    
    pub fn send_upstream_message(&mut self, 
        server_sender_id: &String,
        collapse_key:     &String,
        message_id:       &String,
        message_type:     &String,
        time_to_live:     i32,
        additional_data:  &StringPairArray)  {
        
        todo!();
        /*
            ignoreUnused (serverSenderId, collapseKey, messageId, messageType);
            ignoreUnused (timeToLive, additionalData);
        */
    }
}
