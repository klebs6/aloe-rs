crate::ix!();

/**
  | Scheduled parameter implementation:
  |
  */
pub type ParameterEventList = Vec<AudioUnitParameterEvent>;

/**
  | Private data members to discourage
  | hacking in subclasses
  |
  */
pub struct RenderCallback {
    render_notify:         AURenderCallback,
    render_notify_ref_con: *mut c_void,
}

impl PartialEq<RenderCallback> for RenderCallback {
    
    #[inline] fn eq(&self, other: &RenderCallback) -> bool {
        todo!();
        /*
            return this->mRenderNotify == other.mRenderNotify &&
                        this->mRenderNotifyRefCon == other.mRenderNotifyRefCon;
        */
    }
}

impl Eq for RenderCallback {}

impl RenderCallback {

    pub fn new(
        proc: AURenderCallback,
        ref_: *mut c_void) -> Self {
    
        todo!();
        /*
        : render_notify(proc),
        : render_notify_ref_con(ref),

        
        */
    }
}

pub type RenderCallbackList = ThreadSafeList<RenderCallback>;

pub fn sort_parameter_event_list(
        ev1: &AudioUnitParameterEvent,
        ev2: &AudioUnitParameterEvent) -> bool {
    
    todo!();
        /*
            int offset1 = ev1.eventType == kParameterEvent_Immediate ?  ev1.eventValues.immediate.bufferOffset : ev1.eventValues.ramp.startBufferOffset;
        int offset2 = ev2.eventType == kParameterEvent_Immediate ?  ev2.eventValues.immediate.bufferOffset : ev2.eventValues.ramp.startBufferOffset;

        if(offset1 < offset2) return true;
        return false;
        */
}


