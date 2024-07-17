crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CADebugMacros.h]

#[cfg(TARGET_RT_BIG_ENDIAN)]
macro_rules! ca4cctocstring {
    ($the4CC:ident) => {
        /*
                { ((char*)&the4CC)[0], ((char*)&the4CC)[1], ((char*)&the4CC)[2], ((char*)&the4CC)[3], 0 }
        */
    }
}

#[cfg(TARGET_RT_BIG_ENDIAN)]
macro_rules! cacopy4cctocstring {
    ($theCString:ident, $the4CC:ident) => {
        /*
                { theCString[0] = ((char*)&the4CC)[0]; theCString[1] = ((char*)&the4CC)[1]; theCString[2] = ((char*)&the4CC)[2]; theCString[3] = ((char*)&the4CC)[3]; theCString[4] = 0; }
        */
    }
}

#[cfg(not(TARGET_RT_BIG_ENDIAN))]
macro_rules! ca4cctocstring {
    ($the4CC:ident) => {
        /*
                { ((char*)&the4CC)[3], ((char*)&the4CC)[2], ((char*)&the4CC)[1], ((char*)&the4CC)[0], 0 }
        */
    }
}

#[cfg(not(TARGET_RT_BIG_ENDIAN))]
macro_rules! cacopy4cctocstring {
    ($theCString:ident, $the4CC:ident) => {
        /*
                { theCString[0] = ((char*)&the4CC)[3]; theCString[1] = ((char*)&the4CC)[2]; theCString[2] = ((char*)&the4CC)[1]; theCString[3] = ((char*)&the4CC)[0]; theCString[4] = 0; }
        */
    }
}

/**
  |  This is a macro that does a sizeof and casts
  |  the result to a UInt32. This is useful for all
  |  the places where -wshorten64-32 catches
  |  assigning a sizeof expression to a UInt32.
  |
  |  For want of a better place to park this, we'll
  |  park it here.
  */
macro_rules! sizeof32 {
    ($X:ident) => {
        /*
                ((UInt32)sizeof(X))
        */
    }
}

/**
  |  This is a macro that does a offsetof and casts
  |  the result to a UInt32. This is useful for all
  |  the places where -wshorten64-32 catches
  |  assigning an offsetof expression to a UInt32.
  |
  |  For want of a better place to park this, we'll
  |  park it here.
  */
macro_rules! offsetof32 {
    ($X:ident, $Y:ident) => {
        /*
                ((UInt32)offsetof(X, Y))
        */
    }
}

/**
  |  This macro casts the expression to
  |  a UInt32. It is called out specially to allow
  |  us to track casts that have been added purely
  |  to avert -wshorten64-32 warnings on 64 bit
  |  platforms.
  |
  |  For want of a better place to park this, we'll
  |  park it here.
  */
macro_rules! touint32 {
    ($X:ident) => {
        /*
                ((UInt32)(X))
        */
    }
}

macro_rules! tosint32 {
    ($X:ident) => {
        /*
                ((SInt32)(X))
        */
    }
}

