crate::ix!();

///----------------------------
#[macro_export] macro_rules! query_interface {
    ($iid:ident, 
     $obj:ident, 
     $InterfaceIID:ident, 
     $InterfaceName:ident) => {
        /*
        
        if (typename Steinberg::FUnknownPrivate::iidEqual (iid, InterfaceIID)) 
        {                                                               
            addRef ();                                                  
            *obj = static_cast< InterfaceName* >(this);                 
            return typename Steinberg::kResultOk;                              
        }
        */
    }
}

#[macro_export] macro_rules! implement_queryinterface {
    ($ClassName:ident, 
     $InterfaceName:ident, 
     $ClassIID:ident) => {
        /*
        
        typename Steinberg::tresult PLUGIN_API ClassName::queryInterface (const typename Steinberg::TUID _iid, void** obj)
        {                                                                                                   
            QUERY_INTERFACE (_iid, obj, typename Steinberg::FUnknown::iid, InterfaceName)                          
            QUERY_INTERFACE (_iid, obj, ClassIID, InterfaceName)                                            
            *obj = nullptr;                                                                                 
            return typename Steinberg::kNoInterface;                                                               
        }
        */
    }
}
