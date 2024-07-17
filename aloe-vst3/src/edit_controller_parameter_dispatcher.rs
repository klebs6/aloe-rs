crate::ix!();

/**
  | Allows parameter updates to be queued
  | up without blocking, and automatically
  | dispatches these updates on the main
  | thread.
  |
  */
pub struct EditControllerParameterDispatcher {
    base:       Timer,
    cache:      CachedParamValues,
    controller: *mut dyn IEditController, // default = nullptr
}

impl Drop for EditControllerParameterDispatcher {

    fn drop(&mut self) {
        todo!();
        /*
            stopTimer();
        */
    }
}

impl EditControllerParameterDispatcher {

    pub fn push(&mut self, 
        index: i32,
        value: f32)  {
        
        todo!();
        /*
            if (controller == nullptr)
                return;

            if (MessageManager::getInstance()->isThisTheMessageThread())
                controller->setParamNormalized (cache.getParamID (index), value);
            else
                cache.set (index, value);
        */
    }
    
    pub fn start(&mut self, controller_in: &mut dyn IEditController)  {
        
        todo!();
        /*
            controller = &controllerIn;
            cache = CachedParamValues { getAllParamIDs (controllerIn) };
            startTimerHz (60);
        */
    }
    
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            cache.ifSet ([this] (i32 index, float value)
            {
                controller->setParamNormalized (cache.getParamID (index), value);
            });
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            flush();
        */
    }
}
