crate::ix!();

#[cfg(target_os="macos")]
pub struct OpenGLContextNativeContextMouseForwardingNSOpenGLViewClass {
    base: SafeNSOpenGLView,
}

#[cfg(target_os="macos")]
impl Default for OpenGLContextNativeContextMouseForwardingNSOpenGLViewClass {
    
    fn default() -> Self {
        todo!();
        /*
        : ObjCClass<NSOpenGLView> ("ALOEGLView_")
            addMethod (@selector (rightMouseDown:),      rightMouseDown,     "v@:@");
            addMethod (@selector (rightMouseUp:),        rightMouseUp,       "v@:@");
            addMethod (@selector (acceptsFirstMouse:),   acceptsFirstMouse,  "v@:@");

            registerClass();
        */
    }
}

#[cfg(target_os="macos")]
impl OpenGLContextNativeContextMouseForwardingNSOpenGLViewClass {
    
    pub fn right_mouse_down(
        self_: Id<Self>,
        _1:    objc::runtime::Sel,
        ev:    SafeNSEvent

    ) {
        
        todo!();
        /*
            [[(NSOpenGLView*) self superview] rightMouseDown: ev];
        */
    }
    
    pub fn right_mouse_up(
        self_: Id<Self>,
        _1:    objc::runtime::Sel,
        ev:    SafeNSEvent

    ) {
        
        todo!();
        /*
            [[(NSOpenGLView*) self superview] rightMouseUp:   ev];
        */
    }
    
    pub fn accepts_first_mouse(
        _0: Id<Self>,
        _1: objc::runtime::Sel,
        _2: SafeNSEvent

    ) -> bool {
        
        todo!();
        /*
            return YES;
        */
    }
}
