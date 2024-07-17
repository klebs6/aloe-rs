crate::ix!();

#[cfg(any(target_os="linux",target_os="bsd"))]
pub struct HostRunloopInterfacesRefCountedRunLoop {
    run_loop:  *mut IRunLoop, // default = nullptr
    ref_count: i32, // default = 0
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl PartialEq<IRunLoop> for HostRunloopInterfacesRefCountedRunLoop {

    #[inline] fn eq(&self, other: &IRunLoop) -> bool {
        todo!();
        /*
           return runLoop == other;
           */
    }
}

//--------------------------------------
#[derive(Default)]
#[no_copy]
#[no_move]
#[leak_detector]
#[cfg(any(target_os="linux",target_os="bsd"))]
pub struct HostRunLoopInterfaces {
    run_loops: Vec<HostRunloopInterfacesRefCountedRunLoop>,
}

#[cfg(any(target_os="linux",target_os="bsd"))]
impl HostRunLoopInterfaces {

    pub fn add(&mut self, run_loop: *mut IRunLoop)  {
        
        todo!();
        /*
            if (auto* refCountedRunLoop = find (runLoop))
                    {
                        ++(refCountedRunLoop->refCount);
                        return;
                    }

                    runLoops.push_back ({ runLoop, 1 });
        */
    }
    
    pub fn remove(&mut self, run_loop: *mut IRunLoop)  {
        
        todo!();
        /*
            if (auto* refCountedRunLoop = find (runLoop))
                        if (--(refCountedRunLoop->refCount) == 0)
                            runLoops.erase (std::find (runLoops.begin(), runLoops.end(), runLoop));
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return runLoops.size();
        */
    }
    
    pub fn contains(&mut self, run_loop: *mut IRunLoop) -> bool {
        
        todo!();
        /*
            return find (runLoop) != nullptr;
        */
    }
    
    pub fn find(&mut self, run_loop: *const IRunLoop) -> *mut HostRunloopInterfacesRefCountedRunLoop {
        
        todo!();
        /*
            auto iter = std::find (runLoops.begin(), runLoops.end(), runLoop);

                    if (iter != runLoops.end())
                        return &(*iter);

                    return nullptr;
        */
    }
}
