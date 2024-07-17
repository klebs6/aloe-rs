crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidHandler {
    native_handler: GlobalRef,
}

#[cfg(target_os="android")]
aloe_declare_singleton!{
    AndroidHandler, false
}

#[cfg(target_os="android")]
aloe_implement_singleton!{
    AndroidHandler
}

#[cfg(target_os="android")]
impl Default for AndroidHandler {
    
    fn default() -> Self {
        todo!();
        /*


            : nativeHandler (LocalRef<jobject> (getEnv()->NewObject (AndroidHandler, AndroidHandler.constructor))
        */
    }
}

#[cfg(target_os="android")]
impl Drop for AndroidHandler {
    fn drop(&mut self) {
        todo!();
        /*      clearSingletonInstance();  */
    }
}

#[cfg(target_os="android")]
impl AndroidHandler {

    pub fn post(&mut self, runnable: jobject) -> bool {
        
        todo!();
        /*
            return (getEnv()->CallBooleanMethod (nativeHandler.get(), AndroidHandler.post, runnable) != 0);
        */
    }
}
