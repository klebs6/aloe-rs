crate::ix!();

#[cfg(any(target_os="linux",target_os="bsd"))]
pub struct AloeVst3EditorMessageManagerLockedDeleter { }

#[cfg(any(target_os="linux",target_os="bsd"))]
impl AloeVst3EditorMessageManagerLockedDeleter {
    
    pub fn invoke<ObjectType>(&self, object: *mut ObjectType)  {
    
        todo!();
        /*
            const MessageManagerLock mmLock;
                    delete object;
        */
    }
}
