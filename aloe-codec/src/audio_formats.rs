crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/aloe_audio_formats.h]

/**
  | Config: ALOE_USE_FLAC
  | 
  | Enables the FLAC audio codec classes
  | (available on all platforms).
  | 
  | If your app doesn't need to read FLAC
  | files, you might want to disable this
  | to reduce the size of your codebase and
  | build time.
  |
  */
#[cfg(not(ALOE_USE_FLAC))]
pub const ALOE_USE_FLAC: usize = 1;

/**
  | Config: ALOE_USE_OGGVORBIS
  | 
  | Enables the Ogg-Vorbis audio codec
  | classes (available on all platforms).
  | 
  | If your app doesn't need to read Ogg-Vorbis
  | files, you might want to disable this
  | to reduce the size of your codebase and
  | build time.
  |
  */
#[cfg(not(ALOE_USE_OGGVORBIS))]
pub const ALOE_USE_OGGVORBIS: usize = 1;

/**
  | Config: ALOE_USE_MP3AUDIOFORMAT
  | 
  | Enables the software-based MP3AudioFormat
  | class.
  | 
  | IMPORTANT DISCLAIMER: By choosing
  | to enable the ALOE_USE_MP3AUDIOFORMAT
  | flag and to compile this MP3 code into
  | your software, you do so AT YOUR OWN RISK!
  | By doing so, you are agreeing that Raw
  | Material Software Limited is in no way
  | responsible for any patent, copyright,
  | or other legal issues that you may suffer
  | as a result.
  | 
  | The code in aloe_MP3AudioFormat.cpp
  | is NOT guaranteed to be free from infringements
  | of 3rd-party intellectual property.
  | If you wish to use it, please seek your
  | own independent advice about the legality
  | of doing so. If you are not willing to
  | accept full responsibility for the
  | consequences of using this code, then
  | do not enable this setting.
  |
  */
#[cfg(not(ALOE_USE_MP3AUDIOFORMAT))]
pub const ALOE_USE_MP3AUDIOFORMAT: usize = 0;

/**
  | Config: ALOE_USE_LAME_AUDIO_FORMAT
  | 
  | Enables the LameEncoderAudioFormat
  | class.
  |
  */
#[cfg(not(ALOE_USE_LAME_AUDIO_FORMAT))]
pub const ALOE_USE_LAME_AUDIO_FORMAT: usize = 0;

/**
  | Config: ALOE_USE_WINDOWS_MEDIA_FORMAT
  | 
  | Enables the Windows Media SDK codecs.
  |
  */
lazy_static!{
    /*
    #[cfg(not(ALOE_USE_WINDOWS_MEDIA_FORMAT))]
    pub const ALOE_USE_WINDOWS_MEDIA_FORMAT: usize = 1;

    #[cfg(any(not(target_os="windows"),ALOE_MINGW))]
    pub const ALOE_USE_WINDOWS_MEDIA_FORMAT: usize = 0;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/aloe_audio_formats.cpp]

pub const ALOE_CORE_INCLUDE_COM_SMART_PTR:  usize = 1;
pub const ALOE_CORE_INCLUDE_JNI_HELPERS:    usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS: usize = 1;