lazy_static!{
    /*
    #if DEBUG || CoreAudio_Debug
        // can be used to break into debugger immediately, also see CADebugger
        #define BusError()      { long* p=NULL; *p=0; }

        //  basic debugging print routines
        #if TARGET_OS_MAC && !TARGET_API_MAC_CARBON
            extern void DebugStr(const unsigned char* debuggerMsg);
            #define DebugMessage(msg)   DebugStr("\p"msg)
            #define DebugMessageN1(msg, N1)
            #define DebugMessageN2(msg, N1, N2)
            #define DebugMessageN3(msg, N1, N2, N3)
        #else
            #include "CADebugPrintf.h"

            #if (CoreAudio_FlushDebugMessages && !CoreAudio_UseSysLog) || defined(CoreAudio_UseSideFile)
                #define FlushRtn    ,fflush(DebugPrintfFile)
            #else
                #define FlushRtn
            #endif

            #if     CoreAudio_ThreadStampMessages
                #include <pthread.h>
                #include "CAHostTimeBase.h"
                #if TARGET_RT_64_BIT
                    #define DebugPrintfThreadIDFormat   "%16p"
                #else
                    #define DebugPrintfThreadIDFormat   "%8p"
                #endif
                #define DebugMsg(inFormat, ...) DebugPrintf("%17qd: " DebugPrintfThreadIDFormat " " inFormat, CAHostTimeBase::GetCurrentTimeInNanos(), pthread_self(), ## __VA_ARGS__) FlushRtn
            #elif   CoreAudio_TimeStampMessages
                #include "CAHostTimeBase.h"
                #define DebugMsg(inFormat, ...) DebugPrintf("%17qd: " inFormat, CAHostTimeBase::GetCurrentTimeInNanos(), ## __VA_ARGS__) FlushRtn
            #else
                #define DebugMsg(inFormat, ...) DebugPrintf(inFormat, ## __VA_ARGS__) FlushRtn
            #endif
        #endif
        void    DebugPrint(const char *fmt, ...);   // can be used like printf
        #ifndef DEBUGPRINT
            #define DEBUGPRINT(msg) DebugPrint msg      // have to double-parenthesize arglist (see Debugging.h)
        #endif
        #if VERBOSE
            #define vprint(msg) DEBUGPRINT(msg)
        #else
            #define vprint(msg)
        #endif

        // Original macro keeps its function of turning on and off use of CADebuggerStop() for both asserts and throws.
        // For backwards compat, it overrides any setting of the two sub-macros.
        #if CoreAudio_StopOnFailure
            #include "CADebugger.h"
            #undef CoreAudio_StopOnAssert
            #define CoreAudio_StopOnAssert 1
            #undef CoreAudio_StopOnThrow
            #define CoreAudio_StopOnThrow 1
            #define STOP    CADebuggerStop()
        #else
            #define STOP
        #endif

        #if CoreAudio_StopOnAssert
            #if !CoreAudio_StopOnFailure
                #include "CADebugger.h"
                #define STOP
            #endif
            #define __ASSERT_STOP CADebuggerStop()
        #else
            #define __ASSERT_STOP
        #endif

        #if CoreAudio_StopOnThrow
            #if !CoreAudio_StopOnFailure
                #include "CADebugger.h"
                #define STOP
            #endif
            #define __THROW_STOP CADebuggerStop()
        #else
            #define __THROW_STOP
        #endif

    #else
        #define DebugMsg(inFormat, ...)
        #ifndef DEBUGPRINT
            #define DEBUGPRINT(msg)
        #endif
        #define vprint(msg)
        #define STOP
        #define __ASSERT_STOP
        #define __THROW_STOP
    #endif
    */
}

/**
    Old-style numbered DebugMessage calls are
    implemented in terms of DebugMsg() now
  */
macro_rules! debugmessage {
    ($msg:ident) => {
        /*
                DebugMsg(msg)
        */
    }
}

macro_rules! debugmessagen1 {
    ($msg:ident, $N1:ident) => {
        /*
                DebugMsg(msg, N1)
        */
    }
}

macro_rules! debugmessagen2 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident) => {
        /*
                DebugMsg(msg, N1, N2)
        */
    }
}

macro_rules! debugmessagen3 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident, 
     $N3:ident) => {
        /*
                DebugMsg(msg, N1, N2, N3)
        */
    }
}

macro_rules! debugmessagen4 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident, 
     $N3:ident, 
     $N4:ident) => {
        /*
                DebugMsg(msg, N1, N2, N3, N4)
        */
    }
}

macro_rules! debugmessagen5 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident, 
     $N3:ident, 
     $N4:ident, 
     $N5:ident) => {
        /*
                DebugMsg(msg, N1, N2, N3, N4, N5)
        */
    }
}

macro_rules! debugmessagen6 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident, 
     $N3:ident, 
     $N4:ident, 
     $N5:ident, 
     $N6:ident) => {
        /*
                DebugMsg(msg, N1, N2, N3, N4, N5, N6)
        */
    }
}

macro_rules! debugmessagen7 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident, 
     $N3:ident, 
     $N4:ident, 
     $N5:ident, 
     $N6:ident, 
     $N7:ident) => {
        /*
                DebugMsg(msg, N1, N2, N3, N4, N5, N6, N7)
        */
    }
}

macro_rules! debug_messagen8 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident, 
     $N3:ident, 
     $N4:ident, 
     $N5:ident, 
     $N6:ident, 
     $N7:ident, 
     $N8:ident) => {
        /*
                DebugMsg(msg, N1, N2, N3, N4, N5, N6, N7, N8)
        */
    }
}

macro_rules! debug_messagen9 {
    ($msg:ident, 
     $N1:ident, 
     $N2:ident, 
     $N3:ident, 
     $N4:ident, 
     $N5:ident, 
     $N6:ident, 
     $N7:ident, 
     $N8:ident, 
     $N9:ident) => {
        /*
                DebugMsg(msg, N1, N2, N3, N4, N5, N6, N7, N8, N9)
        */
    }
}

