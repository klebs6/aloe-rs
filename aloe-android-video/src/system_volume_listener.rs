crate::ix!();

#[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
#[weak_referenceable]
pub struct SystemVolumeListener {
    owner:           &mut VideoComponentImpl,
    native_observer: GlobalRef,
}

#[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
pub mod system_volume_listener {

    lazy_static!{
        /*
        VideoComponent::VideoComponentImpl::SystemVolumeListener::SystemVolumeObserver_Class
        VideoComponent::VideoComponentImpl::SystemVolumeListener::SystemVolumeObserver;
        */
    }

    macro_rules! jni_class_members {
        ($METHOD:ident, 
         $STATICMETHOD:ident, 
         $FIELD:ident, 
         $STATICFIELD:ident, 
         $CALLBACK:ident) => {
            /*
            
                        METHOD (constructor, "<init>",     "(Landroid/app/Activity;J)V") 
                        METHOD (setEnabled,  "setEnabled", "(Z)V")                  
                        CALLBACK (systemVolumeChangedCallback, "mediaSessionSystemVolumeChanged", "(J)V")
            */
        }
    }

    declare_jni_class_with_min_sdk!{
        SystemVolumeObserver, 
        "com/rmsl/aloe/SystemVolumeObserver", 21
    }
}

#[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
impl Drop for SystemVolumeListener {
    fn drop(&mut self) {
        todo!();
        /*
            setEnabled (false);
        */
    }
}

#[cfg(ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME)]
impl SystemVolumeListener {

    pub fn new(owner_to_use: &mut VideoComponentImpl) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                    nativeObserver (createCallbackObject())
                    setEnabled (true);
        */
    }
    
    pub fn create_callback_object(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->NewObject (SystemVolumeObserver,
                                SystemVolumeObserver.constructor,
                                getCurrentActivity().get(),
                                reinterpret_cast<jlong> (this)));
        */
    }
    
    pub fn set_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            getEnv()->CallVoidMethod (nativeObserver, SystemVolumeObserver.setEnabled, shouldBeEnabled);

                    // Send first notification instantly to ensure sync.
                    if (shouldBeEnabled)
                        systemVolumeChanged();
        */
    }
    
    pub fn system_volume_changed(&mut self)  {
        
        todo!();
        /*
            MessageManager::callAsync ([weakThis = WeakReference<SystemVolumeListener> { this }]() mutable
                        {
                            if (weakThis == nullptr)
                                return;

                            if (weakThis->owner.owner.onGlobalMediaVolumeChanged != nullptr)
                                weakThis->owner.owner.onGlobalMediaVolumeChanged();
                        });
        */
    }
    
    pub fn system_volume_changed_callback(
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<VideoComponent::VideoComponentImpl::SystemVolumeListener*> (host))
                        myself->systemVolumeChanged();
        */
    }
}
