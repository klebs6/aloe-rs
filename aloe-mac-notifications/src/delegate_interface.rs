crate::ix!();

pub trait PushNotificationsDelegateInterface {

    fn registered_for_remote_notifications(
        &mut self, 
        device_token: *mut NSData
    );

    fn failed_to_register_for_remote_notifications(
        &mut self, 
        error: *mut NSError
    );

    fn did_receive_remote_notification(
        &mut self, 
        user_info: *mut NSDictionary<NSObject,NSObject>
    );

    fn did_deliver_notification(
        &mut self, 
        notification: *mut NSUserNotification
    );

    fn did_activate_notification(
        &mut self, 
        notification: *mut NSUserNotification
    );

    fn should_present_notification(
        &mut self, 
        notification: *mut NSUserNotification
    ) -> bool;
}
