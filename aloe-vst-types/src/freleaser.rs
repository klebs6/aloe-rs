crate::ix!();

/**
  | Release an interface using automatic
  | object (obsolete).
  | 
  | This class is obsolete and is only kept
  | for compatibility.
  | 
  | The replacement for FReleaser is OPtr.
  | 
  | Usage example with FReleaser:
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | void someFunction ()
  | {
  |     IPath* path = pathCreateMethod ();
  |     FReleaser releaser (path);
  |     .... do something with path...
  |     .... path not used anymore, releaser will destroy it when leaving function scope
  | }
  | 
  | Usage example with OPtr:
  | ----------
  | @code
  | 
  | {.cpp}
  | void someFunction ()
  | {
  |     OPtr<IPath> path = pathCreateMethod ();
  |     .... do something with path...
  |     .... path not used anymore, OPtr will destroy it when leaving function scope
  | }
  |
  */
pub struct FReleaser {
    u: *mut dyn FUnknown,
}

impl Drop for FReleaser {

    fn drop(&mut self) {
        todo!();
        /*
            if (u)
                u->release ();
        */
    }
}

impl FReleaser {

    pub fn new(u: *mut dyn FUnknown) -> Self {
    
        todo!();
        /*
        : u(u),

        
        */
    }
}
