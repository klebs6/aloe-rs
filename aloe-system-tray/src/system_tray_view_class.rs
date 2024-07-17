crate::ix!();

pub struct SystemTrayViewClass<Control:NSControl> {
    base: ObjCClass<Box<Control>>,
}

impl<Control:NSControl> Default for SystemTrayViewClass<Control> {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSControl> ("ALOESystemTrayView_")
                addIvar<ViewBasedStatusItem*> ("owner");
                addIvar<NSImage*> ("image");

                addMethod (@selector (mouseDown:),      handleEventDown, "v@:@");
                addMethod (@selector (rightMouseDown:), handleEventDown, "v@:@");
                addMethod (@selector (drawRect:),       drawRect,        "v@:@");

                ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
                addMethod (@selector (frameChanged:),   frameChanged,    "v@:@");
                ALOE_END_IGNORE_WARNINGS_GCC_LIKE

                registerClass();
        */
    }
}

impl<Control:NSControl> SystemTrayViewClass<Control> {

    pub fn get_owner<'a,StatusItem:NSStatusItem,ImageType:NSImage>(self_: objc_id::Id<NSObject>) -> *mut ViewBasedStatusItem<'a,Control,StatusItem,ImageType> {
        
        todo!();
        /*
            return getIvar<ViewBasedStatusItem*> (self, "owner");
        */
    }
    
    pub fn get_image<ImageType: NSImage>(
        self_: objc_id::Id<NSObject>

    ) -> *mut ImageType {
        
        todo!();
        /*
            return getIvar<NSImage*> (self, "image");
        */
    }
    
    pub fn set_owner<StatusItem:NSStatusItem,ImageType:NSImage>(
        self_: objc_id::Id<NSObject>,
        owner: *mut ViewBasedStatusItem<Control,StatusItem,ImageType>)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", owner);
        */
    }
    
    pub fn set_image<ImageType: NSImage>(
        self_: objc_id::Id<NSObject>,
        image: Box<ImageType>

    ) {
        
        todo!();
        /*
            object_setInstanceVariable (self, "image", image);
        */
    }
    
    pub fn frame_changed(
        self_: objc_id::Id<NSObject>,
        _1:    objc::runtime::Sel,
        _2:    *mut NSNotification)  {
        
        todo!();
        /*
            if (auto* owner = getOwner (self))
                {
                    NSRect r = [[[owner->statusItem.get() view] window] frame];
                    NSRect sr = [[[NSScreen screens] objectAtIndex: 0] frame];
                    r.origin.y = sr.size.height - r.origin.y - r.size.height;
                    owner->owner.setBounds (convertToRectInt (r));
                }
        */
    }
    
    pub fn handle_event_down<EventType: NSEvent>(
        self_: objc_id::Id<NSObject>,
        _1:    objc::runtime::Sel,
        e:     Box<EventType>
    ) {
        
        todo!();
        /*
            if (auto* owner = getOwner (self))
                    owner->handleStatusItemAction (e);
        */
    }
    
    pub fn draw_rect(
        self_: objc_id::Id<NSObject>,
        _1:    objc::runtime::Sel,
        _2:    NSRect)  {
        
        todo!();
        /*
            NSRect bounds = [self bounds];

                if (auto* owner = getOwner (self))
                    [owner->statusItem.get() drawStatusBarBackgroundInRect: bounds
                                                             withHighlight: owner->isHighlighted];

                if (NSImage* const im = getImage (self))
                {
                    NSSize imageSize = [im size];

                    [im drawInRect: NSMakeRect (bounds.origin.x + ((bounds.size.width  - imageSize.width)  / 2.0f),
                                                bounds.origin.y + ((bounds.size.height - imageSize.height) / 2.0f),
                                                imageSize.width, imageSize.height)
                          fromRect: NSZeroRect
                         operation: NSCompositingOperationSourceOver
                          fraction: 1.0f];
                }
        */
    }
}
