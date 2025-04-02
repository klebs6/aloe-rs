pub(crate) use icrate::AppKit::{NSApplicationDelegate,NSApplication};
pub(crate) use icrate::Foundation::{NSError,NSUserNotificationCenterDelegate,NSUserNotification,NSUserNotificationCenter};
pub(crate) use aloe_core::*;
pub(crate) use aloe_derive::*;
pub(crate) use aloe_3p::*;
pub(crate) use aloe_notifications::*;
pub(crate) use aloe_string::*;
pub(crate) use aloe_variant::*;
pub(crate) use objc_foundation::{NSArray,NSDictionary,NSData,NSObject,NSString};

// TODO
pub struct UIApplication {}
pub struct UIBackgroundFetchResult {}
pub struct UILocalNotification {}
pub struct UIUserNotificationAction {}
pub struct UIUserNotificationCategory {}
pub struct UIUserNotificationSettings {}

pub struct PushNotificationsDelegateAction {}
pub struct PushNotificationsDelegateCategory {}
