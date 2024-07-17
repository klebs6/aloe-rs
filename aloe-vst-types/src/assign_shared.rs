crate::ix!();

#[inline] pub fn assign_shared<T>(
        dest:    &mut *mut T,
        new_ptr: *mut T)  {

    todo!();
        /*
            if (dest == newPtr)
            return;
        
        if (dest) 
            dest->release (); 
        dest = newPtr; 
        if (dest) 
            dest->addRef ();
        */
}

