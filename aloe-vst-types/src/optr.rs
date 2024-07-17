crate::ix!();

/**
  | OPtr - "owning" smart pointer used for
  | newly created FObjects. \ingroup pluginBase
  | 
  | FUnknown implementations are supposed
  | to have a refCount of 1 right after creation.
  | 
  | So using an IPtr on newly created objects
  | would lead to a leak.
  | 
  | Instead the OPtr can be used in this case.
  | \n
  | 
  | Example:
  | 
  | -----------
  | @code
  | 
  | OPtr<IPath> path = FHostCreate (IPath, hostClasses);
  | // no release is needed...
  | 
  | The assignment operator takes ownership
  | of a new object and releases the old.
  | 
  | So its safe to write:
  | ----------
  | @code
  | 
  | OPtr<IPath> path = FHostCreate (IPath, hostClasses);
  | path = FHostCreate (IPath, hostClasses);
  | path = 0;
  | 
  | This is the difference to using an IPtr
  | with addRef=false.
  | ----------
  | @code
  | 
  | // DONT DO THIS:
  | IPtr<IPath> path (FHostCreate (IPath, hostClasses), false);
  | path = FHostCreate (IPath, hostClasses);
  | path = 0;
  | 
  | This will lead to a leak!
  |
  */
pub struct OPtr<I> {
    base: IPtr<I>,
}

impl<I> OPtr<I> {

    pub fn new_from_raw(p: *mut I) -> Self {
    
        todo!();
        /*


            : IPtr<I> (p, false)
        */
    }
    
    pub fn new_from_iptr(p: &IPtr<I>) -> Self {
    
        todo!();
        /*
            : IPtr<I> (p)
        */
    }
    
    pub fn new_from_optr(p: &OPtr<I>) -> Self {
    
        todo!();
        /*


            : IPtr<I> (p)
        */
    }
    
    #[inline] pub fn assign_from(&mut self, ptr: *mut I) -> *mut I {
        
        todo!();
        /*
            if (_ptr != this->ptr)
            {
                if (this->ptr)
                    this->ptr->release ();
                this->ptr = _ptr;
            }
            return this->ptr;
        */
    }
}

