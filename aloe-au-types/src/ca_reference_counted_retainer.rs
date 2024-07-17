crate::ix!();

pub struct CAReferenceCountedRetainer {
    object: *mut CAReferenceCounted,
}

impl Drop for CAReferenceCountedRetainer {

    fn drop(&mut self) {
        todo!();
        /*
            mObject->release();
        */
    }
}

impl CAReferenceCountedRetainer {

    pub fn new(obj: *mut CAReferenceCounted) -> Self {
    
        todo!();
        /*
        : object(obj),

            mObject->retain();
        */
    }
}
