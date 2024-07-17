crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/Vst/aloe_Vst_Wrapper.cpp]

lazy_static!{
    /*
    #if AloePlugin_VersionCode < 0x010000   // Major < 0

     #if (AloePlugin_VersionCode & 0x00FF00) > (9 * 0x100) // check if Minor number exceeds 9
      ALOE_COMPILER_WARNING ("When version has 'major' = 0, Vst2 has trouble displaying 'minor' exceeding 9")
     #endif

     #if (AloePlugin_VersionCode & 0xFF) > 9   // check if Bugfix number exceeds 9
      ALOE_COMPILER_WARNING ("When version has 'major' = 0, Vst2 has trouble displaying 'bugfix' exceeding 9")
     #endif

    #elif AloePlugin_VersionCode >= 0x650000   // Major >= 101

     #if (AloePlugin_VersionCode & 0x00FF00) > (99 * 0x100) // check if Minor number exceeds 99
      ALOE_COMPILER_WARNING ("When version has 'major' > 100, Vst2 has trouble displaying 'minor' exceeding 99")
     #endif

     #if (AloePlugin_VersionCode & 0xFF) > 99  // check if Bugfix number exceeds 99
      ALOE_COMPILER_WARNING ("When version has 'major' > 100, Vst2 has trouble displaying 'bugfix' exceeding 99")
     #endif

    #endif
    */
}

#[cfg(PRAGMA_ALIGN_SUPPORTED)]
pub const PRAGMA_ALIGN_SUPPORTED: usize = 1;

pub const Vst_FORCE_DEPRECATED: usize = 0;

/*
  | If the following files cannot be found then you
  | are probably trying to build a Vst2 plug-in or
  | a Vst2-compatible Vst3 plug-in. To do this you
  | must have a Vst2 SDK in your header search
  | paths or use the "Vst (Legacy) SDK Folder"
  | field in the Proaloer. The Vst2 SDK can be
  | obtained from the
  | vstsdk3610_11_06_2018_build_37 (or older) Vst3
  | SDK or Aloe version 5.3.2. You also need a Vst2
  | license from Steinberg to distribute Vst2
  | plug-ins.
  |#include "pluginterfaces/vst2.x/aeffect.h"
  |#include "pluginterfaces/vst2.x/aeffectx.h"
  */

pub const ALOE_VstINTERFACE_H_INCLUDED:     usize = 1;
pub const ALOE_GUI_BASICS_INCLUDE_XHEADERS: usize = 1;

lazy_static!{
    /*
    static bool recursionCheck = false;
    */
}

#[cfg(target_os="macos")]
lazy_static!{
    /*
    extern  void initialiseMacVst();
    extern  void* attachComponentToWindowRefVst (Component*, void* parent, bool isNSView);
    extern  void detachComponentFromWindowRefVst (Component*, void* window, bool isNSView);
    extern  void setNativeHostWindowSizeVst (void* window, Component*, int newWidth, int newHeight, bool isNSView);
    extern  void checkWindowVisibilityVst (void* window, Component*, bool isNSView);
    extern  bool forwardCurrentKeyEventToHostVst (Component*, bool isNSView);
    */
}

#[cfg(target_os="macos")]
#[cfg(not(ALOE_64BIT))]
lazy_static!{
    /*
    extern  void updateEditorCompBoundsVst (Component*);
    */
}

#[cfg(all(target_os="windows",ALOE_WIN_PER_MONITOR_DPI_AWARE))]
lazy_static!{
    /*
    extern  double getScaleFactorForWindow (HWND);
    */
}

lazy_static!{
    /*
    extern  bool handleManufacturerSpecificVst2Opcode (int32, pointer_sized_int, void*, float);
    */
}
