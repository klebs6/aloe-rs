crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/native/aloe_android_PushNotifications.cpp]

#[cfg(target_os="android")]
impl PushNotification {
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            bool isValidForPreApi26 = title.isNotEmpty() && body.isNotEmpty() && identifier.isNotEmpty() && icon.isNotEmpty();
        bool apiAtLeast26 = (getAndroidSDKVersion() >= 26);

        if (apiAtLeast26)
            return isValidForPreApi26 && channelId.isNotEmpty();

        return isValidForPreApi26;
        */
    }
}

#[cfg(target_os="android")]
pub fn aloe_handle_notification_intent(intent: *mut c_void) -> bool {
    
    todo!();
        /*
            auto* instance = PushNotifications::getInstanceWithoutCreating();

        if (PushNotifications::PushNotificationsPimpl::isDeleteNotificationIntent ((jobject) intent))
        {
            if (instance)
                instance->pimpl->notifyListenersAboutLocalNotificationDeleted (LocalRef<jobject> ((jobject) intent));

            return true;
        }
        else if (PushNotifications::PushNotificationsPimpl::isLocalNotificationIntent ((jobject) intent))
        {
            if (instance)
                instance->pimpl->notifyListenersAboutLocalNotification (LocalRef<jobject> ((jobject) intent));

            return true;
        }
      #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
        else if (PushNotifications::PushNotificationsPimpl::isRemoteNotificationIntent ((jobject) intent))
        {
            if (instance)
                instance->pimpl->notifyListenersAboutRemoteNotificationFromSystemTray (LocalRef<jobject> ((jobject) intent));

            return true;
        }
      #endif

        return false;
        */
}
