crate::ix!();

#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
           METHOD (addData,        "addData",        "(Ljava/lang/String;Ljava/lang/String;)Lcom/google/firebase/messaging/RemoteMessage$Builder;") 
           METHOD (build,          "build",          "()Lcom/google/firebase/messaging/RemoteMessage;") 
           METHOD (constructor,    "<init>",         "(Ljava/lang/String;)V") 
           METHOD (setCollapseKey, "setCollapseKey", "(Ljava/lang/String;)Lcom/google/firebase/messaging/RemoteMessage$Builder;") 
           METHOD (setMessageId,   "setMessageId",   "(Ljava/lang/String;)Lcom/google/firebase/messaging/RemoteMessage$Builder;") 
           METHOD (setMessageType, "setMessageType", "(Ljava/lang/String;)Lcom/google/firebase/messaging/RemoteMessage$Builder;") 
           METHOD (setTtl,         "setTtl",         "(I)Lcom/google/firebase/messaging/RemoteMessage$Builder;")
        */
    }
}

#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
declare_jni_class!{
    RemoteMessageBuilder, 
    "com/google/firebase/messaging/RemoteMessage$Builder"
}
