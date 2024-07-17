crate::ix!();

#[macro_export] macro_rules! delegate_refcount {
    ($ClassName:ident) => {
        /*
            virtual typename Steinberg::uint32 PLUGIN_API addRef () SMTG_OVERRIDE { return ClassName::addRef ();  } 
            virtual typename Steinberg::uint32 PLUGIN_API release () SMTG_OVERRIDE { return ClassName::release (); }
        */
    }
}

#[macro_export] macro_rules! implement_refcount {
    ($ClassName:ident) => {
        /*
        
        typename Steinberg::uint32 PLUGIN_API ClassName::addRef ()                            
        {                                                                              
            return typename Steinberg::FUnknownPrivate::atomicAdd (__funknownRefCount, 1);    
        }                                                                              
        typename Steinberg::uint32 PLUGIN_API ClassName::release ()                           
        {                                                                              
            if (typename Steinberg::FUnknownPrivate::atomicAdd (__funknownRefCount, -1) == 0) 
            {                                                                          
                delete this;                                                           
                return 0;                                                              
            }                                                                          
            return __funknownRefCount;                                                 
        }
        */
    }
}
