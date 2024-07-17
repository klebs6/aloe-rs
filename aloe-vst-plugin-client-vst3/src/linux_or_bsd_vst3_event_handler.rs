crate::ix!();

#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[derive(Default)]
#[no_copy]
#[no_move]
#[leak_detector]
#[cfg(any(target_os="linux",target_os="bsd"))]
pub struct VstEventHandler {
    base:            IEventHandler,
    message_thread:  SharedResourcePointer<MessageThread>,
    ref_count:       Atomic<i32>, // default = { 1  }
    host_run_loops:  HostRunLoopInterfaces,
    fd_callback_map: HashMap<i32,fn(_0: i32) -> ()>,
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl Drop for VstEventHandler {

    fn drop(&mut self) {
        todo!();
        /*
            jassert (hostRunLoops.size() == 0);

                if (! messageThread->isRunning())
                    messageThread->start();
        */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl VstEventHandler {

    #[PLUGIN_API]
    pub fn query_interface(&mut self, 
        targetiid: TUID,
        obj:       *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            return testFor (*this, targetIID, UniqueBase<typename IEventHandler>{}).extract (obj);
        */
    }

    #[PLUGIN_API]
    pub fn on_fd_is_set(&mut self, fd: FileDescriptor)  {
        
        todo!();
        /*
            updateCurrentMessageThread();

                auto it = fdCallbackMap.find (fd);

                if (it != fdCallbackMap.end())
                    it->second (fd);
        */
    }
    
    pub fn register_handler_for_frame(&mut self, plug_frame: *mut dyn IPlugFrame)  {
        
        todo!();
        /*
            if (auto* runLoop = getRunLoopFromFrame (plugFrame))
                {
                    if (hostRunLoops.contains (runLoop))
                        runLoop->unregisterEventHandler (this);

                    hostRunLoops.add (runLoop);

                    fdCallbackMap.clear();

                    for (auto& cb : getFdReadCallbacks())
                    {
                        fdCallbackMap[cb.first] = cb.second;
                        runLoop->registerEventHandler (this, cb.first);
                    }

                    updateCurrentMessageThread();
                }
        */
    }
    
    pub fn unregister_handler_for_frame(&mut self, plug_frame: *mut dyn IPlugFrame)  {
        
        todo!();
        /*
            if (auto* runLoop = getRunLoopFromFrame (plugFrame))
                {
                    hostRunLoops.remove (runLoop);

                    if (! hostRunLoops.contains (runLoop))
                        runLoop->unregisterEventHandler (this);
                }
        */
    }
    
    pub fn get_run_loop_from_frame(plug_frame: *mut dyn IPlugFrame) -> *mut IRunLoop {
        
        todo!();
        /*
            typename IRunLoop* runLoop = nullptr;

                if (plugFrame != nullptr)
                    plugFrame->queryInterface (typename IRunLoop::iid, (void**) &runLoop);

                jassert (runLoop != nullptr);
                return runLoop;
        */
    }
    
    pub fn update_current_message_thread(&mut self)  {
        
        todo!();
        /*
            if (! MessageManager::getInstance()->isThisTheMessageThread())
                {
                    if (messageThread->isRunning())
                        messageThread->stop();

                    MessageManager::getInstance()->setCurrentThreadAsMessageThread();
                }
        */
    }
}
