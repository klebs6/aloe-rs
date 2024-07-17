crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUViewLocalizedStringKeys.h]

/* ----------------- ACCESS POINT:  ----------------- */
pub fn kLocalizedStringBundle_AUView() -> CFString {  CFString::new("com.apple.audio.units.Components") }
pub fn kLocalizedStringTable_AUView()  -> CFString { CFString::new("CustomUI") }


/* ------------- UNLOCALIZED STRINGS:  ------------- */
pub fn kAUViewUnlocalizedString_TitleSeparator() -> CFString { CFString::new(": ") }


/* ----------------- Generic View:  ----------------- */
pub fn kAUViewLocalizedStringKey_AudioUnit()     -> CFString { CFString::new("Audio Unit") }
pub fn kAUViewLocalizedStringKey_Manufacturer()  -> CFString { CFString::new("Manufacturer") }
pub fn kAUViewLocalizedStringKey_FactoryPreset() -> CFString { CFString::new("Factory Preset") }
pub fn kAUViewLocalizedStringKey_Properties()    -> CFString { CFString::new("Properties") }
pub fn kAUViewLocalizedStringKey_Parameters()    -> CFString { CFString::new("Parameters") }
pub fn kAUViewLocalizedStringKey_Standard()      -> CFString { CFString::new("Standard") }
pub fn kAUViewLocalizedStringKey_Expert()        -> CFString { CFString::new("Expert") }


/* ------------------ AULoadCPU:  ------------------ */
pub fn kAUViewLocalizedStringKey_RestrictCPULoad() -> CFString { CFString::new("Restrict CPU Load") }
pub fn kAUViewLocalizedStringKey_PercentSymbol()   -> CFString { CFString::new("%") }
pub fn kAUViewLocalizedStringKey_NotApplicable()   -> CFString { CFString::new("n/a") }


/* ----------- AUDiskStreamingCheckbox:  ----------- */
pub fn kAUViewLocalizedStringKey_StreamFromDisk() -> CFString { CFString::new("Stream From Disk") }


/* ------------- AURenderQualityPopup:  ------------- */
pub fn kAUViewLocalizedStringKey_RenderQuality() -> CFString { CFString::new("Render Quality") }
pub fn kAUViewLocalizedStringKey_Maximum()       -> CFString { CFString::new("Maximum") }
pub fn kAUViewLocalizedStringKey_High()          -> CFString { CFString::new("High") }
pub fn kAUViewLocalizedStringKey_Medium()        -> CFString { CFString::new("Medium") }
pub fn kAUViewLocalizedStringKey_Low()           -> CFString { CFString::new("Low") }
pub fn kAUViewLocalizedStringKey_Minimum()       -> CFString { CFString::new("Minimum") }

/* ------------- AUChannelLayoutPopUp:  ------------- */
pub fn kAUViewLocalizedStringKey_AudioChannelLayout() -> CFString { CFString::new("Audio Channel Layout") }
