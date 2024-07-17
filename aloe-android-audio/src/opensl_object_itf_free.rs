crate::ix!();

#[cfg(target_os="android")]
pub struct SLObjectItfFree {

}

#[cfg(target_os="android")]
unsafe impl std::alloc::Allocator for SLObjectItfFree {

    fn allocate(&self, _: std::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, std::alloc::AllocError> { 
        todo!() 
    }

    unsafe fn deallocate(&self, _: core::ptr::NonNull<u8>, _: std::alloc::Layout) { 
        todo!() 
        /*
            destroyObject (obj);
        */
    }

}

