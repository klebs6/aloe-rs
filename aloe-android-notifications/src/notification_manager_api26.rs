crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (createNotificationChannel,      "createNotificationChannel",      "(Landroid/app/NotificationChannel;)V") 
          METHOD (createNotificationChannelGroup, "createNotificationChannelGroup", "(Landroid/app/NotificationChannelGroup;)V")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationManagerApi26, 
    "android/app/NotificationManager", 26
}
