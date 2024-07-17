crate::ix!();

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
lazy_static!{
    /*
    AloeFirebaseMessagingService::MessagingService_Class  AloeFirebaseMessagingService::MessagingService;
    */
}

#[cfg(target_os="android")]
#[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
pub mod aloe_firebase_messaging_service {

    use super::*;

    macro_rules! jni_class_members {
        ($METHOD:ident, 
         $STATICMETHOD:ident, 
         $FIELD:ident, 
         $STATICFIELD:ident, 
         $CALLBACK:ident) => {
            /*
            
                 CALLBACK (remoteNotificationReceived,  "firebaseRemoteMessageReceived",  "(Lcom/google/firebase/messaging/RemoteMessage;)V") 
                 CALLBACK (remoteMessagesDeleted,       "firebaseRemoteMessagesDeleted",  "()V") 
                 CALLBACK (remoteMessageSent,           "firebaseRemoteMessageSent",      "(Ljava/lang/String;)V") 
                 CALLBACK (remoteMessageSendError,      "firebaseRemoteMessageSendError", "(Ljava/lang/String;Ljava/lang/String;)V")
            */
        }
    }

    declare_jni_class!{
        MessagingService, 
        "com/rmsl/aloe/AloeFirebaseMessagingService"
    }

    #[JNICALL]
    pub fn remote_notification_received(
            _0:                *mut JNIEnv,
            messaging_service: jobject,
            remote_message:    *mut c_void)  {
        
        todo!();
            /*
                if (auto* instance = PushNotifications::getInstanceWithoutCreating())
                    instance->pimpl->notifyListenersAboutRemoteNotificationFromService (LocalRef<jobject> (static_cast<jobject> (remoteMessage)));
            */
    }

    #[JNICALL]
    pub fn remote_messages_deleted()  {
        
        todo!();
            /*
                if (auto* instance = PushNotifications::getInstanceWithoutCreating())
                    instance->pimpl->notifyListenersAboutRemoteNotificationsDeleted();
            */
    }

    #[JNICALL]
    pub fn remote_message_sent(
            _0:                *mut JNIEnv,
            messaging_service: jobject,
            message_id:        *mut c_void)  {
        
        todo!();
            /*
                if (auto* instance = PushNotifications::getInstanceWithoutCreating())
                    instance->pimpl->notifyListenersAboutUpstreamMessageSent (LocalRef<jstring> (static_cast<jstring> (messageId)));
            */
    }

    #[JNICALL]
    pub fn remote_message_send_error(
            _0:                *mut JNIEnv,
            messaging_service: jobject,
            message_id:        *mut c_void,
            error:             *mut c_void)  {
        
        todo!();
            /*
                if (auto* instance = PushNotifications::getInstanceWithoutCreating())
                    instance->pimpl->notifyListenersAboutUpstreamMessageSendingError (LocalRef<jstring> (static_cast<jstring> (messageId)),
                                                                                      LocalRef<jstring> (static_cast<jstring> (error)));
            */
    }
}
