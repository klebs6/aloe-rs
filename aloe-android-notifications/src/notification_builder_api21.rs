crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (addPerson,        "addPerson",        "(Ljava/lang/String;)Landroid/app/Notification$Builder;") 
          METHOD (setCategory,      "setCategory",      "(Ljava/lang/String;)Landroid/app/Notification$Builder;") 
          METHOD (setColor,         "setColor",         "(I)Landroid/app/Notification$Builder;") 
          METHOD (setPublicVersion, "setPublicVersion", "(Landroid/app/Notification;)Landroid/app/Notification$Builder;") 
          METHOD (setVisibility,    "setVisibility",    "(I)Landroid/app/Notification$Builder;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationBuilderApi21, 
    "android/app/Notification$Builder", 21
}
