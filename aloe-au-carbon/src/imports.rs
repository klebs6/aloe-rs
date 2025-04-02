pub(crate) use core_foundation::string::CFStringRef;

pub(crate) use coreaudio_sys::{
    AUEventListenerRef,
    AUParameterListenerRef,
    AUPreset,
    AudioBufferList,
    AudioChannelLabel,
    AudioChannelLayout,
    AudioChannelLayoutTag,
    AudioDeviceID,
    AudioDeviceIOProcID,
    AudioObjectPropertyAddress,
    AudioObjectPropertyScope,
    AudioObjectPropertySelector,
    AudioTimeStamp,
    AudioUnit,
    AudioUnitEvent,
    AudioUnitParameterInfo,
    AudioUnitProperty,
    AudioUnitPropertyID,
    CFArrayRef,
    CFMutableDictionaryRef,
    Float32,
    Float32Point,
    OSStatus,
    OSType,
    Point,
};

pub(crate) use aloe_au_component::*;
pub(crate) use aloe_au_imports::*;
pub(crate) use aloe_au_parameter::*;
pub(crate) use aloe_derive::*;
pub(crate) use aloe_3p::*;
