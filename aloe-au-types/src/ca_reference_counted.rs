crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAReferenceCounted.h]

/**
  | base class for reference-counted objects
  |
  */
pub struct CAReferenceCounted {
    ref_count: i32,
}

impl Default for CAReferenceCounted {
    
    fn default() -> Self {
        todo!();
        /*
        : ref_count(1),

        
        */
    }
}

pub trait ReleaseObject {

    fn release_object(&mut self);
}

impl ReleaseObject for CAReferenceCounted {

    fn release_object(&mut self)  {
        
        todo!();
        /*
            delete this;
        */
    }
}

impl CAReferenceCounted {

    pub fn retain(&mut self)  {
        
        todo!();
        /*
            CAAtomicIncrement32(&mRefCount);
        */
    }
    
    pub fn release(&mut self)  {
        
        todo!();
        /*
            SInt32 rc = CAAtomicDecrement32(&mRefCount);
                    if (rc == 0) {
                        releaseObject();
                    }
        */
    }
    
    pub fn get_reference_count(&self) -> i32 {
        
        todo!();
        /*
            return mRefCount;
        */
    }
}
