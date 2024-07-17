crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (setChronometerCountDown, "setChronometerCountDown", "(Z)Landroid/app/Notification$Builder;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationBuilderApi24, 
    "android/app/Notification$Builder", 24
}
