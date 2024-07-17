crate::ix!();

#[inline] pub fn safe_release_dependent<T>(
    this: *mut dyn IDependent,
    dest: &mut *mut T

) {

    todo!();
        /*
            if (dest)
            dest->removeDependent (_this);
        SafeRelease (dest);
        */
}

#[inline] pub fn safe_release_dependent_iptr<T>(
    this: *mut dyn IDependent,
    dest: &mut IPtr<T>

) {

    todo!();
        /*
            if (dest)
            dest->removeDependent (_this);
        SafeRelease (dest);
        */
}

/**
  | @name Convenience methods that call
  | release or delete respectively on a
  | pointer if it is non-zero, and then set
  | the pointer to zero.
  | 
  | -----------
  | @note
  | 
  | you should prefer using IPtr or OPtr
  | instead of these methods whenever possible.
  | <b>Examples:</b>
  | 
  | -----------
  | @code
  | 
  | ~Foo ()
  | {
  |     // instead of ...
  |     if (somePointer)
  |     {
  |         somePointer->release ();
  |         somePointer = 0;
  |     }
  |     // ... just being lazy I write
  |     SafeRelease (somePointer)
  | }
  |
  */
#[inline] pub fn safe_release<I>(ptr: &mut *mut I)  {

    todo!();
        /*
            if (ptr) 
        {
            ptr->release (); 
            ptr = 0;
        }
        */
}

#[inline] pub fn safe_release_iptr<I>(ptr: &mut IPtr<I>)  {

    todo!();
        /*
            ptr = 0;
        */
}

#[inline] pub fn safe_delete<T>(ptr: &mut *mut T)  {

    todo!();
        /*
            if (ptr) 
        {
            delete ptr;
            ptr = 0;
        }
        */
}
