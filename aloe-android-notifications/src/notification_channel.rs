crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (constructor,             "<init>",                  "(Ljava/lang/String;Ljava/lang/CharSequence;I)V") 
          METHOD (enableLights,            "enableLights",            "(Z)V") 
          METHOD (enableVibration,         "enableVibration",         "(Z)V") 
          METHOD (setBypassDnd,            "setBypassDnd",            "(Z)V") 
          METHOD (setDescription,          "setDescription",          "(Ljava/lang/String;)V") 
          METHOD (setGroup,                "setGroup",                "(Ljava/lang/String;)V") 
          METHOD (setImportance,           "setImportance",           "(I)V") 
          METHOD (setLightColor,           "setLightColor",           "(I)V") 
          METHOD (setLockscreenVisibility, "setLockscreenVisibility", "(I)V") 
          METHOD (setShowBadge,            "setShowBadge",            "(Z)V") 
          METHOD (setSound,                "setSound",                "(Landroid/net/Uri;Landroid/media/AudioAttributes;)V") 
          METHOD (setVibrationPattern,     "setVibrationPattern",     "([J)V")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationChannel, 
    "android/app/NotificationChannel", 26
}
