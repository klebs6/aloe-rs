crate::ix!();

pub struct AloePlayerItemPreparationStatusObserverClass {
    base: ObjCClass<NSObject>,
}

impl Default for AloePlayerItemPreparationStatusObserverClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("AloePlayerItemStatusObserverClass_")
                    ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                    addMethod (@selector (observeValueForKeyPath:ofObject:change:context:), valueChanged, "v@:@@@?");
                    ALOE_END_IGNORE_WARNINGS_GCC_LIKE

                    addIvar<PlayerAsyncInitialiser*> ("owner");

                    registerClass();
        */
    }
}

impl AloePlayerItemPreparationStatusObserverClass {

    pub fn get_owner<'a>(self_: Id<Self>) -> &'a mut PlayerAsyncInitialiser<'a> {
        
        todo!();
        /*
            return *getIvar<PlayerAsyncInitialiser*> (self, "owner");
        */
    }
    
    pub fn set_owner(
        self_: Id<Self>,
        p:     *mut PlayerAsyncInitialiser)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", p);
        */
    }
    
    pub fn value_changed(
        self_:   Id<Self>,
        _1:      Sel,
        _2:      *mut NSString,
        object:  Id<Self>,
        change:  *mut NSDictionary<*mut NSString,Id<NSObject>>,
        context: *mut c_void)  {
        
        todo!();
        /*
            auto& owner = getOwner (self);

                    if (context == &owner)
                    {
                        auto* playerItem = (AVPlayerItem*) object;
                        auto* urlAsset = (AVURLAsset*) playerItem.asset;

                        Url url (nsStringToAloe (urlAsset.Url.absoluteString));
                        auto oldStatus = [change[NSKeyValueChangeOldKey] intValue];
                        auto newStatus = [change[NSKeyValueChangeNewKey] intValue];

                        // Ignore spurious notifications
                        if (oldStatus == newStatus)
                            return;

                        if (newStatus == AVPlayerItemStatusFailed)
                        {
                            auto errorMessage = playerItem.error != nil
                                              ? nsStringToAloe (playerItem.error.localizedDescription)
                                              : String();

                            owner.notifyOwnerPreparationFinished (url, Result::fail (errorMessage), nullptr);
                        }
                        else if (newStatus == AVPlayerItemStatusReadyToPlay)
                        {
                            owner.notifyOwnerPreparationFinished (url, Result::ok(), owner.player.get());
                        }
                        else
                        {
                            jassertfalse;
                        }
                    }
        */
    }
}
