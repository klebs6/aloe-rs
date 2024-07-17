crate::ix!();

pub struct EmbeddedUI<'a> {
    listener:  &'a mut dyn EmbeddedViewListener,
    ref_count: Atomic<i32>, // default = { 1  }
}

impl<'a> IReaperUIEmbedInterface for EmbeddedUI<'a> {

    fn embed_message(&mut self, 
        msg:   i32,
        parm2: TPtrInt,
        parm3: TPtrInt) -> TPtrInt {
        
        todo!();
        /*
            return listener.handledEmbeddedUIMessage (msg, parm2, parm3);
        */
    }
}

impl<'a> FUnknown for EmbeddedUI<'a> {

    fn add_ref(&mut self) -> u32 {
        
        todo!();
        /*
            return (typename Steinberg::uint32) ++refCount;
        */
    }
    
    fn release(&mut self) -> u32 {
        
        todo!();
        /*
            return (typename Steinberg::uint32) --refCount;
        */
    }

    fn query_interface(
        &mut self, 
        tuid: TUID,
        obj:  *mut *mut c_void

    ) -> i32 {
        
        todo!();
        /*
            if (std::memcmp (tuid, iid, sizeof (Steinberg::TUID)) == 0)
            {
                *obj = this;
                return Steinberg::kResultOk;
            }

            *obj = nullptr;
            return Steinberg::kNoInterface;
        */
    }
}

impl<'a> EmbeddedUI<'a> {

    pub fn new(demo: &mut dyn EmbeddedViewListener) -> Self {
    
        todo!();
        /*
        : listener(demo),

        
        */
    }
}
