crate::ix!();

pub struct SafeNSOpenGLView {
    obj: Id<objc::runtime::Object>,
}

impl SafeNSOpenGLView {

    fn new(obj: Id<objc::runtime::Object>) -> Result<Self, &'static str> {

        unsafe {

            let class: *const objc::runtime::Class = objc::class!(NSOpenGLView);

            let is_kind: bool = objc::msg_send![obj, isKindOfClass: class];

            if is_kind {
                Ok(Self { obj })
            } else {
                Err("The object is not an instance of NSOpenGLView")
            }
        }
    }
    
    // Define methods that operate on `obj` as if it were an NSOpenGLView
    // ...
}
