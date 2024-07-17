crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/aloe_audio_devices.h]

/**
  | Config: ALOE_USE_WINRT_MIDI
  | 
  | Enables the use of the Windows Runtime
  | API for MIDI, allowing connections
  | to Bluetooth Low Energy devices on Windows
  | 10 version 1809 (October 2018 Update)
  | and later. If you enable this flag then
  | older versions of Windows will automatically
  | fall back to using the regular Win32
  | MIDI API.
  | 
  | You will need version 10.0.14393.0
  | of the Windows Standalone SDK to compile
  | and you may need to add the path to the
  | WinRT headers. The path to the headers
  | will be something similar to "C:\Program
  | Files (x86)\Windows Kits\10\Include\10.0.14393.0\winrt".
  |
  */
#[cfg(not(ALOE_USE_WINRT_MIDI))]
pub const ALOE_USE_WINRT_MIDI: usize = 0;

/**
  | Config: ALOE_ASIO
  | 
  | Enables ASIO audio devices (MS Windows
  | only). Turning this on means that you'll
  | need to have the Steinberg ASIO SDK installed
  | on your Windows build machine.
  | 
  | See the comments in the ASIOAudioIODevice
  | class's header file for more info about
  | this.
  |
  */
#[cfg(not(ALOE_ASIO))]
pub const ALOE_ASIO: usize = 0;

/**
  | Config: ALOE_WASAPI
  | 
  | Enables WASAPI audio devices (Windows
  | Vista and above).
  |
  */
#[cfg(not(ALOE_WASAPI))]
pub const ALOE_WASAPI: usize = 1;

/**
  | Config: ALOE_DIRECTSOUND
  | 
  | Enables DirectSound audio (MS Windows
  | only).
  |
  */
#[cfg(not(ALOE_DIRECTSOUND))]
pub const ALOE_DIRECTSOUND: usize = 1;

/**
  | Config: ALOE_ALSA
  | 
  | Enables ALSA audio devices (Linux only).
  |
  */
#[cfg(not(ALOE_ALSA))]
pub const ALOE_ALSA: usize = 1;

/**
  | Config: ALOE_JACK
  | 
  | Enables JACK audio devices (Linux only).
  |
  */
#[cfg(not(ALOE_JACK))]
pub const ALOE_JACK: usize = 0;

/**
  | Config: ALOE_BELA
  | 
  | Enables Bela audio devices on Bela boards.
  |
  */
#[cfg(not(ALOE_BELA))]
pub const ALOE_BELA: usize = 0;

/**
  | Config: ALOE_USE_ANDROID_OBOE
  | 
  | Enables Oboe devices (Android only).
  |
  */
#[cfg(not(ALOE_USE_ANDROID_OBOE))]
pub const ALOE_USE_ANDROID_OBOE: usize = 1;

/**
  | Config: ALOE_USE_OBOE_STABILIZED_CALLBACK
  | 
  | If ALOE_USE_ANDROID_OBOE is enabled,
  | enabling this will wrap output audio
  | streams in the oboe::StabilizedCallback
  | class. This class attempts to keep the
  | CPU spinning to avoid it being scaled
  | down on certain devices. (Android only).
  |
  */
#[cfg(not(ALOE_USE_ANDROID_OBOE_STABILIZED_CALLBACK))]
pub const ALOE_USE_ANDROID_OBOE_STABILIZED_CALLBACK: usize = 0;

/**
  | Config: ALOE_USE_ANDROID_OPENSLES
  | 
  | Enables OpenSLES devices (Android
  | only).
  |
  */
#[cfg(not(ALOE_USE_ANDROID_OPENSLES))]
#[cfg(not(ALOE_USE_ANDROID_OBOE))] pub const ALOE_USE_ANDROID_OPENSLES: usize = 1;

#[cfg(not(ALOE_USE_ANDROID_OPENSLES))]
#[cfg(ALOE_USE_ANDROID_OBOE)]      pub const ALOE_USE_ANDROID_OPENSLES: usize = 0;

/**
  | Config: ALOE_DISABLE_AUDIO_MIXING_WITH_OTHER_APPS
  | 
  | Turning this on gives your app exclusive
  | access to the system's audio on platforms
  | which support it (currently iOS only).
  |
  */
#[cfg(not(ALOE_DISABLE_AUDIO_MIXING_WITH_OTHER_APPS))]
pub const ALOE_DISABLE_AUDIO_MIXING_WITH_OTHER_APPS: usize = 0;


//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/aloe_audio_devices.cpp]

pub const ALOE_CORE_INCLUDE_OBJC_HELPERS:           usize = 1;
pub const ALOE_CORE_INCLUDE_COM_SMART_PTR:          usize = 1;
pub const ALOE_CORE_INCLUDE_JNI_HELPERS:            usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS:         usize = 1;
pub const ALOE_EVENTS_INCLUDE_WIN32_MESSAGE_WINDOW: usize = 1;

#[cfg(ALOE_USE_WINRT_MIDI)]
pub const ALOE_EVENTS_INCLUDE_WINRT_WRAPPER: usize = 1;

//TODO
pub struct SystemAudioVolume {}

/**
  | None of these methods are available.
  | (On Windows you might need to enable WASAPI
  | for this)
  |
  */
#[cfg(not(ALOE_SYSTEMAUDIOVOL_IMPLEMENTED))]
impl SystemAudioVolume {
    
    pub fn get_gain(&mut self) -> f32 {
        
        todo!();
        /*
            jassertfalse; return 0.0f;
        */
    }
    
    pub fn set_gain(&mut self, _0: f32) -> bool {
        
        todo!();
        /*
            jassertfalse; return false;
        */
    }
    
    pub fn is_muted(&mut self) -> bool {
        
        todo!();
        /*
            jassertfalse; return false;
        */
    }
    
    pub fn set_muted(&mut self, _0: bool) -> bool {
        
        todo!();
        /*
            jassertfalse; return false;
        */
    }
}

#[cfg(ALOE_SYSTEMAUDIOVOL_IMPLEMENTED)]
pub const ALOE_SYSTEMAUDIOVOL_IMPLEMENTED: usize = 1;

#[cfg(ALOE_SYSTEMAUDIOVOL_IMPLEMENTED)]
impl SystemAudioVolume {
    
    pub fn get_gain(&mut self) -> f32 {
        
        todo!();
        /*
            return SystemVol (aloeAudioHardwareServiceDeviceProperty_VirtualMainVolume).getGain();
        */
    }
    
    pub fn set_gain(&mut self, gain: f32) -> bool {
        
        todo!();
        /*
            return SystemVol (aloeAudioHardwareServiceDeviceProperty_VirtualMainVolume).setGain (gain);
        */
    }
    
    pub fn is_muted(&mut self) -> bool {
        
        todo!();
        /*
            return SystemVol (kAudioDevicePropertyMute).isMuted();
        */
    }
    
    pub fn set_muted(&mut self, mute: bool) -> bool {
        
        todo!();
        /*
            return SystemVol (kAudioDevicePropertyMute).setMuted (mute);
        */
    }
}
