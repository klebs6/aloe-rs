crate::ix!();

#[cfg(any(target_os="linux",target_os="bsd"))]
#[derive(Default)]
#[no_copy]
#[leak_detector]
#[no_move]
pub struct RunLoop {
    base:              IRunLoop,
    event_handler_map: HashMap<FileDescriptor,Vec<*mut IEventHandler>>,
    timer_callers:     LinkedList<RunLoopTimerCaller>,
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl Drop for RunLoop {

    fn drop(&mut self) {
        todo!();
        /*
            for (const auto& h : eventHandlerMap)
                LinuxEventLoop::unregisterFdCallback (h.first);
        */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl RunLoop {

    #[PLUGIN_API]
    pub fn register_event_handler(
        &mut self, 
        handler: *mut IEventHandler,
        fd:      FileDescriptor

    ) -> tresult {
        
        todo!();
        /*
            if (handler == nullptr)
                return kInvalidArgument;

            auto& handlers = eventHandlerMap[fd];

            if (handlers.empty())
            {
                LinuxEventLoop::registerFdCallback (fd, [this] (int descriptor)
                {
                    for (auto* h : eventHandlerMap[descriptor])
                        h->onFDIsSet (descriptor);

                    return true;
                });
            }

            handlers.push_back (handler);

            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn unregister_event_handler(&mut self, handler: *mut IEventHandler) -> tresult {
        
        todo!();
        /*
            if (handler == nullptr)
                return kInvalidArgument;

            for (auto iter = eventHandlerMap.begin(), end = eventHandlerMap.end(); iter != end;)
            {
                auto& handlers = iter->second;

                auto handlersIter = std::find (std::begin (handlers), std::end (handlers), handler);

                if (handlersIter != std::end (handlers))
                {
                    handlers.erase (handlersIter);

                    if (handlers.empty())
                    {
                        LinuxEventLoop::unregisterFdCallback (iter->first);
                        iter = eventHandlerMap.erase (iter);
                        continue;
                    }
                }

                ++iter;
            }

            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn register_timer(
        &mut self, 
        handler:      *mut ITimerHandler,
        milliseconds: TimerInterval

    ) -> tresult {
        
        todo!();
        /*
            if (handler == nullptr || milliseconds <= 0)
                return kInvalidArgument;

            timerCallers.emplace_back (handler, (int) milliseconds);
            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn unregister_timer(&mut self, handler: *mut ITimerHandler) -> tresult {
        
        todo!();
        /*
            auto iter = std::find (timerCallers.begin(), timerCallers.end(), handler);

            if (iter == timerCallers.end())
                return kInvalidArgument;

            timerCallers.erase (iter);
            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn add_ref(&mut self) -> u32 {
        
        todo!();
        /*
            return 1000;
        */
    }
    
    #[PLUGIN_API]
    pub fn release(&mut self) -> u32 {
        
        todo!();
        /*
            return 1000;
        */
    }
    
    #[PLUGIN_API]
    pub fn query_interface(&mut self, 
        tuid: &TUID,
        _1:   *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            return kNoInterface;
        */
    }
}
