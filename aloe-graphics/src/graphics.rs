crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/aloe_graphics.h]

/**
  | Config: ALOE_USE_COREIMAGE_LOADER
  | 
  | On OSX, enabling this flag means that
  | the CoreImage codecs will be used to
  | load
  | 
  | PNG/JPEG/GIF files. It is enabled by
  | default, but you may want to disable
  | it if you'd rather use libpng, libjpeg,
  | etc.
  |
  */
#[cfg(not(ALOE_USE_COREIMAGE_LOADER))]
pub const ALOE_USE_COREIMAGE_LOADER: usize = 1;

/**
  | Config: ALOE_USE_DIRECTWRITE
  | 
  | Enabling this flag means that DirectWrite
  | will be used when available for font
  | management and layout.
  |
  */
#[cfg(not(ALOE_USE_DIRECTWRITE))]
pub const ALOE_USE_DIRECTWRITE: usize = 1;

/**
  | Config: ALOE_DISABLE_COREGRAPHICS_FONT_SMOOTHING
  | 
  | Setting this flag will turn off CoreGraphics
  | font smoothing on macOS, which some
  | people find makes the text too 'fat'
  | for their taste.
  |
  */
#[cfg(not(ALOE_DISABLE_COREGRAPHICS_FONT_SMOOTHING))]
pub const ALOE_DISABLE_COREGRAPHICS_FONT_SMOOTHING: usize = 0;

#[cfg(not(ALOE_INCLUDE_PNGLIB_CODE))]
pub const ALOE_INCLUDE_PNGLIB_CODE: usize = 1;

#[cfg(not(ALOE_INCLUDE_JPEGLIB_CODE))]
pub const ALOE_INCLUDE_JPEGLIB_CODE: usize = 1;

#[cfg(not(USE_COREGRAPHICS_RENDERING))]
pub const USE_COREGRAPHICS_RENDERING: usize = 1;

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/aloe_graphics.cpp]

pub const ALOE_CORE_INCLUDE_OBJC_HELPERS:             usize = 1;
pub const ALOE_CORE_INCLUDE_COM_SMART_PTR:            usize = 1;
pub const ALOE_CORE_INCLUDE_JNI_HELPERS:              usize = 1;
pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS:           usize = 1;
pub const ALOE_GRAPHICS_INCLUDE_COREGRAPHICS_HELPERS: usize = 1;

#[cfg(any(target_os="linux",target_os="bsd"))]
#[cfg(not(ALOE_USE_FREETYPE))]
pub const ALOE_USE_FREETYPE: usize = 1;

#[cfg(all(any(target_os="macos",target_os="ios"),all(USE_COREGRAPHICS_RENDERING,ALOE_USE_COREIMAGE_LOADER)))]
pub const ALOE_USING_COREIMAGE_LOADER: usize = 1;

#[cfg(not(all(any(target_os="macos",target_os="ios"),all(USE_COREGRAPHICS_RENDERING,ALOE_USE_COREIMAGE_LOADER))))]
pub const ALOE_USING_COREIMAGE_LOADER: usize = 0;
