crate::ix!();

#[macro_export]
macro_rules! obj_methods {
    ($className:ty, $baseClass:ty) => {
        /*
        
            static inline Steinberg::FClassID getFClassID () {return (#className);}     
            virtual Steinberg::FClassID isA () const SMTG_OVERRIDE {return className::getFClassID ();}  
            virtual bool isA (Steinberg::FClassID s) const SMTG_OVERRIDE {return isTypeOf (s, false);}  
            virtual bool isTypeOf (Steinberg::FClassID s, bool askBaseClass = true) const SMTG_OVERRIDE 
            {  return (classIDsEqual (s, #className) ? true : (askBaseClass ? baseClass::isTypeOf (s, true) : false)); } 
        */
    }
}

/**
  | Delegate refcount functions to BaseClass.
  | 
  | BaseClase must implement ref counting.
  |
  */
#[macro_export]
macro_rules! refcount_methods {
    ($BaseClass:ident) => {
        /*
        
        virtual Steinberg::uint32 PLUGIN_API addRef ()SMTG_OVERRIDE{ return BaseClass::addRef (); } 
        virtual Steinberg::uint32 PLUGIN_API release ()SMTG_OVERRIDE{ return BaseClass::release (); }
        */
    }
}

/**
  | @name Macros to implement IUnknown
  | interfaces with FObject. <b>Examples:</b>
  | 
  | -----------
  | @code
  | 
  | class MyEnumFormat : public FObject, IEnumFORMATETC
  | {
  |     ...
  |     COM_UNKNOWN_METHODS (IEnumFORMATETC, IUnknown)
  |     ...
  | };
  |
  */
#[macro_export]
#[cfg(COM_COMPATIBLE)]
macro_rules! iunknown_refcount_methods {
    ($BaseClass:ident) => {
        /*
        
        STDMETHOD_ (ULONG, AddRef) (void) {return BaseClass::addRef ();} 
        STDMETHOD_ (ULONG, Release) (void) {return BaseClass::release ();}
        */
    }
}
