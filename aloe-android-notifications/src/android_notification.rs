crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          FIELD (extras, "extras", "Landroid/os/Bundle;")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    AndroidNotification, 
    "android/app/Notification", 19
}


