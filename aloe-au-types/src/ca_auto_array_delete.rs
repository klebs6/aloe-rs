crate::ix!();

///------------------------------
pub struct CAAutoArrayDelete<T> {
    ptr: *mut T,
}

impl<T> Default for CAAutoArrayDelete<T> {
    
    fn default() -> Self {
        todo!();
        /*
        : ptr(0),

        
        */
    }
}

impl<T> Drop for CAAutoArrayDelete<T> {

    fn drop(&mut self) {
        todo!();
        /*
            free();
        */
    }
}

impl<T> Deref for CAAutoArrayDelete<T> {

    type Target = T;
    
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return ptr_;
        */
    }
}

impl<T> PartialEq<CAAutoArrayDelete<T>> for CAAutoArrayDelete<T> {
    
    #[inline] fn eq(&self, other: &CAAutoArrayDelete<T>) -> bool {
        todo!();
        /*
            return ptr_ == that.ptr_;
        */
    }
}

impl<T> Eq for CAAutoArrayDelete<T> {}

impl<T> PartialEq<T> for CAAutoArrayDelete<T> {
    
    #[inline] fn eq(&self, other: &T) -> bool {
        todo!();
        /*
            return ptr_ == ptr;
        */
    }
}

impl<T,U> Into<CAPtrRef<U>> for CAAutoArrayDelete<T> {

    #[inline] fn into(self) -> CAPtrRef<U> {
        todo!();
        /*
            return CAPtrRef<U>(release());
        */
    }
}

impl<T> CAAutoArrayDelete<T> {

    #[inline] pub fn into_ca_auto_array_delete_u<U>(self) -> CAAutoArrayDelete<U> {
        todo!();
        /*
            return CAAutoFree<U>(release());
        */
    }
    
    #[inline] pub fn into_type(self) -> T {
        todo!();
        /*
            return ptr_;
        */
    }

    pub fn new_from_raw(ptr: *mut T) -> Self {
    
        todo!();
        /*
        : ptr(ptr),

        
        */
    }

    /**
       take ownership
      */
    pub fn new_from_delete_u<U>(that: &mut CAAutoArrayDelete<U>) -> Self {
    
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
    pub fn new_from_delete_t(that: &mut CAAutoArrayDelete<T>) -> Self {
    
        todo!();
        /*
        : ptr(that.release()),

        
        */
    }

    /**
       this becomes an ambiguous call if n == 0
      */
    pub fn new(n: usize) -> Self {
    
        todo!();
        /*


            : ptr_(new T[n])
        */
    }
    
    pub fn alloc(&mut self, num_items: usize)  {
        
        todo!();
        /*
            free();
            ptr_ = new T [numItems];
        */
    }
    
    pub fn assign_from_u<U>(&mut self, that: &mut CAAutoArrayDelete<U>) -> &mut CAAutoArrayDelete<T> {
    
        todo!();
        /*
            set(that.release());    // take ownership
            return *this;
        */
    }
    
    pub fn assign_from_delete_t(&mut self, that: &mut CAAutoArrayDelete<T>) -> &mut CAAutoArrayDelete<T> {
        
        todo!();
        /*
            set(that.release());    // take ownership
            return *this;
        */
    }
    
    pub fn assign_from_raw_t(&mut self, ptr: *mut T) -> &mut CAAutoArrayDelete<T> {
        
        todo!();
        /*
            set(ptr);
            return *this;
        */
    }
    
    pub fn assign_from_raw_u<U>(&mut self, ptr: *mut U) -> &mut CAAutoArrayDelete<T> {
    
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
                delete [] ptr_;
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
    
    pub fn assign_from_ca_ptr_ref_t(&mut self, ref_: CAPtrRef<T>) -> &mut CAAutoArrayDelete<T> {
        
        todo!();
        /*
            set(ref.ptr_);
            return *this;
        */
    }
}
