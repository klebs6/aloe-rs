crate::ix!();

pub struct FileBasedDocumentSafeParentPointer<'a> {
    ptr:          WeakReference<FileBasedDocumentImpl<'a>>,
    should_check: bool, // default = false
}

impl<'a> Deref for FileBasedDocumentSafeParentPointer<'a> {

    type Target = FileBasedDocumentImpl<'a>;
    
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return ptr.get();
        */
    }
}

impl<'a> PartialEq<FileBasedDocumentImpl<'a>> for FileBasedDocumentSafeParentPointer<'a> {
    
    #[inline] fn eq(&self, other: &FileBasedDocumentImpl<'a>) -> bool {
        todo!();
        /*
            return ptr.get() == object;
        */
    }
}

impl<'a> FileBasedDocumentSafeParentPointer<'a> {
    
    pub fn new(
        parent:   *mut FileBasedDocumentImpl<'a>,
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
