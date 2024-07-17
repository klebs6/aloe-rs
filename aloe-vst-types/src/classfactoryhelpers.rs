/*!
  |------------------------------------------------------------------------------
  | Helper Macros. Not intended for direct use.
  | Use:
  |  META_CLASS(className),
  |  META_CLASS_IFACE(className,Interface),
  |  META_CLASS_SINGLE(className,Interface)
  | instead.
  |------------------------------------------------------------------------------
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/base/source/classfactoryhelpers.h]

macro_rules! meta_create_func {
    ($funcName:ident) => {
        /*
                static FUnknown* funcName ()
        */
    }
}

macro_rules! class_create_func {
    ($className:ident) => {
        /*
        
            namespace Meta {                                                               
            META_CREATE_FUNC (make##className) { return (NEW className)->unknownCast (); } 
            }
        */
    }
}

macro_rules! single_create_func {
    ($className:ident) => {
        /*
        
            namespace Meta {                                                                      
            META_CREATE_FUNC (make##className) { return className::instance ()->unknownCast (); } 
            }
        */
    }
}

macro_rules! meta_class {
    ($className:ident) => {
        /*
        
            namespace Meta {                                                                   
            static Steinberg::MetaClass meta##className ((#className), Meta::make##className); 
            }
        */
    }
}

macro_rules! meta_class_iface {
    ($className:ident, $Interface:ident) => {
        /*
        
            namespace Meta {                                                                             
            static Steinberg::MetaClass meta##Interface##className ((#className), Meta::make##className, 
                                                                    Interface##_iid);                    
            }
        */
    }
}

/* --------------------- TODO  --------------------- */
macro_rules! meta_class {
    ($className:ident) => {
        /*
        
            CLASS_CREATE_FUNC (className) 
            _META_CLASS (className)
        */
    }
}

macro_rules! meta_class_iface {
    ($className:ident, $Interface:ident) => {
        /*
        
            CLASS_CREATE_FUNC (className)              
            _META_CLASS_IFACE (className, Interface)
        */
    }
}

macro_rules! meta_class_single {
    ($className:ident, $Interface:ident) => {
        /*
        
            SINGLE_CREATE_FUNC (className)              
            _META_CLASS_IFACE (className, Interface)
        */
    }
}
