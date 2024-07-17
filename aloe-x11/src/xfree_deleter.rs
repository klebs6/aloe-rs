crate::ix!();

#[cfg(all(ALOE_DEBUG,not(ALOE_DEBUG_XERRORS)))]
pub const ALOE_DEBUG_XERRORS: usize = 1;

#[cfg(all(ALOE_DEBUG,not(ALOE_DEBUG_XERRORS)))]
#[cfg(not(ALOE_DEBUG_XERRORS_SYNCHRONOUSLY))]
pub const ALOE_DEBUG_XERRORS_SYNCHRONOUSLY: usize = 1;

///--------------
#[cfg(ALOE_MODULE_AVAILABLE_aloe_gui_extra)]
pub const ALOE_X11_SUPPORTS_XEMBED: usize = 1;

#[cfg(not(ALOE_MODULE_AVAILABLE_aloe_gui_extra))]
pub const ALOE_X11_SUPPORTS_XEMBED: usize = 0;

pub struct XFreeDeleter { }

unsafe impl Allocator for XFreeDeleter {

    fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> { 
        todo!() 
    }

    unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) { 
        todo!() 
    }
}

impl XFreeDeleter {

    pub fn invoke(&self, ptr: *mut c_void)  {
        
        todo!();
        /*
            if (ptr != nullptr)
                X11Symbols::getInstance()->xFree (ptr);
        */
    }
}

pub fn make_xfree_ptr<Data>(raw: *mut Data) -> Box<Data,XFreeDeleter> {

    todo!();
        /*
            return std::unique_ptr<Data, XFreeDeleter> (raw);
        */
}

pub fn make_deleted_ptr<Data, Deleter: Allocator>(
    raw: *mut Data,
    d:   &Deleter

) -> Box<Data,Deleter> {

    todo!();
        /*
            return std::unique_ptr<Data, Deleter> (raw, d);
        */
}
