/*!
  | Macros to redirect debugging output
  | to various logging services
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CADebugPrintf.h]
lazy_static!{
    /*
    #if DEBUG || CoreAudio_Debug

        #if TARGET_OS_WIN32
            #if defined(__cplusplus)
            extern "C"
            #endif
            extern int CAWin32DebugPrintf(char* inFormat, ...);
            #define DebugPrintfRtn          CAWin32DebugPrintf
            #define DebugPrintfFile
            #define DebugPrintfLineEnding   "\n"
            #define DebugPrintfFileComma
        #else
            #if CoreAudio_UseSysLog
                #include <sys/syslog.h>
                #define DebugPrintfRtn  syslog
                #define DebugPrintfFile LOG_NOTICE
                #define DebugPrintfLineEnding   ""
                #define DebugPrintfFileComma    DebugPrintfFile,
            #elif defined(CoreAudio_UseSideFile)
                #include <stdio.h>
                #if defined(__cplusplus)
                extern "C"
                #endif
                void OpenDebugPrintfSideFile();
                extern FILE* sDebugPrintfSideFile;
                #define DebugPrintfRtn  fprintf
                #define DebugPrintfFile ((sDebugPrintfSideFile != NULL) ? sDebugPrintfSideFile : stderr)
                #define DebugPrintfLineEnding   "\n"
                #define DebugPrintfFileComma    DebugPrintfFile,
            #else
                #include <stdio.h>
                #define DebugPrintfRtn  fprintf
                #define DebugPrintfFile stderr
                #define DebugPrintfLineEnding   "\n"
                #define DebugPrintfFileComma    DebugPrintfFile,
            #endif
        #endif

        #define DebugPrintf(inFormat, ...)  DebugPrintfRtn(DebugPrintfFileComma inFormat DebugPrintfLineEnding, ## __VA_ARGS__)
    #else
        #define DebugPrintfRtn
        #define DebugPrintfFile
        #define DebugPrintfLineEnding
        #define DebugPrintfFileComma
        #define DebugPrintf(inFormat, ...)
    #endif
    */
}
