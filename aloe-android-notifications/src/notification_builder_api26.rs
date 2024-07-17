crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (setBadgeIconType,      "setBadgeIconType",      "(I)Landroid/app/Notification$Builder;") 
          METHOD (setGroupAlertBehavior, "setGroupAlertBehavior", "(I)Landroid/app/Notification$Builder;") 
          METHOD (setTimeoutAfter,       "setTimeoutAfter",       "(J)Landroid/app/Notification$Builder;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationBuilderApi26, 
    "android/app/Notification$Builder", 26
}
