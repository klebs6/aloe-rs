crate::ix!();

///---------------------
#[cfg(target_os="android")]
#[derive(Default)]
pub struct SlRef<T> {
    base: SlObjectRef,
    ty: Option<*const *const T>,
}

#[cfg(target_os="android")]
impl<T> Deref for SlRef<T> {

    type Target = *mut *mut T;
    
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return type;
        */
    }
}

#[cfg(target_os="android")]
impl<T> From<&SlRef<T>> for SlRef<T> {

    fn from(r: &SlRef<T>) -> Self {
    
        todo!();
        /*
        : sl_object_ref(r),
        : ty(r.type),

        
        */
    }
}

#[cfg(target_os="android")]
impl<T> From<SlRef<T>> for SlRef<T> {
    
    fn from(r: SlRef<T>) -> Self {
    
        todo!();
        /*
        : sl_object_ref(std::move (r)),
        : ty(r.type),

            r.type = nullptr;
        */
    }
}

#[cfg(target_os="android")]
impl<T> SlRef<T> {
    
    pub fn assign_from(&mut self, r: &SlRef<T>) -> &mut SlRef<T> {
        
        todo!();
        /*
            SlObjectRef::operator= (r); type = r.type; return *this;
        */
    }
    
    pub fn assign_from(&mut self, r: SlRef<T>) -> &mut SlRef<T> {
        
        todo!();
        /*
            SlObjectRef::operator= (std::move (r)); type = r.type; r.type = nullptr; return *this;
        */
    }
    
    pub fn assign_from_null_ptr(&mut self) -> &mut SlRef<T> {
        
        todo!();
        /*
            SlObjectRef::operator= (nullptr); type = nullptr; return *this;
        */
    }
    
    pub fn cast_from_mut(base: &mut SlObjectRef) -> SlRef<T> {
        
        todo!();
        /*
            return SlRef (base);
        */
    }
    
    pub fn cast(base: SlObjectRef) -> SlRef<T> {
        
        todo!();
        /*
            return SlRef (std::move (base));
        */
    }
}

#[cfg(target_os="android")]
impl<T> From<&mut SlObjectRef> for SlRef<T> {

    fn from(base: &mut SlObjectRef) -> Self {
    
        todo!();
        /*
        : sl_object_ref(base),

            if (auto obj = SlObjectRef::operator->())
            {
                auto err = (*obj)->GetInterface (obj, &IntfIID<T>::iid, &type);

                if (type != nullptr && err == SL_RESULT_SUCCESS)
                    return;
            }

            *this = nullptr;
        */
    }
}

#[cfg(target_os="android")]
impl<T> From<&SlObjectRef> for SlRef<T> {
    
    fn from(base: &SlObjectRef) -> Self {
    
        todo!();
        /*
        : sl_object_ref(std::move (base)),

            if (auto obj = SlObjectRef::operator->())
            {
                auto err = (*obj)->GetInterface (obj, &IntfIID<T>::iid, &type);
                base = nullptr;

                if (type != nullptr && err == SL_RESULT_SUCCESS)
                    return;
            }

            *this = nullptr;
        */
    }
}
