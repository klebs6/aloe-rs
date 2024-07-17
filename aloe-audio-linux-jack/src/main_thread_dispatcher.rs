crate::ix!();

pub struct JackAudioIODeviceMainThreadDispatcher<'a> {
    base: AsyncUpdater<'a>,
    ref_: &'a mut JackAudioIODevice<'a>,
}

impl<'a> Drop for JackAudioIODeviceMainThreadDispatcher<'a> {

    fn drop(&mut self) {
        todo!();
        /*      cancelPendingUpdate();  */
    }
}

impl<'a> JackAudioIODeviceMainThreadDispatcher<'a> {

    pub fn new(device: &mut JackAudioIODevice) -> Self {
    
        todo!();
        /*
        : ref_(device),

        
        */
    }
    
    pub fn update_active_ports(&mut self)  {
        
        todo!();
        /*
            if (MessageManager::getInstance()->isThisTheMessageThread())
                    handleAsyncUpdate();
                else
                    triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            ref.updateActivePorts();
        */
    }
}
