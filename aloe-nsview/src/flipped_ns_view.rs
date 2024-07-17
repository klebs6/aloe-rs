crate::ix!();

#[cfg(target_os="macos")]
pub struct FlippedNSView {
    base: ObjCClass<NSView>,
}

#[cfg(target_os="macos")]
impl Default for FlippedNSView {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass ("AloeFlippedNSView_")
                addIvar<NSViewComponentWithParent*> ("owner");

                addMethod (@selector (isFlipped),      isFlipped,     "c@:");
                addMethod (@selector (isOpaque),       isOpaque,      "c@:");
                addMethod (@selector (didAddSubview:), didAddSubview, "v@:@");

                registerClass();
        */
    }
}

#[cfg(target_os="macos")]
impl FlippedNSView {

    pub fn is_flipped(_0: objc_id::Id<NSObject>, _1: objc::runtime::Sel) -> bool {
        
        todo!();
        /*
            return YES;
        */
    }
    
    pub fn is_opaque(_0: objc_id::Id<NSObject>, _1: objc::runtime::Sel) -> bool {
        
        todo!();
        /*
            return YES;
        */
    }
    
    pub fn nudge(self_: *mut NSView)  {
        
        todo!();
        /*
            if (auto* owner = getIvar<NSViewComponentWithParent*> (self, "owner"))
                    if (owner->wantsNudge == WantsNudge::yes)
                        owner->triggerAsyncUpdate();
        */
    }
    
    pub fn view_did_unhide(
        self_: *mut NSView,
        _1:    objc::runtime::Sel)  {
        
        todo!();
        /*
            nudge (self);
        */
    }
    
    pub fn did_add_subview(
        self_: *mut NSView,
        _1:    objc::runtime::Sel,
        _2:    *mut NSView)  {
        
        todo!();
        /*
            nudge (self);
        */
    }
    
    pub fn view_did_move_to_superview(
        self_: *mut NSView,
        _1:    objc::runtime::Sel)  {
        
        todo!();
        /*
            nudge (self);
        */
    }
    
    pub fn view_did_move_to_window(
        self_: *mut NSView,
        _1:    objc::runtime::Sel)  {
        
        todo!();
        /*
            nudge (self);
        */
    }
}
