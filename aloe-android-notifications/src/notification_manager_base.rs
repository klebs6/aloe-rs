crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (cancel,    "cancel",    "(Ljava/lang/String;I)V") 
          METHOD (cancelAll, "cancelAll", "()V") 
          METHOD (notify,    "notify",    "(Ljava/lang/String;ILandroid/app/Notification;)V")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class!{
    NotificationManagerBase, 
    "android/app/NotificationManager"
}
