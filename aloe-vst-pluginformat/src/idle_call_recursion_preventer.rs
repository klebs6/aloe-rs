crate::ix!();

#[no_copy]
pub struct IdleCallRecursionPreventer {
    is_message_thread: bool,
}

impl Default for IdleCallRecursionPreventer {
    
    fn default() -> Self {
        todo!();
        /*


            : isMessageThread (MessageManager::getInstance()->isThisTheMessageThread())
            if (isMessageThread)
                ++insideVSTCallback;
        */
    }
}

impl Drop for IdleCallRecursionPreventer {
    fn drop(&mut self) {
        todo!();
        /*
            if (isMessageThread)
                --insideVSTCallback;
        */
    }
}


