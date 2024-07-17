crate::ix!();

#[cfg(not(kPrintfBufferSize))]
pub const kPrintfBufferSize: usize = 4096;

///---------------------------
#[cfg(SMTG_OS_MACOS)]
pub const SMTG_ENABLE_DEBUG_CFALLOCATOR: usize = 0;

#[cfg(SMTG_OS_MACOS)]
pub const SMTG_DEBUG_CFALLOCATOR:        usize = DEVELOPMENT && SMTG_ENABLE_DEBUG_CFALLOCATOR;

#[cfg(SMTG_OS_MACOS)]
#[cfg(SMTG_DEBUG_CFALLOCATOR)]
lazy_static!{
    /*
    static CFAllocatorRef kCFAllocator = NULL;
    */
}

#[cfg(SMTG_OS_MACOS)]
#[cfg(SMTG_DEBUG_CFALLOCATOR)]
pub struct CFStringDebugAllocator {
    base:              CFAllocatorContext,
    module_name:       String,
    cf_allocator:      CFAllocatorRef,
    num_allocations:   Volatile<i64>,
    num_deallocations: Volatile<i64>,
    allocation_size:   Volatile<i64>,
}

#[cfg(SMTG_OS_MACOS)]
#[cfg(SMTG_DEBUG_CFALLOCATOR)]
impl Default for CFStringDebugAllocator {
    
    fn default() -> Self {
        todo!();
        /*

            version = 0;
            info = this;
            retain = nullptr;
            release = nullptr;
            copyDescription = nullptr;
            allocate = allocateCallBack;
            reallocate = reallocateCallBack;
            deallocate = deallocateCallBack;
            preferredSize = preferredSizeCallBack;

            numAllocations = allocationSize = numDeallocations = 0;
            cfAllocator = CFAllocatorCreate (kCFAllocatorUseContext, this);

            Dl_info info;
            if (dladdr ((const void*)CFStringDebugAllocator::allocateCallBack, &info))
            {
                moduleName = info.dli_fname;
            }
            kCFAllocator = cfAllocator;
        */
    }
}

#[cfg(SMTG_OS_MACOS)]
#[cfg(SMTG_DEBUG_CFALLOCATOR)]
impl Drop for CFStringDebugAllocator {
    fn drop(&mut self) {
        todo!();
        /*
            kCFAllocator = kCFAllocatorDefault;
            CFRelease (cfAllocator);
            FDebugPrint ("CFStringDebugAllocator (%s):\n", moduleName.text8 ());
            FDebugPrint ("\tNumber of allocations  : %u\n", numAllocations);
            FDebugPrint ("\tNumber of deallocations: %u\n", numDeallocations);
            FDebugPrint ("\tAllocated Bytes        : %u\n", allocationSize);
        */
    }
}

#[cfg(SMTG_OS_MACOS)]
#[cfg(SMTG_DEBUG_CFALLOCATOR)]
impl CFStringDebugAllocator {

    pub fn do_allocate(&mut self, 
        alloc_size: CFIndex,
        hint:       CFOptionFlags)  {
        
        todo!();
        /*
            void* ptr = CFAllocatorAllocate (kCFAllocatorDefault, allocSize, hint);
            OSAtomicIncrement64 (&numAllocations);
            OSAtomicAdd64 (allocSize, &allocationSize);
            return ptr;
        */
    }
    
    pub fn do_reallocate(&mut self, 
        ptr:     *mut c_void,
        newsize: CFIndex,
        hint:    CFOptionFlags)  {
        
        todo!();
        /*
            void* newPtr = CFAllocatorReallocate (kCFAllocatorDefault, ptr, newsize, hint);
            return newPtr;
        */
    }
    
    pub fn do_deallocate(&mut self, ptr: *mut c_void)  {
        
        todo!();
        /*
            CFAllocatorDeallocate (kCFAllocatorDefault, ptr);
            OSAtomicIncrement64 (&numDeallocations);
        */
    }
    
    pub fn get_preferred_size(&mut self, 
        size: CFIndex,
        hint: CFOptionFlags) -> CFIndex {
        
        todo!();
        /*
            return CFAllocatorGetPreferredSizeForSize (kCFAllocatorDefault, size, hint);
        */
    }
    
    pub fn allocate_call_back(
        alloc_size: CFIndex,
        hint:       CFOptionFlags,
        info:       *mut c_void)  {
        
        todo!();
        /*
            return static_cast<CFStringDebugAllocator*> (info)->doAllocate (allocSize, hint);
        */
    }
    
    pub fn reallocate_call_back(
        ptr:     *mut c_void,
        newsize: CFIndex,
        hint:    CFOptionFlags,
        info:    *mut c_void)  {
        
        todo!();
        /*
            return static_cast<CFStringDebugAllocator*> (info)->doReallocate (ptr, newsize, hint);
        */
    }
    
    pub fn deallocate_call_back(
        ptr:  *mut c_void,
        info: *mut c_void)  {
        
        todo!();
        /*
            static_cast<CFStringDebugAllocator*> (info)->doDeallocate (ptr);
        */
    }
    
    pub fn preferred_size_call_back(
        size: CFIndex,
        hint: CFOptionFlags,
        info: *mut c_void) -> CFIndex {
        
        todo!();
        /*
            return static_cast<CFStringDebugAllocator*> (info)->getPreferredSize (size, hint);
        */
    }
}

#[cfg(SMTG_OS_MACOS)]
#[cfg(SMTG_DEBUG_CFALLOCATOR)]
lazy_static!{
    /*
    static CFStringDebugAllocator gDebugAllocator;
    */
}

#[cfg(SMTG_OS_MACOS)]
#[cfg(not(SMTG_DEBUG_CFALLOCATOR))]
pub const k_cf_allocator: CFAllocatorRef = kCFAllocatorDefault;

