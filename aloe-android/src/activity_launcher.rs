crate::ix!();

#[cfg(target_os="android")]
pub struct ActivityLauncher {
    base:         FragmentOverlay,
    intent:       GlobalRef,
    request_code: i32,
    callback:     fn(
            _0: i32,
            _1: i32,
            _2: LocalRef<jobject>
    ) -> (),
}

#[cfg(target_os="android")]
impl ActivityLauncher {

    pub fn new(
        intent_to_use:       &LocalRef<jobject>,
        request_code_to_use: i32,
        callback_to_use:     fn(
                _0: i32,
                _1: i32,
                _2: LocalRef<jobject>
        ) -> ()) -> Self {
    
        todo!();
        /*


            : intent (intentToUse), requestCode (requestCodeToUse), callback (std::move (callbackToUse))
        */
    }
    
    pub fn on_start(&mut self)  {
        
        todo!();
        /*
            getEnv()->CallVoidMethod (getNativeHandle(), AndroidFragment.startActivityForResult,
                                      intent.get(), requestCode);
        */
    }
    
    pub fn on_activity_result(&mut self, 
        activity_request_code: i32,
        result_code:           i32,
        data:                  LocalRef<jobject>)  {
        
        todo!();
        /*
            if (callback)
                callback (activityRequestCode, resultCode, std::move (data));

            getEnv()->CallVoidMethod (getNativeHandle(), AloeFragmentOverlay.close);
            delete this;
        */
    }
}
