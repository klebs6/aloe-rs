crate::ix!();

pub struct FileBasedDocumentSafeParentPointer<'a> {
    ptr:          WeakReference<FileBasedDocumentPimpl<'a>>,
    should_check: bool, // default = false
}

impl<'a> Deref for FileBasedDocumentSafeParentPointer<'a> {

    type Target = FileBasedDocumentPimpl<'a>;
    
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return ptr.get();
        */
    }
}

impl<'a> PartialEq<FileBasedDocumentPimpl<'a>> for FileBasedDocumentSafeParentPointer<'a> {
    
    #[inline] fn eq(&self, other: &FileBasedDocumentPimpl<'a>) -> bool {
        todo!();
        /*
            return ptr.get() == object;
        */
    }
}

impl<'a> FileBasedDocumentSafeParentPointer<'a> {
    
    pub fn new(
        parent:   *mut FileBasedDocumentPimpl<'a>,
        is_async: bool) -> Self {
    
        todo!();
        /*
        : ptr(parent),
        : should_check(isAsync),
        */
    }
    
    pub fn should_exit_async_callback(&self) -> bool {
        
        todo!();
        /*
            return shouldCheck && ptr == nullptr;
        */
    }
}
