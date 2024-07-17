crate::ix!();

/**
  | Holds a tresult and a pointer to an object.
  | 
  | Calling InterfaceResultWithDeferredAddRef::extract()
  | will also call addRef on the pointed-to
  | object. It is expected that users will
  | use
  | 
  | InterfaceResultWithDeferredAddRef
  | to hold intermediate results of a queryInterface
  | call. When a suitable interface is found,
  | the function can be exited with `return
  | suitableInterface.extract (obj)`,
  | which will set the obj pointer, add a
  | reference to the interface, and return
  | the appropriate result code.
  |
  */
#[derive(Default)]
pub struct InterfaceResultWithDeferredAddRef {
    result:     QueryInterfaceResult,
    add_ref_fn: Option<fn(_0: *mut c_void) -> c_void>,
}

impl InterfaceResultWithDeferredAddRef {

    pub fn new<Ptr>(
        result_in: tresult,
        ptr_in:    *mut Ptr) -> Self {
    
        todo!();
        /*
        : result(resultIn, ptrIn),
        : add_ref_fn(doAddRef<Ptr>),

        
        */
    }
    
    pub fn is_ok(&self) -> bool {
        
        todo!();
        /*
            return result.isOk();
        */
    }
    
    pub fn extract(&self, obj: *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            const auto toReturn = result.extract (obj);

            if (result.isOk() && addRefFn != nullptr && *obj != nullptr)
                addRefFn (*obj);

            return toReturn;
        */
    }
    
    
    pub fn do_add_ref<Ptr>(obj: *mut c_void)  {
    
        todo!();
        /*
            static_cast<Ptr*> (obj)->addRef();
        */
    }
}
