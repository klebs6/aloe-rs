crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (getNotification,  "getNotification",  "()Landroid/app/Notification;") 
          METHOD (setAutoCancel,    "setAutoCancel",    "(Z)Landroid/app/Notification$Builder;") 
          METHOD (setContentInfo,   "setContentInfo",   "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;") 
          METHOD (setContentIntent, "setContentIntent", "(Landroid/app/PendingIntent;)Landroid/app/Notification$Builder;") 
          METHOD (setContentText,   "setContentText",   "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;") 
          METHOD (setContentTitle,  "setContentTitle",  "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;") 
          METHOD (setDefaults,      "setDefaults",      "(I)Landroid/app/Notification$Builder;") 
          METHOD (setDeleteIntent,  "setDeleteIntent",  "(Landroid/app/PendingIntent;)Landroid/app/Notification$Builder;") 
          METHOD (setLargeIcon,     "setLargeIcon",     "(Landroid/graphics/Bitmap;)Landroid/app/Notification$Builder;") 
          METHOD (setLights,        "setLights",        "(III)Landroid/app/Notification$Builder;") 
          METHOD (setNumber,        "setNumber",        "(I)Landroid/app/Notification$Builder;") 
          METHOD (setOngoing,       "setOngoing",       "(Z)Landroid/app/Notification$Builder;") 
          METHOD (setOnlyAlertOnce, "setOnlyAlertOnce", "(Z)Landroid/app/Notification$Builder;") 
          METHOD (setProgress,      "setProgress",      "(IIZ)Landroid/app/Notification$Builder;") 
          METHOD (setSmallIcon,     "setSmallIcon",     "(I)Landroid/app/Notification$Builder;") 
          METHOD (setSound,         "setSound",         "(Landroid/net/Uri;)Landroid/app/Notification$Builder;") 
          METHOD (setTicker,        "setTicker",        "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;") 
          METHOD (setVibrate,       "setVibrate",       "([J)Landroid/app/Notification$Builder;") 
          METHOD (setWhen,          "setWhen",          "(J)Landroid/app/Notification$Builder;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationBuilderBase, 
    "android/app/Notification$Builder", 11
}
