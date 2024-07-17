crate::ix!();

pub trait PushNotificationsDelegateInterface {

    fn did_register_user_notification_settings(&mut self, 
        notification_settings: *mut UIUserNotificationSettings);

    fn registered_for_remote_notifications(&mut self, device_token: *mut NSData);

    fn failed_to_register_for_remote_notifications(&mut self, error: *mut NSError);

    fn did_receive_remote_notification(&mut self, user_info: *mut NSDictionary<NSObject,NSObject>);

    fn did_receive_remote_notification_fetch_completion_handler(&mut self, 
        user_info:          *mut NSDictionary<NSObject,NSObject>,
        completion_handler: fn(result: UIBackgroundFetchResult) -> c_void);

    fn handle_action_for_remote_notification_completion_handler(&mut self, 
        action_identifier:  *mut NSString,
        user_info:          *mut NSDictionary<NSObject,NSObject>,
        response_info:      *mut NSDictionary<NSObject,NSObject>,
        completion_handler: fn() -> c_void);

    fn did_receive_local_notification(&mut self, notification: *mut UILocalNotification);

    fn handle_action_for_local_notification_completion_handler(&mut self, 
        action_identifier:  *mut NSString,
        notification:       *mut UILocalNotification,
        completion_handler: fn() -> c_void);

    fn handle_action_for_local_notification_with_response_completion_handler(&mut self, 
        action_identifier:  *mut NSString,
        notification:       *mut UILocalNotification,
        response_info:      *mut NSDictionary<NSObject,NSObject>,
        completion_handler: fn() -> c_void);
}