/**
   writes to syslog (and stderr if debugging)
  */
pub fn log_error(
        fmt:  *const u8,
        args: &[&str])  {
    
    todo!();
        /*
        
        */
}

/**
   writes to syslog (and stderr if debugging)
  */
pub fn log_warning(
        fmt:  *const u8,
        args: &[&str])  {
    
    todo!();
        /*
        
        */
}

macro_rules! no_action {
    () => {
        /*
                (void)0
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! assert {
    ($inCondition:ident, 
     $inMessage:ident) => {
        /*
        
                    if(!(inCondition))                                                          
                    {                                                                           
                        DebugMessage(inMessage);                                                
                        __ASSERT_STOP;                                                                  
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! assertfileline {
    ($inCondition:ident, 
     $inMessage:ident) => {
        /*
        
                    if(!(inCondition))                                                          
                    {                                                                           
                        DebugMessageN3("%s, line %d: %s", __FILE__, __LINE__, inMessage);       
                        __ASSERT_STOP;                                                          
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! assertnoerror {
    ($inError:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        SInt32 __Err = (inError);                                               
                        if(__Err != 0)                                                          
                        {                                                                       
                            char __4CC[5] = CA4CCToCString(__Err);                              
                            DebugMessageN2(inMessage ", Error: %d (%s)", (int)__Err, __4CC);        
                            __ASSERT_STOP;                                                      
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! assertnokernelerror {
    ($inError:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        unsigned int __Err = (unsigned int)(inError);                           
                        if(__Err != 0)                                                          
                        {                                                                       
                            DebugMessageN1(inMessage ", Error: 0x%X", __Err);                   
                            __ASSERT_STOP;                                                      
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! assertnotnull {
    ($inPtr:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        if((inPtr) == NULL)                                                     
                        {                                                                       
                            DebugMessage(inMessage);                                            
                            __ASSERT_STOP;                                                      
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failif {
    ($inCondition:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        DebugMessage(inMessage);                                                
                        STOP;                                                                   
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failwithaction {
    ($inCondition:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        DebugMessage(inMessage);                                                
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failifnull {
    ($inPointer:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inPointer) == NULL)                                                     
                    {                                                                           
                        DebugMessage(inMessage);                                                
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failifkernelerror {
    ($inKernelError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        unsigned int __Err = (inKernelError);                                   
                        if(__Err != 0)                                                          
                        {                                                                       
                            DebugMessageN1(inMessage ", Error: 0x%X", __Err);                   
                            STOP;                                                               
                            { inAction; }                                                       
                            goto inHandler;                                                     
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failiferror {
    ($inError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        SInt32 __Err = (inError);                                               
                        if(__Err != 0)                                                          
                        {                                                                       
                            char __4CC[5] = CA4CCToCString(__Err);                              
                            DebugMessageN2(inMessage ", Error: %ld (%s)", (long int)__Err, __4CC);  
                            STOP;                                                               
                            { inAction; }                                                       
                            goto inHandler;                                                     
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failifnomessage {
    ($inCondition:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        STOP;                                                                   
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failwithactionnomessage {
    ($inCondition:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failifnullnomessage {
    ($inPointer:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inPointer) == NULL)                                                     
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failifkernelerrornomessage {
    ($inKernelError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        unsigned int __Err = (inKernelError);                                   
                        if(__Err != 0)                                                          
                        {                                                                       
                            STOP;                                                               
                            { inAction; }                                                       
                            goto inHandler;                                                     
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! failiferrornomessage {
    ($inError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        SInt32 __Err = (inError);                                               
                        if(__Err != 0)                                                          
                        {                                                                       
                            STOP;                                                               
                            { inAction; }                                                       
                            goto inHandler;                                                     
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! throw {
    ($inException:ident) => {
        /*
                __THROW_STOP; throw (inException)
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! throw_if {
    ($inCondition:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        DebugMessage(inMessage);                                                
                        Throw(inException);                                                     
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! throw_ifnull {
    ($inPointer:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inPointer) == NULL)                                                     
                    {                                                                           
                        DebugMessage(inMessage);                                                
                        Throw(inException);                                                     
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! throw_if_kernel_error {
    ($inKernelError:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        int __Err = (inKernelError);                                            
                        if(__Err != 0)                                                          
                        {                                                                       
                            DebugMessageN1(inMessage ", Error: 0x%X", __Err);                   
                            Throw(inException);                                                 
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! throw_if_error {
    ($inError:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        SInt32 __Err = (inError);                                               
                        if(__Err != 0)                                                          
                        {                                                                       
                            char __4CC[5] = CA4CCToCString(__Err);                              
                            DebugMessageN2(inMessage ", Error: %d (%s)", (int)__Err, __4CC);    
                            Throw(inException);                                                 
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
#[cfg(TARGET_OS_WIN32)]
macro_rules! throw_if_win_error {
    ($inError:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        HRESULT __Err = (inError);                                              
                        if(FAILED(__Err))                                                       
                        {                                                                       
                            DebugMessageN2(inMessage ", Code: %d, Facility: 0x%X", HRESULT_CODE(__Err), HRESULT_FACILITY(__Err));           
                            Throw(inException);                                                 
                        }                                                                       
                    }
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! subclass_responsibility {
    ($inMethodName:ident, 
     $inException:ident) => {
        /*
        
                    {                                                                           
                        DebugMessage(inMethodName": Subclasses must implement this method");    
                        Throw(inException);                                                     
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! assert {
    ($inCondition:ident, 
     $inMessage:ident) => {
        /*
        
                    if(!(inCondition))                                                          
                    {                                                                           
                        __ASSERT_STOP;                                                          
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! assert_file_line {
    ($inCondition:ident, 
     $inMessage:ident) => {
        /*
                Assert(inCondition, inMessage)
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! assert_no_error {
    ($inError:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        SInt32 __Err = (inError);                                               
                        if(__Err != 0)                                                          
                        {                                                                       
                            __ASSERT_STOP;                                                      
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! assert_no_kernel_error {
    ($inError:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        unsigned int __Err = (unsigned int)(inError);                           
                        if(__Err != 0)                                                          
                        {                                                                       
                            __ASSERT_STOP;                                                      
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! assert_notnull {
    ($inPtr:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        if((inPtr) == NULL)                                                     
                        {                                                                       
                            __ASSERT_STOP;                                                      
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_if {
    ($inCondition:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        STOP;                                                                   
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_with_action {
    ($inCondition:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_ifnull {
    ($inPointer:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inPointer) == NULL)                                                     
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_if_kernel_error {
    ($inKernelError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inKernelError) != 0)                                                    
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_if_error {
    ($inError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inError) != 0)                                                          
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_if_no_message {
    ($inCondition:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        STOP;                                                                   
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_with_action_no_message {
    ($inCondition:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_if_null_no_message {
    ($inPointer:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inPointer) == NULL)                                                     
                    {                                                                           
                        STOP;                                                                   
                        { inAction; }                                                           
                        goto inHandler;                                                         
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_if_kernel_error_no_message {
    ($inKernelError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        unsigned int __Err = (inKernelError);                                   
                        if(__Err != 0)                                                          
                        {                                                                       
                            STOP;                                                               
                            { inAction; }                                                       
                            goto inHandler;                                                     
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! fail_if_error_no_message {
    ($inError:ident, 
     $inAction:ident, 
     $inHandler:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        SInt32 __Err = (inError);                                               
                        if(__Err != 0)                                                          
                        {                                                                       
                            STOP;                                                               
                            { inAction; }                                                       
                            goto inHandler;                                                     
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! throw {
    ($inException:ident) => {
        /*
                __THROW_STOP; throw (inException)
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! throw_if {
    ($inCondition:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    if(inCondition)                                                             
                    {                                                                           
                        Throw(inException);                                                     
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! throw_ifnull {
    ($inPointer:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    if((inPointer) == NULL)                                                     
                    {                                                                           
                        Throw(inException);                                                     
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! throw_if_kernel_error {
    ($inKernelError:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        int __Err = (inKernelError);                                            
                        if(__Err != 0)                                                          
                        {                                                                       
                            Throw(inException);                                                 
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! throw_if_error {
    ($inError:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        SInt32 __Err = (inError);                                               
                        if(__Err != 0)                                                          
                        {                                                                       
                            Throw(inException);                                                 
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
#[cfg(TARGET_OS_WIN32)]
macro_rules! throw_if_win_error {
    ($inError:ident, 
     $inException:ident, 
     $inMessage:ident) => {
        /*
        
                    {                                                                           
                        HRESULT __Err = (inError);                                              
                        if(FAILED(__Err))                                                       
                        {                                                                       
                            Throw(inException);                                                 
                        }                                                                       
                    }
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! subclass_responsibility {
    ($inMethodName:ident, 
     $inException:ident) => {
        /*
        
                    {                                                                           
                        Throw(inException);                                                     
                    }
        */
    }
}
