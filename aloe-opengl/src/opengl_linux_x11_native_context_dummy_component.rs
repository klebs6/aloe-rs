crate::ix!();

#[cfg(target_os="linux")]
pub struct OpenGLContextNativeContextDummyComponent {
    base:   Component,
    native: &mut OpenGLContext::OpenGLContextNativeContext,
}

#[cfg(target_os="linux")]
impl OpenGLContextNativeContextDummyComponent {

    pub fn new(native_parent_context: &mut OpenGLContext::OpenGLContextNativeContext) -> Self {

        todo!();
        /*
           : native(nativeParentContext),


           */
    }

    pub fn handle_command_message(&mut self, command_id: i32)  {

        todo!();
        /*
           if (commandId == 0)
           native.triggerRepaint();
           */
    }
}
