crate::ix!();

pub struct AloePlayerStatusObserverClass {
    base: ObjCClass<NSObject>,
}

impl Default for AloePlayerStatusObserverClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("AloePlayerStatusObserverClass_")
                ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                addMethod (@selector (observeValueForKeyPath:ofObject:change:context:), valueChanged, "v@:@@@?");
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE

                addIvar<PlayerAsyncInitialiser*> ("owner");

                registerClass();
        */
    }
}

impl AloePlayerStatusObserverClass {
    
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
    
    pub fn value_changed(
        self_:    Id<Self>,
        _1:       Sel,
        key_path: *mut NSString,
        _3:       Id<Self>,
        change:   *mut NSDictionary<*mut NSString,Id<Self>>,
        _5:       *mut c_void)  {
        
        todo!();
        /*
            auto& owner = getOwner (self);

                if ([keyPath isEqualToString: nsStringLiteral ("rate")])
                {
                    auto oldRate = [change[NSKeyValueChangeOldKey] floatValue];
                    auto newRate = [change[NSKeyValueChangeNewKey] floatValue];

                    if (oldRate == 0 && newRate != 0)
                        owner.playbackStarted();
                    else if (oldRate != 0 && newRate == 0)
                        owner.playbackStopped();
                }
                else if ([keyPath isEqualToString: nsStringLiteral ("status")])
                {
                    auto status = [change[NSKeyValueChangeNewKey] intValue];

                    if (status == AVPlayerStatusFailed)
                        owner.errorOccurred();
                }
        */
    }
}

