crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_ios_PushNotifications.cpp]
pub struct PushNotificationPimpl<'a> {
    base:              PushNotificationsDelegate,
    owner:             &'a mut PushNotifications,
    os_earlier_than10: bool, //= std::floor (NSFoundationVersionNumber) <= NSFoundationVersionNumber_iOS_9_x_Max;
    initialised:       bool, // default = false
    device_token:      String,
    settings:          PushNotificationSettings,
}

impl<'a> PushNotificationPimpl<'a> {

    pub fn new(p: &mut PushNotifications) -> Self {
    
        todo!();
        /*
        : owner(p),

        
        */
    }
    
    pub fn request_permissions_with_settings(&mut self, settings_to_use: &PushNotificationSettings)  {
        
        todo!();
        /*
            settings = settingsToUse;

                auto categories = [NSMutableSet setWithCapacity: (NSUInteger) settings.categories.size()];

                if (iOSEarlierThan10)
                {
                    for (const auto& c : settings.categories)
                    {
                        auto* category = (UIUserNotificationCategory*) PushNotificationsDelegateDetails::categoryToNSCategory (c, iOSEarlierThan10);
                        [categories addObject: category];
                    }

                    UIUserNotificationType type = NSUInteger ((bool)settings.allowBadge << 0
                                                            | (bool)settings.allowSound << 1
                                                            | (bool)settings.allowAlert << 2);

                    UIUserNotificationSettings* s = [UIUserNotificationSettings settingsForTypes: type categories: categories];
                    [[UIApplication sharedApplication] registerUserNotificationSettings: s];
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {
                    for (const auto& c : settings.categories)
                    {
                        auto* category = (UNNotificationCategory*) PushNotificationsDelegateDetails::categoryToNSCategory (c, iOSEarlierThan10);
                        [categories addObject: category];
                    }

                    UNAuthorizationOptions authOptions = NSUInteger ((bool)settings.allowBadge << 0
                                                                   | (bool)settings.allowSound << 1
                                                                   | (bool)settings.allowAlert << 2);

                    [[UNUserNotificationCenter currentNotificationCenter] setNotificationCategories: categories];
                    [[UNUserNotificationCenter currentNotificationCenter] requestAuthorizationWithOptions: authOptions
                                                                                        completionHandler: ^(BOOL /*granted*/, NSError* /*error*/)
                                                                                                           {
                                                                                                               requestSettingsUsed();
                                                                                                           }];
                }
               #endif

                [[UIApplication sharedApplication] registerForRemoteNotifications];
        */
    }
    
    pub fn request_settings_used(&mut self)  {
        
        todo!();
        /*
            if (iOSEarlierThan10)
                {
                    UIUserNotificationSettings* s = [UIApplication sharedApplication].currentUserNotificationSettings;

                    settings.allowBadge = s.types & UIUserNotificationTypeBadge;
                    settings.allowSound = s.types & UIUserNotificationTypeSound;
                    settings.allowAlert = s.types & UIUserNotificationTypeAlert;

                    for (UIUserNotificationCategory *c in s.categories)
                        settings.categories.add (PushNotificationsDelegateDetails::uiUserNotificationCategoryToCategory (c));

                    owner.listeners.call ([&] (Listener& l) { l.notificationSettingsReceived (settings); });
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {

                    [[UNUserNotificationCenter currentNotificationCenter] getNotificationSettingsWithCompletionHandler:
                     ^(UNNotificationSettings* s)
                     {
                         [[UNUserNotificationCenter currentNotificationCenter] getNotificationCategoriesWithCompletionHandler:
                          ^(NSSet<UNNotificationCategory*>* categories)
                          {
                              settings.allowBadge = s.badgeSetting == UNNotificationSettingEnabled;
                              settings.allowSound = s.soundSetting == UNNotificationSettingEnabled;
                              settings.allowAlert = s.alertSetting == UNNotificationSettingEnabled;

                              for (UNNotificationCategory* c in categories)
                                  settings.categories.add (PushNotificationsDelegateDetails::unNotificationCategoryToCategory (c));

                              owner.listeners.call ([&] (Listener& l) { l.notificationSettingsReceived (settings); });
                          }
                         ];

                     }];
                }
               #endif
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
            if (iOSEarlierThan10)
                {
                    auto* notification = PushNotificationsDelegateDetails::aloeNotificationToUILocalNotification (n);

                    [[UIApplication sharedApplication] scheduleLocalNotification: notification];
                    [notification release];
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {

                    UNNotificationRequest* request = PushNotificationsDelegateDetails::aloeNotificationToUNNotificationRequest (n);

                    [[UNUserNotificationCenter currentNotificationCenter] addNotificationRequest: request
                                                                           withCompletionHandler: ^(NSError* error)
                                                                                                  {
                                                                                                      jassert (error == nil);

                                                                                                      if (error != nil)
                                                                                                          NSLog (nsStringLiteral ("addNotificationRequest error: %@"), error);
                                                                                                  }];
                }
               #endif
        */
    }
    
    pub fn get_delivered_notifications(&self)  {
        
        todo!();
        /*
            if (iOSEarlierThan10)
                {
                    // Not supported on this platform
                    jassertfalse;
                    owner.listeners.call ([] (Listener& l) { l.deliveredNotificationsListReceived ({}); });
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {
                    [[UNUserNotificationCenter currentNotificationCenter] getDeliveredNotificationsWithCompletionHandler:
                     ^(NSArray<UNNotification*>* notifications)
                     {
                        Vec<PushNotifications::PushNotification> notifs;

                        for (UNNotification* n in notifications)
                            notifs.add (PushNotificationsDelegateDetails::unNotificationToAloeNotification (n));

                        owner.listeners.call ([&] (Listener& l) { l.deliveredNotificationsListReceived (notifs); });
                     }];
                }
               #endif
        */
    }
    
    pub fn remove_all_delivered_notifications(&mut self)  {
        
        todo!();
        /*
            if (iOSEarlierThan10)
                {
                    // Not supported on this platform
                    jassertfalse;
                }
                else
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                {

                    [[UNUserNotificationCenter currentNotificationCenter] removeAllDeliveredNotifications];
                }
               #endif
        */
    }
    
    pub fn remove_delivered_notification(&mut self, identifier: &String)  {
        
        todo!();
        /*
            if (iOSEarlierThan10)
                {
                    ignoreUnused (identifier);
                    // Not supported on this platform
                    jassertfalse;
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {

                    NSArray<NSString*>* identifiers = [NSArray arrayWithObject: aloeStringToNS (identifier)];

                    [[UNUserNotificationCenter currentNotificationCenter] removeDeliveredNotificationsWithIdentifiers: identifiers];
                }
               #endif
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
            if (iOSEarlierThan10)
                {
                    Vec<PushNotifications::PushNotification> notifs;

                    for (UILocalNotification* n in [UIApplication sharedApplication].scheduledLocalNotifications)
                        notifs.add (PushNotificationsDelegateDetails::uiLocalNotificationToAloeNotification (n));

                    owner.listeners.call ([&] (Listener& l) { l.pendingLocalNotificationsListReceived (notifs); });
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {

                    [[UNUserNotificationCenter currentNotificationCenter] getPendingNotificationRequestsWithCompletionHandler:
                     ^(NSArray<UNNotificationRequest*>* requests)
                     {
                         Vec<PushNotifications::PushNotification> notifs;

                         for (UNNotificationRequest* r : requests)
                             notifs.add (PushNotificationsDelegateDetails::unNotificationRequestToAloeNotification (r));

                         owner.listeners.call ([&] (Listener& l) { l.pendingLocalNotificationsListReceived (notifs); });
                     }
                    ];
                }
               #endif
        */
    }
    
    pub fn remove_pending_local_notification(&mut self, identifier: &String)  {
        
        todo!();
        /*
            if (iOSEarlierThan10)
                {
                    // Not supported on this platform
                    jassertfalse;
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {

                    NSArray<NSString*>* identifiers = [NSArray arrayWithObject: aloeStringToNS (identifier)];

                    [[UNUserNotificationCenter currentNotificationCenter] removePendingNotificationRequestsWithIdentifiers: identifiers];
                }
               #endif
        */
    }
    
    pub fn remove_all_pending_local_notifications(&mut self)  {
        
        todo!();
        /*
            if (iOSEarlierThan10)
                {
                    [[UIApplication sharedApplication] cancelAllLocalNotifications];
                }
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                else
                {
                    [[UNUserNotificationCenter currentNotificationCenter] removeAllPendingNotificationRequests];
                }
               #endif
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
    pub fn did_register_user_notification_settings(&mut self, _0: *mut UIUserNotificationSettings)  {
        
        todo!();
        /*
            requestSettingsUsed();
        */
    }
    
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
    
    pub fn did_receive_remote_notification(&mut self, user_info: *mut NSDictionary<NSObject,NSObject>)  {
        
        todo!();
        /*
            auto n = PushNotificationsDelegateDetails::nsDictionaryToAloeNotification (userInfo);

                owner.listeners.call ([&] (Listener& l) { l.handleNotification (false, n); });
        */
    }
    
    pub fn did_receive_remote_notification_fetch_completion_handler(&mut self, 
        user_info:          *mut NSDictionary<NSObject,NSObject>,
        completion_handler: fn(result: UIBackgroundFetchResult) -> c_void)  {
        
        todo!();
        /*
            didReceiveRemoteNotification (userInfo);
                completionHandler (UIBackgroundFetchResultNewData);
        */
    }
    
    pub fn handle_action_for_remote_notification_completion_handler(&mut self, 
        action_identifier:  *mut NSString,
        user_info:          *mut NSDictionary<NSObject,NSObject>,
        response_info:      *mut NSDictionary<NSObject,NSObject>,
        completion_handler: fn() -> c_void)  {
        
        todo!();
        /*
            auto n = PushNotificationsDelegateDetails::nsDictionaryToAloeNotification (userInfo);
                auto actionString = nsStringToAloe (actionIdentifier);
                auto response = PushNotificationsDelegateDetails::getUserResponseFromNSDictionary (responseInfo);

                owner.listeners.call ([&] (Listener& l) { l.handleNotificationAction (false, n, actionString, response); });

                completionHandler();
        */
    }
    
    pub fn did_receive_local_notification(&mut self, notification: *mut UILocalNotification)  {
        
        todo!();
        /*
            auto n = PushNotificationsDelegateDetails::uiLocalNotificationToAloeNotification (notification);

                owner.listeners.call ([&] (Listener& l) { l.handleNotification (true, n); });
        */
    }
    
    pub fn handle_action_for_local_notification_completion_handler(&mut self, 
        action_identifier:  *mut NSString,
        notification:       *mut UILocalNotification,
        completion_handler: fn() -> c_void)  {
        
        todo!();
        /*
            handleActionForLocalNotificationWithResponseCompletionHandler (actionIdentifier,
                                                                               notification,
                                                                               nil,
                                                                               completionHandler);
        */
    }
    
    pub fn handle_action_for_local_notification_with_response_completion_handler(&mut self, 
        action_identifier:  *mut NSString,
        notification:       *mut UILocalNotification,
        response_info:      *mut NSDictionary<NSObject,NSObject>,
        completion_handler: fn() -> c_void)  {
        
        todo!();
        /*
            auto n = PushNotificationsDelegateDetails::uiLocalNotificationToAloeNotification (notification);
                auto actionString = nsStringToAloe (actionIdentifier);
                auto response = PushNotificationsDelegateDetails::getUserResponseFromNSDictionary (responseInfo);

                owner.listeners.call ([&] (Listener& l) { l.handleNotificationAction (true, n, actionString, response); });

                completionHandler();
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
