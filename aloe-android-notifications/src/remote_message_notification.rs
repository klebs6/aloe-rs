crate::ix!();

#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
           METHOD (getBody,                  "getBody",                  "()Ljava/lang/String;") 
           METHOD (getBodyLocalizationArgs,  "getBodyLocalizationArgs",  "()[Ljava/lang/String;") 
           METHOD (getBodyLocalizationKey,   "getBodyLocalizationKey",   "()Ljava/lang/String;") 
           METHOD (getClickAction,           "getClickAction",           "()Ljava/lang/String;") 
           METHOD (getColor,                 "getColor",                 "()Ljava/lang/String;") 
           METHOD (getIcon,                  "getIcon",                  "()Ljava/lang/String;") 
           METHOD (getLink,                  "getLink",                  "()Landroid/net/Uri;") 
           METHOD (getSound,                 "getSound",                 "()Ljava/lang/String;") 
           METHOD (getTag,                   "getTag",                   "()Ljava/lang/String;") 
           METHOD (getTitle,                 "getTitle",                 "()Ljava/lang/String;") 
           METHOD (getTitleLocalizationArgs, "getTitleLocalizationArgs", "()[Ljava/lang/String;") 
           METHOD (getTitleLocalizationKey,  "getTitleLocalizationKey",  "()Ljava/lang/String;")
        */
    }
}

#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
declare_jni_class!{
    RemoteMessageNotification, 
    "com/google/firebase/messaging/RemoteMessage$Notification"
}
