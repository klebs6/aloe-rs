crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (addExtras,      "addExtras",      "(Landroid/os/Bundle;)Landroid/app/Notification$Action$Builder;") 
          METHOD (addRemoteInput, "addRemoteInput", "(Landroid/app/RemoteInput;)Landroid/app/Notification$Action$Builder;") 
          METHOD (constructor,    "<init>",         "(ILjava/lang/CharSequence;Landroid/app/PendingIntent;)V") 
          METHOD (build,          "build",          "()Landroid/app/Notification$Action;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationActionBuilder, 
    "android/app/Notification$Action$Builder", 20
}


