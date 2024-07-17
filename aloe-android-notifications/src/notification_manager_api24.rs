crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (areNotificationsEnabled, "areNotificationsEnabled", "()Z")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationManagerApi24, 
    "android/app/NotificationManager", 24
}
