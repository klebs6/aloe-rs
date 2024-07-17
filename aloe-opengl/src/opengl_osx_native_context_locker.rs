crate::ix!();

#[cfg(target_os="macos")]
pub struct OpenGLContextNativeContextLocker {
    cgl_context: CGLContextObj,
}

#[cfg(target_os="macos")]
impl Drop for OpenGLContextNativeContextLocker {

    fn drop(&mut self) {
        todo!();
        /*
            CGLUnlockContext (cglContext);
        */
    }
}

#[cfg(target_os="macos")]
impl OpenGLContextNativeContextLocker {

    pub fn new(nc: &mut OpenGLContextNativeContext) -> Self {
    
        todo!();
        /*


            : cglContext ((CGLContextObj) [nc.renderContext CGLContextObj])
                CGLLockContext (cglContext);
        */
    }
}
