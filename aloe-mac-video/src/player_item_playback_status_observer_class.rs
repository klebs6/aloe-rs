crate::ix!();

pub struct AloePlayerItemPlaybackStatusObserverClass {
    base: ObjCClass<NSObject>,
}

impl Default for AloePlayerItemPlaybackStatusObserverClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("AloePlayerItemPlaybackStatusObserverClass_")
                ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                addMethod (@selector (processNotification:), notificationReceived, "v@:@");
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE

                addIvar<PlayerControllerBase*> ("owner");

                registerClass();
        */
    }
}

impl AloePlayerItemPlaybackStatusObserverClass {

    pub fn get_owner<'a>(self_: Id<Self>) -> &'a mut PlayerControllerBase<'a,Self> {
        
        todo!();
        /*
            return *getIvar<PlayerControllerBase*> (self, "owner");
        */
    }
    
    pub fn set_owner<'a>(
        self_: Id<Self>,
        p:     *mut PlayerControllerBase<'a,Self>)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", p);
        */
    }
    
    pub fn notification_received(
        self_:        Id<Self>,
        _1:           Sel,
        notification: *mut NSNotification)  {
        
        todo!();
        /*
            if ([notification.name isEqualToString: AVPlayerItemDidPlayToEndTimeNotification])
                    getOwner (self).playbackReachedEndTime();
        */
    }
}
