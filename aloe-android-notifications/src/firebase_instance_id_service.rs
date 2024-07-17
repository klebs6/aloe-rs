crate::ix!();

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_INSTANCE_ID_SERVICE_CLASSNAME)]
pub mod aloe_firebase_instance_id_service {
    use super::*;

    macro_rules! jni_class_members {
        ($METHOD:ident, 
         $STATICMETHOD:ident, 
         $FIELD:ident, 
         $STATICFIELD:ident, 
         $CALLBACK:ident) => {
            /*
            
                 CALLBACK (tokenRefreshed, "firebaseInstanceIdTokenRefreshed", "(Ljava/lang/String;)V")
            */
        }
    }

    declare_jni_class!{
        InstanceIdService, "com/rmsl/aloe/AloeFirebaseInstanceIdService"
    }

    #[JNICALL]
    pub fn token_refreshed(
            _0:                  *mut JNIEnv,
            instance_id_service: jobject,
            token:               *mut c_void)  {
        
        todo!();
            /*
                if (auto* instance = PushNotifications::getInstanceWithoutCreating())
                    instance->pimpl->notifyListenersTokenRefreshed (aloeString (static_cast<jstring> (token)));
            */
    }
}

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_INSTANCE_ID_SERVICE_CLASSNAME)]
lazy_static!{
    /*
    AloeFirebaseInstanceIdService::InstanceIdService_Class AloeFirebaseInstanceIdService::InstanceIdService;
    */
}


