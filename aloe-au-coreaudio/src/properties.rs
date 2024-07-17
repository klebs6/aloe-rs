crate::ix!();

lazy_static!{
    /*
    constexpr auto aloeAudioObjectPropertyElementMain =
           #if defined (MAC_OS_VERSION_12_0)
            kAudioObjectPropertyElementMain;
           #else
            kAudioObjectPropertyElementMaster;
           #endif
    */
}

lazy_static!{
    /*
    constexpr auto aloeAudioHardwareServiceDeviceProperty_VirtualMainVolume =
           #if defined (MAC_OS_VERSION_12_0)
            kAudioHardwareServiceDeviceProperty_VirtualMainVolume;
           #else
            kAudioHardwareServiceDeviceProperty_VirtualMasterVolume;
           #endif
    */
}

