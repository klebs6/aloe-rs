crate::ix!();

type DelegateType = NSObject /*<UIApplicationDelegate>*/;

pub struct PushNotificationsDelegateClass {
    base: ObjCClass<DelegateType>,
}

impl Default for PushNotificationsDelegateClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject<UIApplicationDelegate>> ("AloePushNotificationsDelegate_")
                addIvar<PushNotificationsDelegate*> ("self");

                addMethod (@selector (application:didRegisterUserNotificationSettings:),                                                 didRegisterUserNotificationSettings,                            "v@:@@");
                addMethod (@selector (application:didRegisterForRemoteNotificationsWithDeviceToken:),                                    registeredForRemoteNotifications,                               "v@:@@");
                addMethod (@selector (application:didFailToRegisterForRemoteNotificationsWithError:),                                    failedToRegisterForRemoteNotifications,                         "v@:@@");
                addMethod (@selector (application:didReceiveRemoteNotification:),                                                        didReceiveRemoteNotification,                                   "v@:@@");
                addMethod (@selector (application:didReceiveRemoteNotification:fetchCompletionHandler:),                                 didReceiveRemoteNotificationFetchCompletionHandler,             "v@:@@@");
                addMethod (@selector (application:handleActionWithIdentifier:forRemoteNotification:withResponseInfo:completionHandler:), handleActionForRemoteNotificationCompletionHandler,             "v@:@@@@@");
                addMethod (@selector (application:didReceiveLocalNotification:),                                                         didReceiveLocalNotification,                                    "v@:@@");
                addMethod (@selector (application:handleActionWithIdentifier:forLocalNotification:completionHandler:),                   handleActionForLocalNotificationCompletionHandler,              "v@:@@@@");
                addMethod (@selector (application:handleActionWithIdentifier:forLocalNotification:withResponseInfo:completionHandler:),  handleActionForLocalNotificationWithResponseCompletionHandler,  "v@:@@@@@");

               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                addMethod (@selector (userNotificationCenter:willPresentNotification:withCompletionHandler:),                            willPresentNotificationWithCompletionHandler,                   "v@:@@@");
                addMethod (@selector (userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:),                     didReceiveNotificationResponseWithCompletionHandler,            "v@:@@@");
               #endif

                registerClass();
        */
    }
}

impl PushNotificationsDelegateClass {

    pub fn get_this<'a>(self_: objc_id::Id<NSObject>) -> &'a mut PushNotificationsDelegate {
        
        todo!();
        /*
            return *getIvar<PushNotificationsDelegate*> (self, "self");
        */
    }
    
    pub fn set_this(
        self_: objc_id::Id<NSObject>,
        d:     *mut PushNotificationsDelegate)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "self", d);
        */
    }
    
    pub fn did_register_user_notification_settings(
        self_:    objc_id::Id<NSObject>,
        _1:       objc::runtime::Sel,
        _2:       *mut UIApplication,
        settings: *mut UIUserNotificationSettings)  {
        
        todo!();
        /*
            getThis (self).didRegisterUserNotificationSettings (settings);
        */
    }
    
    pub fn registered_for_remote_notifications(
        self_:        objc_id::Id<NSObject>,
        _1:           objc::runtime::Sel,
        _2:           *mut UIApplication,
        device_token: *mut NSData)  {
        
        todo!();
        /*
            getThis (self).registeredForRemoteNotifications (deviceToken);
        */
    }
    
    pub fn failed_to_register_for_remote_notifications(
        self_: objc_id::Id<NSObject>,
        _1:    objc::runtime::Sel,
        _2:    *mut UIApplication,
        error: *mut NSError)  {
        
        todo!();
        /*
            getThis (self).failedToRegisterForRemoteNotifications (error);
        */
    }
    
    pub fn did_receive_remote_notification<K,V>(
        self_:     objc_id::Id<NSObject>,
        _1:        objc::runtime::Sel,
        _2:        *mut UIApplication,
        user_info: *mut NSDictionary<K,V>)  {
        
        todo!();
        /*
            getThis (self).didReceiveRemoteNotification (userInfo);
        */
    }
    
    pub fn did_receive_remote_notification_fetch_completion_handler<K,V>(
        self_:              objc_id::Id<NSObject>,
        _1:                 objc::runtime::Sel,
        _2:                 *mut UIApplication,
        user_info:          *mut NSDictionary<K,V>,
        completion_handler: fn(result: UIBackgroundFetchResult) -> c_void)  {
        
        todo!();
        /*
            getThis (self).didReceiveRemoteNotificationFetchCompletionHandler (userInfo, completionHandler);
        */
    }
    
    pub fn handle_action_for_remote_notification_completion_handler<K,V>(
        self_:              objc_id::Id<NSObject>,
        _1:                 objc::runtime::Sel,
        _2:                 *mut UIApplication,
        action_identifier:  *mut NSString,
        user_info:          *mut NSDictionary<K,V>,
        response_info:      *mut NSDictionary<K,V>,
        completion_handler: fn() -> c_void)  {
        
        todo!();
        /*
            getThis (self).handleActionForRemoteNotificationCompletionHandler (actionIdentifier, userInfo, responseInfo, completionHandler);
        */
    }
    
    pub fn did_receive_local_notification(
        self_:        objc_id::Id<NSObject>,
        _1:           objc::runtime::Sel,
        _2:           *mut UIApplication,
        notification: *mut UILocalNotification)  {
        
        todo!();
        /*
            getThis (self).didReceiveLocalNotification (notification);
        */
    }
    
    pub fn handle_action_for_local_notification_completion_handler(
        self_:              objc_id::Id<NSObject>,
        _1:                 objc::runtime::Sel,
        _2:                 *mut UIApplication,
        action_identifier:  *mut NSString,
        notification:       *mut UILocalNotification,
        completion_handler: fn() -> c_void)  {
        
        todo!();
        /*
            getThis (self).handleActionForLocalNotificationCompletionHandler (actionIdentifier, notification, completionHandler);
        */
    }
    
    pub fn handle_action_for_local_notification_with_response_completion_handler<K,V>(
        self_:              objc_id::Id<NSObject>,
        _1:                 objc::runtime::Sel,
        _2:                 *mut UIApplication,
        action_identifier:  *mut NSString,
        notification:       *mut UILocalNotification,
        response_info:      *mut NSDictionary<K,V>,
        completion_handler: fn() -> c_void)  {
        
        todo!();
        /*
            getThis (self). handleActionForLocalNotificationWithResponseCompletionHandler (actionIdentifier, notification, responseInfo, completionHandler);
        */
    }
}
