crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (getNotification, "getNotification", "()Landroid/app/Notification;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    StatusBarNotification, 
    "android/service/notification/StatusBarNotification", 23
}
