crate::ix!();

/**
  | Some life-time and type management
  | of OpenSL objects
  |
  */
#[cfg(target_os="android")]
#[derive(Default)]
pub struct SlObjectRef {
    cb: ReferenceCountedObjectPtr<SLObjectRefControlBlock>,
}

#[cfg(target_os="android")]
impl Deref for SlObjectRef {
    type Target = SLObjectItf_;
    
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return *cb->ptr.get();
        */
    }
}

#[cfg(target_os="android")]
impl Into<SLObjectItf> for SlObjectRef {

    #[inline] fn into(self) -> SLObjectItf {
        todo!();
        /*
            return (cb == nullptr ? nullptr :  cb->ptr.get());
        */
    }
}

#[cfg(target_os="android")]
impl SlObjectRef {
    
    #[inline] pub fn eq_nullptr(&self) -> bool {
        todo!();
        /*
            return (cb == nullptr || cb->ptr == nullptr);
        */
    }
}

#[cfg(target_os="android")]
impl From<&SlObjectRef> for SlObjectRef {

    fn from(obj: &SlObjectRef) -> Self {
    
        todo!();
        /*
        : cb(obj.cb),

        
        */
    }
}

#[cfg(target_os="android")]
impl From<SlObjectRef> for SlObjectRef {
    
    fn from(obj: SlObjectRef) -> Self {
    
        todo!();
        /*
        : cb(std::move (obj.cb)),

            obj.cb = nullptr;
        */
    }
}
    
#[cfg(target_os="android")]
impl From<SLObjectItf> for SlObjectRef {

    fn from(o: SLObjectItf) -> Self {
    
        todo!();
        /*


            : cb (new SLObjectRefControlBlock (o))
        */
    }
}
