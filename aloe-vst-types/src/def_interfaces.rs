crate::ix!();

/*
  | @name Macros to implement FUnknown::queryInterface
  | ().
  | 
  | <b>Examples:</b>
  | 
  | -----------
  | @code
  | 
  | class Foo : public FObject, public IFoo2, public IFoo3
  | {
  |     ...
  |     DEFINE_INTERFACES
  |         DEF_INTERFACE (IFoo2)
  |         DEF_INTERFACE (IFoo3)
  |     END_DEFINE_INTERFACES (FObject)
  |     REFCOUNT_METHODS(FObject)
  |     // Implement IFoo2 interface ...
  |     // Implement IFoo3 interface ...
  |     ...
  | };
  |
  */

/**
  | Start defining interfaces.
  |
  */
#[macro_export]
macro_rules! define_interfaces {
    () => {
        /*
        
        Steinberg::tresult PLUGIN_API queryInterface (const Steinberg::TUID iid, void** obj) SMTG_OVERRIDE 
        {
        */
    }
}

/**
  | Add a interfaces.
  |
  */
#[macro_export]
macro_rules! def_interface {
    ($InterfaceName:ident) => {
        /*
        
            QUERY_INTERFACE (iid, obj, InterfaceName::iid, InterfaceName)
        */
    }
}

/**
  | End defining interfaces.
  |
  */
#[macro_export]
macro_rules! end_define_interfaces {
    ($BaseClass:ident) => {
        /*
        
            return BaseClass::queryInterface (iid, obj); 
        }
        */
    }
}

/**
  | @name Convenient macros to implement
  | Steinberg::FUnknown::queryInterface
  | (). <b>Examples:</b>
  | 
  | -----------
  | @code
  | 
  | class Foo : public FObject, public IFoo2, public IFoo3
  | {
  |     ...
  |     DEF_INTERFACES_2(IFoo2,IFoo3,FObject)
  |     REFCOUNT_METHODS(FObject)
  |     ...
  | };
  |
  */
#[macro_export]
macro_rules! def_interfaces_1 {
    ($InterfaceName:ident, 
     $BaseClass:ident) => {
        /*
        
        DEFINE_INTERFACES 
        DEF_INTERFACE (InterfaceName) 
        END_DEFINE_INTERFACES (BaseClass)
        */
    }
}

#[macro_export]
macro_rules! def_interfaces_2 {
    ($InterfaceName1:ident, 
     $InterfaceName2:ident, 
     $BaseClass:ident) => {
        /*
        
        DEFINE_INTERFACES 
        DEF_INTERFACE (InterfaceName1) 
        DEF_INTERFACE (InterfaceName2) 
        END_DEFINE_INTERFACES (BaseClass)
        */
    }
}

#[macro_export]
macro_rules! def_interfaces_3 {
    ($InterfaceName1:ident, 
     $InterfaceName2:ident, 
     $InterfaceName3:ident, 
     $BaseClass:ident) => {
        /*
        
        DEFINE_INTERFACES 
        DEF_INTERFACE (InterfaceName1) 
        DEF_INTERFACE (InterfaceName2) 
        DEF_INTERFACE (InterfaceName3) 
        END_DEFINE_INTERFACES (BaseClass)
        */
    }
}

#[macro_export]
macro_rules! def_interfaces_4 {
    ($InterfaceName1:ident, 
     $InterfaceName2:ident, 
     $InterfaceName3:ident, 
     $InterfaceName4:ident, 
     $BaseClass:ident) => {
        /*
        
            DEFINE_INTERFACES 
            DEF_INTERFACE (InterfaceName1) 
            DEF_INTERFACE (InterfaceName2) 
            DEF_INTERFACE (InterfaceName3) 
            DEF_INTERFACE (InterfaceName4) 
            END_DEFINE_INTERFACES (BaseClass)
        */
    }
}
