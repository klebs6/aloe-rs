crate::ix!();

pub struct AUBasePropertyListener {
    propertyid:       AudioUnitPropertyID,
    listener_proc:    AudioUnitPropertyListenerProc,
    listener_ref_con: *mut c_void,
}

pub type AUBasePropertyListeners = Vec<AUBasePropertyListener>;

