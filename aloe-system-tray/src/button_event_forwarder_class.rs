crate::ix!();

pub struct ButtonEventForwarderClass {
    base: ObjCClass<NSObject>,
}

impl Default for ButtonEventForwarderClass {
    
    fn default() -> Self {
        todo!();
        /*
            : ObjCClass<NSObject> ("ALOEButtonEventForwarderClass_")
                addIvar<ButtonBasedStatusItem*> ("owner");

                addMethod (@selector (handleEvent:), handleEvent, "v@:@");

                registerClass();
        */
    }
}

impl ButtonEventForwarderClass {

    pub fn get_owner<'a,StatusItem:NSStatusItem,ImageType:NSImage>(self_: objc_id::Id<NSObject>) -> *mut ButtonBasedStatusItem<'a,StatusItem,ImageType> {
        
        todo!();
        /*
            return getIvar<ButtonBasedStatusItem*> (self, "owner");
        */
    }
    
    pub fn set_owner<StatusItem:NSStatusItem,ImageType:NSImage>(
        self_: objc_id::Id<NSObject>,
        owner: *mut ButtonBasedStatusItem<StatusItem,ImageType>)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", owner);
        */
    }
    
    pub fn handle_event(
        self_: objc_id::Id<NSObject>,
        _1:    objc::runtime::Sel,
        _2:    objc_id::Id<NSObject>)  {
        
        todo!();
        /*
            if (auto* owner = getOwner (self))
                    owner->handleEvent();
        */
    }
}
