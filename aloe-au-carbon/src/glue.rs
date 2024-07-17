crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUCarbonViewDispatch.cpp]
pub struct CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage {}
pub type AudioUnitCarbonViewEventID       = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type AudioUnitCarbonViewEventListener = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type AudioUnitCarbonView              = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type ControlFontStyleRec              = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type ControlKeyFilterResult           = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type ControlPartCode                  = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type ControlRef                       = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type EventHandlerCallRef              = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type EventLoopTimerRef                = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type EventLoopTimerUPP                = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type EventModifiers                   = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type EventRef                         = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type EventTargetRef                   = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type EventTypeSpec                    = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type HIPoint                          = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
pub type WindowRef                        = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;
//pub type au_carbon_view_base              = CarbonIsDeprecatedAndWeShouldPossiblyRemoveItsUsage;

/* -------------- component dispatch  -------------- */

lazy_static!{
    /*
    #if PRAGMA_STRUCT_ALIGN
        #pragma options align=mac68k
    #elif PRAGMA_STRUCT_PACKPUSH
        #pragma pack(push, 2)
    #elif PRAGMA_STRUCT_PACK
        #pragma pack(2)
    #endif
    */
}

pub struct AudioUnitCarbonViewCreateGluePB {
    component_flags:      u8,
    component_param_size: u8,
    component_what:       i16,
    out_control:          *mut ControlRef,
    in_size:              *mut Float32Point,
    in_location:          *mut Float32Point,
    in_parent_control:    ControlRef,
    in_window:            WindowRef,
    in_audio_unit:        AudioUnit,
    in_view:              AudioUnitCarbonView,
}

#[cfg(not(__LP64__))]
pub struct AudioUnitCarbonViewSetEventListenerGluePB {
    component_flags:      u8,
    component_param_size: u8,
    component_what:       i16,
    in_user_data:         *mut c_void,
    in_callback:          AudioUnitCarbonViewEventListener,
    in_view:              AudioUnitCarbonView,
}

lazy_static!{
    /*
    #if PRAGMA_STRUCT_ALIGN
        #pragma options align=reset
    #elif PRAGMA_STRUCT_PACKPUSH
        #pragma pack(pop)
    #elif PRAGMA_STRUCT_PACK
        #pragma pack()
    #endif
    */
}

macro_rules! checknull {
    ($x:ident) => {
        /*
                if ((x) == NULL) return paramErr;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUCarbonViewControl.cpp]

///-------------------------
#[cfg(not(__LP64__))]
lazy_static!{
    /*
    static CFStringRef kStringFactoryPreset = kAUViewLocalizedStringKey_FactoryPreset;
    static bool sAUVPresetLocalized = false;
    */
}
