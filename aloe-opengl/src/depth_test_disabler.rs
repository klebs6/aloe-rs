crate::ix!();

pub struct DepthTestDisabler {
    was_enabled: GLboolean,
}

impl Default for DepthTestDisabler {
    
    fn default() -> Self {
        todo!();
        /*


            glGetBooleanv (GL_DEPTH_TEST, &wasEnabled);

            if (wasEnabled)
                glDisable (GL_DEPTH_TEST);
        */
    }
}

impl Drop for DepthTestDisabler {

    fn drop(&mut self) {
        todo!();
        /*
            if (wasEnabled)
                glEnable (GL_DEPTH_TEST);
        */
    }
}
