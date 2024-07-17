/**
   Description : Compile time type size check
   macro
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/typesizecheck.h]

#[cfg(SMTG_CPP11)]
macro_rules! smtg_type_static_check {
    ($Operator:ident, 
     $Type:ident, 
     $Platform64Size:ident, 
     $MacOS32Size:ident, 
     $Win32Size:ident, 
     $Linux32Size:ident) => {
        /*
        
            namespace {                                                                                    
            template <typename Type, size_t w, size_t x, size_t y, size_t z>                               
            struct Operator##Check##Type                                                                   
            {                                                                                              
                constexpr Operator##Check##Type ()                                                         
                {                                                                                          
                    static_assert (Operator (Type) ==                                                      
                                       (SMTG_PLATFORM_64 ? w : SMTG_OS_MACOS ? x : SMTG_OS_LINUX ? z : y), 
                                   "Struct " #Operator " error: " #Type);                                     
                }                                                                                          
            };                                                                                             
            static constexpr Operator##Check##Type<Type, Platform64Size, MacOS32Size, Win32Size,           
                                                   Linux32Size>                                            
                instance##Operator##Type;                                                                  
            }
        */
    }
}

/**
  | Check the size of a structure depending
  | on compilation platform
  | 
  | Used to check that structure sizes don't
  | change between SDK releases.
  |
  */
#[cfg(SMTG_CPP11)]
#[macro_export]
macro_rules! smtg_type_size_check {
    ($Type:ident, 
     $Platform64Size:expr, 
     $MacOS32Size:expr, 
     $Win32Size:expr, 
     $Linux32Size:expr) => {
        /*
        
            SMTG_TYPE_STATIC_CHECK (sizeof, Type, Platform64Size, MacOS32Size, Win32Size, Linux32Size)
        */
    }
}

/**
  | Check the alignment of a structure depending
  | on compilation platform
  | 
  | Used to check that structure alignments
  | don't change between SDK releases.
  |
  */
#[cfg(SMTG_CPP11)]
macro_rules! smtg_type_align_check {
    ($Type:ident, 
     $Platform64Size:ident, 
     $MacOS32Size:ident, 
     $Win32Size:ident, 
     $Linux32Size:ident) => {
        /*
        
            SMTG_TYPE_STATIC_CHECK (alignof, Type, Platform64Size, MacOS32Size, Win32Size, Linux32Size)
        */
    }
}

/**
   need static_assert
  */
#[cfg(not(SMTG_CPP11))]
#[macro_export]
macro_rules! smtg_type_size_check { 
    ($Type:ident, 
     $Platform64Size:expr, 
     $MacOS32Size:expr, 
     $Win32Size:expr, 
     $Linux32Size:expr) => { } }
