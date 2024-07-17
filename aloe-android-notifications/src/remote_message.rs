crate::ix!();

#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
           METHOD (getCollapseKey,  "getCollapseKey",  "()Ljava/lang/String;") 
           METHOD (getData,         "getData",         "()Ljava/util/Map;") 
           METHOD (getFrom,         "getFrom",         "()Ljava/lang/String;") 
           METHOD (getMessageId,    "getMessageId",    "()Ljava/lang/String;") 
           METHOD (getMessageType,  "getMessageType",  "()Ljava/lang/String;") 
           METHOD (getNotification, "getNotification", "()Lcom/google/firebase/messaging/RemoteMessage$Notification;") 
           METHOD (getSentTime,     "getSentTime",     "()J") 
           METHOD (getTo,           "getTo",           "()Ljava/lang/String;") 
           METHOD (getTtl,          "getTtl",          "()I")
        */
    }
}

#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
declare_jni_class!{
    RemoteMessage, 
    "com/google/firebase/messaging/RemoteMessage"
}
