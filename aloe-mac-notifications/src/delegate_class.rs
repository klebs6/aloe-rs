crate::ix!();

type DelegateType = NSObject/*<NSApplicationDelegate,NSUserNotificationCenterDelegate>*/;

pub struct PushNotificationsDelegateClass {
    base: ObjCClass<DelegateType>,
}

impl Default for PushNotificationsDelegateClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject<NSApplicationDelegate, NSUserNotificationCenterDelegate>> ("AloePushNotificationsDelegate_")
                addIvar<PushNotificationsDelegate*> ("self");

                addMethod (@selector (application:didRegisterForRemoteNotificationsWithDeviceToken:), registeredForRemoteNotifications,       "v@:@@");
                addMethod (@selector (application:didFailToRegisterForRemoteNotificationsWithError:), failedToRegisterForRemoteNotifications, "v@:@@");
                addMethod (@selector (application:didReceiveRemoteNotification:),                     didReceiveRemoteNotification,           "v@:@@");
                addMethod (@selector (userNotificationCenter:didDeliverNotification:),                didDeliverNotification,                 "v@:@@");
                addMethod (@selector (userNotificationCenter:didActivateNotification:),               didActivateNotification,                "v@:@@");
                addMethod (@selector (userNotificationCenter:shouldPresentNotification:),             shouldPresentNotification,              "c@:@@");

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
    
    pub fn registered_for_remote_notifications(
        self_:        objc_id::Id<NSObject>,
        _1:           objc::runtime::Sel,
        _2:           *mut NSApplication,
        device_token: *mut NSData)  {
        
        todo!();
        /*
            getThis (self).registeredForRemoteNotifications (deviceToken);
        */
    }
    
    pub fn failed_to_register_for_remote_notifications(
        self_: objc_id::Id<NSObject>,
        _1:    objc::runtime::Sel,
        _2:    *mut NSApplication,
        error: *mut NSError)  {
        
        todo!();
        /*
            getThis (self).failedToRegisterForRemoteNotifications (error);
        */
    }
    
    pub fn did_receive_remote_notification<K,V>(
        self_:     objc_id::Id<NSObject>,
        _1:        objc::runtime::Sel,
        _2:        *mut NSApplication,
        user_info: *mut NSDictionary<K,V>)  {
        
        todo!();
        /*
            getThis (self).didReceiveRemoteNotification (userInfo);
        */
    }
    
    pub fn did_deliver_notification(
        self_:        objc_id::Id<NSObject>,
        _1:           objc::runtime::Sel,
        _2:           *mut NSUserNotificationCenter,
        notification: *mut NSUserNotification)  {
        
        todo!();
        /*
            getThis (self).didDeliverNotification (notification);
        */
    }
    
    pub fn did_activate_notification(
        self_:        objc_id::Id<NSObject>,
        _1:           objc::runtime::Sel,
        _2:           *mut NSUserNotificationCenter,
        notification: *mut NSUserNotification)  {
        
        todo!();
        /*
            getThis (self).didActivateNotification (notification);
        */
    }
    
    pub fn should_present_notification(
        self_:        objc_id::Id<NSObject>,
        _1:           objc::runtime::Sel,
        _2:           *mut NSUserNotificationCenter,
        notification: *mut NSUserNotification) -> bool {
        
        todo!();
        /*
            return getThis (self).shouldPresentNotification (notification);
        */
    }
}
