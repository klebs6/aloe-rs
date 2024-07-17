crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
           METHOD (addAction,          "addAction",          "(ILjava/lang/CharSequence;Landroid/app/PendingIntent;)Landroid/app/Notification$Builder;") 
           METHOD (build,              "build",              "()Landroid/app/Notification;") 
           METHOD (setPriority,        "setPriority",        "(I)Landroid/app/Notification$Builder;") 
           METHOD (setSubText,         "setSubText",         "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;") 
           METHOD (setUsesChronometer, "setUsesChronometer", "(Z)Landroid/app/Notification$Builder;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationBuilderApi16, 
    "android/app/Notification$Builder", 16
}

