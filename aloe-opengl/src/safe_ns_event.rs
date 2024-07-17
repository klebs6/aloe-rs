crate::ix!();

pub struct SafeNSEvent {
    obj: Id<objc::runtime::Object>,
}

impl SafeNSEvent {

    fn new(obj: Id<objc::runtime::Object>) -> Result<Self, &'static str> {

        unsafe {

            let class: *const objc::runtime::Class = objc::class!(NSEvent);

            let is_kind: bool = objc::msg_send![obj, isKindOfClass: class];

            if is_kind {
                Ok(Self { obj })
            } else {
                Err("The object is not an instance of NSEvent")
            }
        }
    }
    
    // Define methods that operate on `obj` as if it were an NSOpenGLContext
    // ...
}

