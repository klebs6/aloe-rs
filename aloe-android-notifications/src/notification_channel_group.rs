crate::ix!();

#[cfg(target_os="android")]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (constructor, "<init>", "(Ljava/lang/String;Ljava/lang/CharSequence;)V")
        */
    }
}

#[cfg(target_os="android")]
declare_jni_class_with_min_sdk!{
    NotificationChannelGroup, 
    "android/app/NotificationChannelGroup", 26
}


