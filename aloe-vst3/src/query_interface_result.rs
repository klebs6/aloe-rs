crate::ix!();

/**
  | Holds a tresult and a pointer to an object.
  | 
  | Useful for holding intermediate results
  | of calls to queryInterface.
  |
  */
#[derive(Default)]
pub struct QueryInterfaceResult {
    result: tresult, //= typename kResultFalse;
    ptr:    Option<*mut c_void>, // default = nullptr
}

impl QueryInterfaceResult {
    
    pub fn new(
        result_in: tresult,
        ptr_in:    *mut c_void) -> Self {
    
        todo!();
        /*
        : result(resultIn),
        : ptr(ptrIn),

        
        */
    }
    
    pub fn is_ok(&self) -> bool {
        
        todo!();
        /*
            return result == typename kResultOk;
        */
    }
    
    pub fn extract(&self, obj: *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            *obj = result == typename kResultOk ? ptr : nullptr;
            return result;
        */
    }
}
