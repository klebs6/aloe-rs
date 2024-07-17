crate::ix!();

///--------------------------
pub struct CAAutoFree<T> {
    ptr: T,
}

impl<T> Default for CAAutoFree<T> {
    
    fn default() -> Self {
        todo!();
        /*
        : ptr(0),

        
        */
    }
}

impl<T> Drop for CAAutoFree<T> {

    fn drop(&mut self) {
        todo!();
        /*
            free();
        */
    }
}

impl<T> Deref for CAAutoFree<T> {

    type Target = T;
    
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return ptr_;
        */
    }
}

impl<T> CAAutoFree<T> {
    
    #[inline] pub fn into_type(self) -> T {
        todo!();
        /*
            return ptr_;
        */
    }
}

impl<T> PartialEq<CAAutoFree<T>> for CAAutoFree<T> {
    
    #[inline] fn eq(&self, other: &CAAutoFree<T>) -> bool {
        todo!();
        /*
            return ptr_ == that.ptr_;
        */
    }
}

impl<T> Eq for CAAutoFree<T> {}

impl<T> PartialEq<T> for CAAutoFree<T> {
    
    #[inline] fn eq(&self, other: &T) -> bool {
        todo!();
        /*
            return ptr_ == ptr;
        */
    }
}

impl<T,U> Into<CAPtrRef<U>> for CAAutoFree<T> {
    
    #[inline] fn into(self) -> CAPtrRef<U> {
        todo!();
        /*
            return CAPtrRef<U>(release());
        */
    }
}

impl<T> CAAutoFree<T> {

    #[inline] pub fn into_ca_auto_free_u<U>(self) -> CAAutoFree<U> {
        todo!();
        /*
            return CAAutoFree<U>(release());
        */
    }

    pub fn new(ptr: *mut T) -> Self {
    
        todo!();
        /*
        : ptr(ptr),
        */
    }

    /**
       take ownership
      */
    pub fn new_from_ca_auto_free_u<U>(that: &mut CAAutoFree<U>) -> Self {
    
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
    pub fn new_from_ca_auto_free_t(that: &mut CAAutoFree<T>) -> Self {
    
        todo!();
        /*
        : ptr(that.release()),
        */
    }

    /**
      | this becomes an ambiguous call if n == 0
      |
      */
    pub fn new_from_n_and_clear(
        n:     usize,
        clear: Option<bool>

    ) -> Self {

        let clear: bool = clear.unwrap_or(false);

        todo!();
        /*
        : ptr(0),

            size_t maxItems = ~size_t(0) / sizeof(T);
                if (n > maxItems)
                    throw std::bad_alloc();

                ptr_ = static_cast<T*>(clear ? CA_calloc(n, sizeof(T)) : CA_malloc(n * sizeof(T)));
        */
    }
    
    pub fn alloc(
        &mut self, 
        num_items: usize,
        clear:     Option<bool>

    ) {

        let clear: bool = clear.unwrap_or(false);

        todo!();
        /*
            size_t maxItems = ~size_t(0) / sizeof(T);
            if (numItems > maxItems) throw std::bad_alloc();

            free();
            ptr_ = static_cast<T*>(clear ? CA_calloc(numItems, sizeof(T)) : CA_malloc(numItems * sizeof(T)));
        */
    }
    
    pub fn alloc_bytes(
        &mut self, 
        num_bytes: usize,
        clear:     Option<bool>

    ) {

        let clear: bool = clear.unwrap_or(false);

        todo!();
        /*
            free();
            ptr_ = static_cast<T*>(clear ? CA_calloc(1, numBytes) : CA_malloc(numBytes));
        */
    }
    
    pub fn realloc_bytes(&mut self, num_bytes: usize)  {
        
        todo!();
        /*
            ptr_ = static_cast<T*>(CA_realloc(ptr_, numBytes));
        */
    }
    
    pub fn realloc_items(&mut self, num_items: usize)  {
        
        todo!();
        /*
            size_t maxItems = ~size_t(0) / sizeof(T);
            if (numItems > maxItems) throw std::bad_alloc();

            ptr_ = static_cast<T*>(CA_realloc(ptr_, numItems * sizeof(T)));
        */
    }
    
    pub fn assign_from<U>(&mut self, that: &mut CAAutoFree<U>) -> &mut CAAutoFree<T> {
    
        todo!();
        /*
            set(that.release());    // take ownership
            return *this;
        */
    }
    
    pub fn assign_from_ca_auto_free_t(&mut self, that: &mut CAAutoFree<T>) -> &mut CAAutoFree<T> {
        
        todo!();
        /*
            set(that.release());    // take ownership
            return *this;
        */
    }
    
    pub fn assign_from_raw(&mut self, ptr: *mut T) -> &mut CAAutoFree<T> {
        
        todo!();
        /*
            set(ptr);
            return *this;
        */
    }
    
    pub fn assign_from_raw_u<U>(&mut self, ptr: *mut U) -> &mut CAAutoFree<T> {
    
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
                ::free(ptr_);
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
    
    pub fn assign_from_ca_ptr_ref_t(&mut self, ref_: CAPtrRef<T>) -> &mut CAAutoFree<T> {
        
        todo!();
        /*
            set(ref.ptr_);
            return *this;
        */
    }
}
