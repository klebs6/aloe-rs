crate::ix!();

pub trait ComponentRestarterListener
{
    fn restart_component_on_message_thread(&mut self, flags: i32);
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_Vst3Common.h]

/**
  | Ensures that a 'restart' call only ever
  | happens on the main thread.
  |
  */
pub struct ComponentRestarter<'a> {
    base:     AsyncUpdater<'a>,
    listener: &'a mut dyn ComponentRestarterListener,
    flags:    Atomic<i32>, // default = { 0  }
}

impl<'a> Drop for ComponentRestarter<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            cancelPendingUpdate();
        */
    }
}

impl<'a> ComponentRestarter<'a> {

    pub fn new(listener_in: &mut dyn ComponentRestarterListener) -> Self {
    
        todo!();
        /*
        : listener(listenerIn),

        
        */
    }
    
    pub fn restart(&mut self, new_flags: i32)  {
        
        todo!();
        /*
            if (newFlags == 0)
                return;

            flags.fetch_or (newFlags);

            if (MessageManager::getInstance()->isThisTheMessageThread())
                handleAsyncUpdate();
            else
                triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            listener.restartComponentOnMessageThread (flags.exchange (0));
        */
    }
}
