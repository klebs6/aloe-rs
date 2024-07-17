crate::ix!();

#[cfg(target_os="android")]
pub fn destroy_object<SLObjectType>(object: SLObjectType)  {

    todo!();
    /*
        if (object != nullptr && *object != nullptr)
            (*object)->Destroy (object);
    */
}


#[cfg(target_os="android")]
pub type CreateEngineFunc = fn(
    _0: *mut SLObjectItf, 
    _1: u32, 
    _2: *const SLEngineOption, 
    _3: u32, 
    _4: *const SLInterfaceID, 
    _5: *const bool) -> SLresult;




#[cfg(target_os="android")]
pub fn is_open_sl_available() -> bool {
    
    todo!();
    /*
        return OpenSLAudioDeviceType::isOpenSLAvailable();
    */
}
