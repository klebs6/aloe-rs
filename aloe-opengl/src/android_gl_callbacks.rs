crate::ix!();

pub struct AndroidGLCallbacks {

}

impl AndroidGLCallbacks {

    pub fn attached_to_window(&mut self, 
        _0:   *mut JNIEnv,
        this: jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* nativeContext = reinterpret_cast<OpenGLContext::NativeContext*> (host))
            nativeContext->attachedToWindow();
        */
    }
    
    pub fn detached_from_window(&mut self, 
        _0:   *mut JNIEnv,
        this: jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* nativeContext = reinterpret_cast<OpenGLContext::NativeContext*> (host))
            nativeContext->detachedFromWindow();
        */
    }
    
    pub fn dispatch_draw(&mut self, 
        _0:     *mut JNIEnv,
        this:   jobject,
        host:   i64,
        canvas: jobject)  {
        
        todo!();
        /*
            if (auto* nativeContext = reinterpret_cast<OpenGLContext::NativeContext*> (host))
            nativeContext->dispatchDraw (canvas);
        */
    }
}

