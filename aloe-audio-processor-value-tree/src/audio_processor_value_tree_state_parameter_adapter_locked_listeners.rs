crate::ix!();

pub struct AudioProcessorValueTreeStateParameterAdapterLockedListeners {
    mutex:     CriticalSection,
    listeners: ListenerList<Rc<RefCell<dyn AudioProcessorValueTreeStateListener>>>,
}

impl AudioProcessorValueTreeStateParameterAdapterLockedListeners {

    pub fn call<Fn>(&mut self, fn_: Fn)  {
    
        todo!();
        /*
            const CriticalSection::ScopedLockType lock (mutex);
                listeners.call (std::forward<Fn> (fn));
        */
    }
    
    pub fn add(&mut self, l: *mut dyn AudioProcessorValueTreeStateListener)  {
        
        todo!();
        /*
            const CriticalSection::ScopedLockType lock (mutex);
                listeners.add (l);
        */
    }
    
    pub fn remove(&mut self, l: *mut dyn AudioProcessorValueTreeStateListener)  {
        
        todo!();
        /*
            const CriticalSection::ScopedLockType lock (mutex);
                listeners.remove (l);
        */
    }
}
