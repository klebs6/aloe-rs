crate::ix!();

#[macro_export]
#[cfg(COM_COMPATIBLE)]
macro_rules! com_query_interface {
    ($iid:ident, 
     $obj:ident, 
     $InterfaceName:ident) => {
        /*
        
        if (riid == __uuidof(InterfaceName))                     
        {                                                        
            addRef ();                                           
            *obj = (InterfaceName*)this;                         
            return kResultOk;                                    
        }
        */
    }
}

#[macro_export]
#[cfg(COM_COMPATIBLE)]
macro_rules! com_object_query_interface {
    ($InterfaceName:ident, 
     $BaseClass:ident) => {
        /*
        
        STDMETHOD (QueryInterface) (REFIID riid, void** object)            
        {                                                                  
            COM_QUERY_INTERFACE (riid, object, InterfaceName)              
            return BaseClass::queryInterface ((FIDString)&riid, object);   
        }
        */
    }
}

#[macro_export]
#[cfg(COM_COMPATIBLE)]
macro_rules! com_unknown_methods {
    ($InterfaceName:ident, 
     $BaseClass:ident) => {
        /*
        
        COM_OBJECT_QUERY_INTERFACE(InterfaceName,BaseClass) 
        IUNKNOWN_REFCOUNT_METHODS(BaseClass)
        */
    }
}
