crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (addAction,       "addAction",       "(Landroid/app/Notification$Action;)Landroid/app/Notification$Builder;") 
          METHOD (addExtras,       "addExtras",       "(Landroid/os/Bundle;)Landroid/app/Notification$Builder;") 
          METHOD (setLocalOnly,    "setLocalOnly",    "(Z)Landroid/app/Notification$Builder;") 
          METHOD (setGroup,        "setGroup",        "(Ljava/lang/String;)Landroid/app/Notification$Builder;") 
          METHOD (setGroupSummary, "setGroupSummary", "(Z)Landroid/app/Notification$Builder;") 
          METHOD (setSortKey,      "setSortKey",      "(Ljava/lang/String;)Landroid/app/Notification$Builder;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationBuilderApi20, 
    "android/app/Notification$Builder", 20
}

