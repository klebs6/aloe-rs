crate::ix!();

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
           STATICMETHOD (getInstance, "getInstance", "()Lcom/google/firebase/messaging/FirebaseMessaging;") 
           METHOD (send,                 "send",                 "(Lcom/google/firebase/messaging/RemoteMessage;)V") 
           METHOD (subscribeToTopic,     "subscribeToTopic",     "(Ljava/lang/String;)Lcom/google/android/gms/tasks/Task;") 
           METHOD (unsubscribeFromTopic, "unsubscribeFromTopic", "(Ljava/lang/String;)Lcom/google/android/gms/tasks/Task;") 
        */
    }
}

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
declare_jni_class!{
    FirebaseMessaging, 
    "com/google/firebase/messaging/FirebaseMessaging"
}
