crate::ix!();

/**
  | IPtr - Smart pointer template class.
  | \ingroup pluginBase
  | 
  | - can be used as an I* pointer
  | 
  | - handles refCount of the interface
  | 
  | - Usage example:
  | 
  | -----------
  | @code
  | 
  | IPtr<IPath> path (sharedPath);
  | if (path)
  |     path->ascend ();
  |
  */
pub struct IPtr<I> {
    ptr: *mut I,
}

impl<I> IPtr<I> {
    
    #[inline] pub fn into_inner(self) -> I {
        todo!();
        /*
            return ptr;
        */
    }
}

impl<I> Deref for IPtr<I> {

    type Target = I;

    /**
      | act as I*
      |
      */
    fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return ptr;
        */
    }
}

impl<I> Drop for IPtr<I> {

    fn drop(&mut self) {
        todo!();
        /*
            if (ptr) 
        {
            ptr->release ();
            ptr = nullptr;  //TODO_CORE: how much does this cost? is this something hiding for us?
        }
        */
    }
}

impl<I> Default for IPtr<I> {

    fn default() -> Self {
    
        todo!();
        /*
        : ptr(0),

        
        */
    }
}
    
impl<I> IPtr<I> {

    pub fn new_from_iptr<T>(other: &IPtr<T>) -> Self {
    
        todo!();
        /*
        : ptr(other.get ()),

            if (ptr)
                ptr->addRef ();
        */
    }
    
    #[inline] pub fn assign_from_iptr<T>(&mut self, other: &IPtr<T>) -> &mut IPtr<I> {
    
        todo!();
        /*
            operator= (other.get ());
            return *this;
        */
    }
    
    #[inline] pub fn get(&self) -> *mut I {
        
        todo!();
        /*
            return ptr;
        */
    }

    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    pub fn new_from_move(move_ptr: IPtr<I>) -> Self {
    
        todo!();
        /*
        : ptr(movePtr.take ()),

        
        */
    }
    
    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    pub fn new_from_move_ptr<T>(move_ptr: IPtr<T>) -> Self {
    
        todo!();
        /*
        : ptr(movePtr.take ()),

        
        */
    }
    
    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    #[inline] pub fn assign_from_move(&mut self, move_ptr: IPtr<I>) -> &mut IPtr {
        
        todo!();
        /*
            if (ptr)
                ptr->release ();

            ptr = movePtr.take ();
            return *this;
        */
    }
    
    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    #[inline] pub fn assign_from_move_other<T>(&mut self, move_ptr: IPtr<T>) -> &mut IPtr {
        
        todo!();
        /*
            if (ptr)
                ptr->release ();
            
            ptr = movePtr.take ();
            return *this;
        */
    }
    
    #[inline] pub fn reset(&mut self, obj: *mut I)  {

        todo!();
        /*
            if (ptr)
                ptr->release();
            ptr = obj;
        */
    }
    
    pub fn take(&mut self) -> *mut I {
        
        todo!();
        /*
            I* out = ptr; 
            ptr = nullptr; 
            return out;
        */
    }
    
    pub fn adopt<T>(obj: *mut T) -> IPtr<T> {
    
        todo!();
        /*
            return IPtr<T> (obj, false);
        */
    }
    
    pub fn new_from_raw_with_add_ref(
        ptr:     *mut I,
        add_ref: Option<bool>

    ) -> Self {

        let add_ref: bool = add_ref.unwrap_or(true);
    
        todo!();
        /*
        : ptr(_ptr),

            if (ptr && addRef)
            ptr->addRef ();
        */
    }
    
    pub fn new_from_other(other: &IPtr<I>) -> Self {
    
        todo!();
        /*
        : ptr(other.ptr),

            if (ptr)
            ptr->addRef ();
        */
    }
    
    #[inline] pub fn assign_from_raw(&mut self, ptr: *mut I) -> *mut I {
        
        todo!();
        /*
            if (_ptr != ptr)
        {
            if (ptr)
                ptr->release ();
            ptr = _ptr;
            if (ptr)
                ptr->addRef ();
        }
        return ptr;
        */
    }
    
    #[inline] pub fn assign_from_iptr_ref(&mut self, ptr: &IPtr<I>) -> &mut IPtr<I> {
        
        todo!();
        /*
            operator= (_ptr.ptr);
        return *this;
        */
    }
}
