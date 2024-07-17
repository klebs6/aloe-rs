crate::ix!();

pub struct CAAutoDelete<T> {
    ptr: *mut T,
}

impl<T> Default for CAAutoDelete<T> {
    
    fn default() -> Self {
        todo!();
        /*
        : ptr(0),

        
        */
    }
}

impl<T> Drop for CAAutoDelete<T> {
    fn drop(&mut self) {
        todo!();
        /*
            free();
        */
    }
}

impl<T> Deref for CAAutoDelete<T> {
    type Target = T;

    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return ptr_;
        */
    }
}

impl<T> CAAutoDelete<T> {
    
    #[inline] pub fn into_type(self) -> T {
        todo!();
        /*
            return ptr_;
        */
    }
}

impl<T> PartialEq<CAAutoDelete<T>> for CAAutoDelete<T> {
    
    #[inline] fn eq(&self, other: &CAAutoDelete<T>) -> bool {
        todo!();
        /*
            return ptr_ == that.ptr_;
        */
    }
}

impl<T> Eq for CAAutoDelete<T> {}

impl<T> PartialEq<T> for CAAutoDelete<T> {
    
    #[inline] fn eq(&self, other: &T) -> bool {
        todo!();
        /*
            return ptr_ == ptr;
        */
    }
}

impl<T,U> Into<CAPtrRef<U>> for CAAutoDelete<T> {
    
    #[inline] fn into(self) -> CAPtrRef<U> {
        todo!();
        /*
            return CAPtrRef<U>(release());
        */
    }
}

impl<T,U> Into<CAAutoFree<U>> for CAAutoDelete<T> {
    
    #[inline] fn into(self) -> CAAutoFree<U> {
        todo!();
        /*
            return CAAutoFree<U>(release());
        */
    }
}

impl<T> CAAutoDelete<T> {

    pub fn new_from_raw(ptr: *mut T) -> Self {
    
        todo!();
        /*
        : ptr(ptr),

        
        */
    }

    /**
       take ownership
      */
    pub fn new_from_ca_auto_delete_u<U>(that: &mut CAAutoDelete<U>) -> Self {
    
        todo!();
        /*
        : ptr(that.release()),

        
        */
    }

    /**
      | C++ std says: a template constructor is
      | never a copy constructor
      |
      | take ownership
      */
    pub fn new_from_ca_auto_delete_t(that: &mut CAAutoDelete<T>) -> Self {
    
        todo!();
        /*
        : ptr(that.release()),

        
        */
    }
    
    pub fn assign_from_ca_auto_delete_u<U>(&mut self, that: &mut CAAutoDelete<U>) -> &mut CAAutoDelete<T> {
    
        todo!();
        /*
            set(that.release());    // take ownership
            return *this;
        */
    }
    
    pub fn assign_from_ca_auto_delete_t(&mut self, that: &mut CAAutoDelete<T>) -> &mut CAAutoDelete<T> {
        
        todo!();
        /*
            set(that.release());    // take ownership
            return *this;
        */
    }
    
    pub fn assign_from_raw(&mut self, ptr: *mut T) -> &mut CAAutoDelete<T> {
        
        todo!();
        /*
            set(ptr);
            return *this;
        */
    }
    
    pub fn assign_from_raw_u<U>(&mut self, ptr: *mut U) -> &mut CAAutoDelete<T> {
    
        todo!();
        /*
            set(ptr);
            return *this;
        */
    }
    
    pub fn invoke(&self) -> *mut T {
        
        todo!();
        /*
            return ptr_;
        */
    }
    
    pub fn get(&self) -> *mut T {
        
        todo!();
        /*
            return ptr_;
        */
    }
    
    pub fn release(&mut self) -> *mut T {
        
        todo!();
        /*
            // release ownership
            T* result = ptr_;
            ptr_ = 0;
            return result;
        */
    }
    
    pub fn set(&mut self, ptr: *mut T)  {
        
        todo!();
        /*
            if (ptr != ptr_)
            {
                delete ptr_;
                ptr_ = ptr;
            }
        */
    }
    
    pub fn free(&mut self)  {
        
        todo!();
        /*
            set(0);
        */
    }

    /**
      | automatic conversions to allow assignment
      | from results of functions.
      |
      | hard to explain. see auto_ptr
      | implementation and/or Josuttis' STL book.
      */
    pub fn new_from_ca_ptr_ref_t(ref_: CAPtrRef<T>) -> Self {
    
        todo!();
        /*
        : ptr(ref.ptr_),

        
        */
    }
    
    pub fn assign_from(&mut self, ref_: CAPtrRef<T>) -> &mut CAAutoDelete<T> {
        
        todo!();
        /*
            set(ref.ptr_);
            return *this;
        */
    }
}
