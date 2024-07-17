crate::ix!();

/**
  | FUnknownPtr - automatic interface
  | conversion and smart pointer in one.
  | 
  | This template class can be used for interface
  | conversion like this:
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | IPtr<IPath> path = owned (FHostCreate (IPath, hostClasses));
  | FUnknownPtr<IPath2> path2 (path); // does a query interface for IPath2
  | if (path2)
  |     ...
  |
  */
pub struct FUnknownPtr<I> {
    base: IPtr<I>,
}

impl<I> FUnknownPtr<I> {

    pub fn new_from_ptr_ref(p: &FUnknownPtr<I>) -> Self {
    
        todo!();
        /*
            : IPtr<I> (p)
        */
    }
    
    #[inline] pub fn assign_from_funknown_ptr_ref(&mut self, p: &FUnknownPtr<I>) -> &mut FUnknownPtr<I> {
        
        todo!();
        /*
            IPtr<I>::operator= (p);
            return *this;
        */
    }
    
    #[inline] pub fn get_interface(&mut self) -> *mut I {
        
        todo!();
        /*
            return this->ptr;
        */
    }

    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    pub fn new_from_ptr(p: FUnknownPtr) -> Self {
    
        todo!();
        /*


            : IPtr<I> (std::move (p))
        */
    }
    
    #[cfg(SMTG_CPP11_STDLIBSUPPORT)]
    #[inline] pub fn assign_from_funknown_ptr(&mut self, p: FUnknownPtr) -> &mut FUnknownPtr {
        
        todo!();
        /*
            IPtr<I>::operator= (std::move (p));
            return *this;
        */
    }

    /**
      | query interface
      |
      */
    pub fn new(unknown: *mut dyn FUnknown) -> Self {
    
        todo!();
        /*


            if (unknown && unknown->queryInterface (getTUID<I> (), (void**)&this->ptr) != kResultOk)
            this->ptr = 0;
        */
    }
    
    #[inline] pub fn assign_from_raw(&mut self, unknown: *mut dyn FUnknown) -> *mut I {
        
        todo!();
        /*
            I* newPtr = 0;
        if (unknown && unknown->queryInterface (getTUID<I> (), (void**)&newPtr) == kResultOk)
        {
            OPtr<I> rel (newPtr);
            return IPtr<I>::operator= (newPtr);
        }

        return IPtr<I>::operator= (0);
        */
    }
}
