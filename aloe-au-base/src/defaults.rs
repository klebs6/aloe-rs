crate::ix!();

pub const kAUDefaultSampleRate: f32 = 44100.0;

#[cfg(not(TARGET_OS_WIN32))]
pub const kAUDefaultMaxFramesPerSlice: usize = 1156;

/**
  this allows enough default frames for a 512 dest
  44K and SRC from 96K add a padding of 4 frames
  for any altivec rounding
  */
#[cfg(TARGET_OS_WIN32)]
pub const kAUDefaultMaxFramesPerSlice: usize = 2048;

lazy_static!{
    /*
    static SInt32   sVectorUnitType;
    SInt32 AUBase::sVectorUnitType = kVecUninitialized;
    */
}

lazy_static!{
    /*
    static const Float64 kNoLastRenderedSampleTime;
    const Float64 AUBase::kNoLastRenderedSampleTime = -1.;

    */
}

#[cfg(not(CA_BASIC_AU_FEATURES))]
pub const AU_BASE_K_NUM_SCOPES: usize = 4;

#[cfg(CA_BASIC_AU_FEATURES)]
pub const AU_BASE_K_NUM_SCOPES: usize = 3;

lazy_static!{
    /*
    static bool sAUBaseCFStringsInitialized = false;
    // this is used for the presets
    static CFStringRef kUntitledString = NULL;
    //these are the current keys for the class info document
    static CFStringRef kVersionString = NULL;
    static CFStringRef kTypeString = NULL;
    static CFStringRef kSubtypeString = NULL;
    static CFStringRef kManufacturerString = NULL;
    static CFStringRef kDataString = NULL;
    static CFStringRef kNameString = NULL;
    static CFStringRef kRenderQualityString = NULL;
    static CFStringRef kCPULoadString = NULL;
    static CFStringRef kElementNameString = NULL;
    static CFStringRef kPartString = NULL;
    */
}

pub fn add_num_to_dictionary(
    dict:  CFMutableDictionaryRef,
    key:   CFStringRef,
    value: i32

) {
    
    todo!();
        /*
            CFNumberRef num = CFNumberCreate (NULL, kCFNumberSInt32Type, &value);
        CFDictionarySetValue (dict, key, num);
        CFRelease (num);
        */
}

pub const kCurrentSavedStateVersion: usize = 0;
