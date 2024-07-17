crate::ix!();

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_INSTANCE_ID_SERVICE_CLASSNAME)]
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
           STATICMETHOD (getInstance, "getInstance", "()Lcom/google/firebase/iid/FirebaseInstanceId;") 
           METHOD (getToken, "getToken", "()Ljava/lang/String;")
        */
    }
}

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_INSTANCE_ID_SERVICE_CLASSNAME)]
declare_jni_class!{
    FirebaseInstanceId, 
    "com/google/firebase/iid/FirebaseInstanceId"
}
