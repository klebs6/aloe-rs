crate::ix!();

#[cfg(target_os="android")]
#[derive(Default)]
pub struct SLObjectRefControlBlock {
    base: ReferenceCountedObject,
    ptr:  Option<Box<*const SLObjectItf_,SLObjectItfFree>>,
}

#[cfg(target_os="android")]
impl SLObjectRefControlBlock {

    pub fn new(o: SLObjectItf) -> Self {
    
        todo!();
        /*
        : ptr(o),

        
        */
    }
}
