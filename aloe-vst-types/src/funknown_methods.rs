crate::ix!();

#[macro_export] macro_rules! implement_funknown_methods {
    ($ClassName:ident, 
     $InterfaceName:ident, 
     $ClassIID:expr) => {
        /*
        
            IMPLEMENT_REFCOUNT (ClassName)                                   
            IMPLEMENT_QUERYINTERFACE (ClassName, InterfaceName, ClassIID)
        */
    }
}

/**
  | @name Convenient macros to implement
  | Steinberg::FUnknown methods. <b>Examples:</b>
  | 
  | -----------
  | @code
  | 
  | class Foo : public FObject, public IFoo2, public IFoo3
  | {
  |     ...
  |     FUNKNOWN_METHODS2(IFoo2,IFoo3,FObject)
  |     ...
  | };
  |
  */
#[macro_export]
macro_rules! funknown_methods {
    ($InterfaceName:ident, 
     $BaseClass:ident) => {
        /*
        
        DEF_INTERFACES_1(InterfaceName,BaseClass) 
        REFCOUNT_METHODS(BaseClass)
        */
    }
}

#[macro_export]
macro_rules! funknown_methods2 {
    ($InterfaceName1:ident, 
     $InterfaceName2:ident, 
     $BaseClass:ident) => {
        /*
        
        DEF_INTERFACES_2(InterfaceName1,InterfaceName2,BaseClass) 
        REFCOUNT_METHODS(BaseClass)
        */
    }
}

#[macro_export]
macro_rules! funknown_methods3 {
    ($InterfaceName1:ident, 
     $InterfaceName2:ident, 
     $InterfaceName3:ident, 
     $BaseClass:ident) => {
        /*
        
        DEF_INTERFACES_3(InterfaceName1,InterfaceName2,InterfaceName3,BaseClass) 
        REFCOUNT_METHODS(BaseClass)
        */
    }
}

#[macro_export]
macro_rules! funknown_methods4 {
    ($InterfaceName1:ident, 
     $InterfaceName2:ident, 
     $InterfaceName3:ident, 
     $InterfaceName4:ident, 
     $BaseClass:ident) => {
        /*
        
        DEF_INTERFACES_4(InterfaceName1,InterfaceName2,InterfaceName3,InterfaceName4,BaseClass) 
        REFCOUNT_METHODS(BaseClass)
        */
    }
}

/* -------- FUnknown implementation macros  -------- */

#[macro_export] macro_rules! declare_funknown_methods {
    () => {
        /*
            virtual typename Steinberg::tresult PLUGIN_API queryInterface (const typename Steinberg::TUID _iid, void** obj) SMTG_OVERRIDE; 
            virtual typename Steinberg::uint32 PLUGIN_API addRef () SMTG_OVERRIDE;                                   
            virtual typename Steinberg::uint32 PLUGIN_API release () SMTG_OVERRIDE;                                  
        protected :                                                                                           
            typename Steinberg::int32 __funknownRefCount;                                                            
        */
    }
}


#[macro_export] macro_rules! funknown_ctor {
    () => {
        /*
           __funknownRefCount = 1;
        */
    }
}

#[cfg(SMTG_FUNKNOWN_DTOR_ASSERT)]
#[macro_export] macro_rules! funknown_dtor {
    () => {
        /*
                { assert (__funknownRefCount == 0); }
        */
    }
}

#[cfg(not(SMTG_FUNKNOWN_DTOR_ASSERT))]
#[macro_export] macro_rules! funknown_dtor { () => { } }

