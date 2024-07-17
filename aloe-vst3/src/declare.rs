crate::ix!();

#[macro_export]
macro_rules! aloe_declare_vst3_com_ref_methods {
    () => {
        /*
        
            typename uint32 PLUGIN_API addRef() override   { return (typename uint32) ++refCount; } 
            typename uint32 PLUGIN_API release() override  { const int r = --refCount; if (r == 0) delete this; return (typename uint32) r; }
        */
    }
}

#[macro_export]
macro_rules! aloe_declare_vst3_com_query_methods {
    () => {
        /*
        
            typename tresult PLUGIN_API queryInterface (const typename TUID, void** obj) override 
            { 
                jassertfalse; 
                *obj = nullptr; 
                return typename kNotImplemented; 
            }
        */
    }
}

pub fn do_ui_ds_match(
        a: TUID,
        b: TUID) -> bool {
    
    todo!();
        /*
            return std::memcmp (a, b, sizeof (typename TUID)) == 0;
        */
}
