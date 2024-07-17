crate::ix!();

#[cfg(any(target_os="linux",target_os="bsd"))]
#[derive(Default)]
pub struct AloeVst3ComponentLockedVSTComSmartPtr<T> {
    ptr: VSTComSmartPtr<T>,
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl Drop for AloeVst3ComponentLockedVSTComSmartPtr {

    fn drop(&mut self) {
        todo!();
        /*
            const MessageManagerLock mmLock;
                ptr = {};
        */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl Deref for AloeVst3ComponentLockedVSTComSmartPtr {

    type Target = T;
    
    #[inline] fn deref(self) -> &Self::Target {
        todo!();
        /*
            return ptr.operator->();
        */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl Into<T> for AloeVst3ComponentLockedVSTComSmartPtr {

    #[inline] fn into(self) -> T {
        todo!();
        /*
            return ptr.get();
        */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl AloeVst3ComponentLockedVSTComSmartPtr<T> {

    pub fn new(ptr_in: &VSTComSmartPtr<T>) -> Self {
    
        todo!();
        /*
        : ptr(ptrIn),

        
        */
    }
    
    pub fn get(&self) -> *mut T {
        
        todo!();
        /*
            return ptr.get();
        */
    }
    
    pub fn load_from<Args>(&mut self, args: Args) -> bool {
    
        todo!();
        /*
            return ptr.loadFrom (std::forward<Args> (args)...);
        */
    }
}
