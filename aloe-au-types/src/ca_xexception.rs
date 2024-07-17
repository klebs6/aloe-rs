crate::ix!();

pub type CAXExceptionWarningHandler = fn(msg: *const u8, err: OSStatus) -> *mut c_void;

lazy_static!{
    /*
    static WarningHandler   sWarningHandler;
    */
}

/**
  | An extended exception class that includes
  | the name of the failed operation
  |
  */
pub struct CAXException {
    operation: [u8; 256],
    error:     OSStatus,
}

impl CAXException {

    pub fn new(
        operation: *const u8,
        err:       OSStatus) -> Self {
    
        todo!();
        /*
        : error(err),

            if (operation == NULL)
                    mOperation[0] = '\0';
                else if (strlen(operation) >= sizeof(mOperation)) {
                    memcpy(mOperation, operation, sizeof(mOperation) - 1);
                    mOperation[sizeof(mOperation) - 1] = '\0';
                } else

                strlcpy(mOperation, operation, sizeof(mOperation));
        */
    }
    
    pub fn format_error_from_raw_with_size(
        &self, 
        str_:    *mut u8,
        strsize: usize

    ) -> *mut u8 {
        
        todo!();
        /*
            return FormatError(str, strsize, mError);
        */
    }
    
    pub fn format_error(
        str_:    *mut u8,
        strsize: usize,
        error:   OSStatus) -> *mut u8 {
        
        todo!();
        /*
            strlcpy(str, CAX4CCString(error), strsize);
            return str;
        */
    }
    
    pub fn warning(
        s:     *const u8,
        error: OSStatus)  {
        
        todo!();
        /*
            if (sWarningHandler)
                (*sWarningHandler)(s, error);
        */
    }
    
    pub fn set_warning_handler(f: CAXExceptionWarningHandler)  {
        
        todo!();
        /*
            sWarningHandler = f;
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! xthrowiferror {
    ($error:ident, $operation:ident) => {
        /*
        
                do {                                                                    
                    OSStatus __err = error;                                             
                    if (__err) {                                                        
                        DebugMessageN4("%s:%d: about to throw %s: %s", __FILE__, __LINE__, CAX4CCString(__err).get(), operation);
                        __THROW_STOP;                                                           
                        throw CAXException(operation, __err);                           
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! xthrowif {
    ($condition:ident, $error:ident, $operation:ident) => {
        /*
        
                do {                                                                    
                    if (condition) {                                                    
                        OSStatus __err = error;                                         
                        DebugMessageN4("%s:%d: about to throw %s: %s", __FILE__, __LINE__, CAX4CCString(__err).get(), operation);
                        __THROW_STOP;                                                           
                        throw CAXException(operation, __err);                           
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! xrequirenoerror {
    ($error:ident, $label:ident) => {
        /*
        
                do {                                                                    
                    OSStatus __err = error;                                             
                    if (__err) {                                                        
                        DebugMessageN4("%s:%d: about to throw %s: %s", __FILE__, __LINE__, CAX4CCString(__err).get(), #error);
                        STOP;                                                           
                        goto label;                                                     
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! xassert {
    ($assertion:ident) => {
        /*
        
                do {                                                                    
                    if (!(assertion)) {                                                 
                        DebugMessageN3("%s:%d: error: failed assertion: %s", __FILE__, __LINE__, #assertion);       
                        __ASSERT_STOP;                                                          
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! xassertnoerror {
    ($error:ident) => {
        /*
        
                do {                                                                    
                    OSStatus __err = error;                                             
                    if (__err) {                                                        
                        DebugMessageN4("%s:%d: error %s: %s", __FILE__, __LINE__, CAX4CCString(__err).get(), #error);
                        STOP;                                                           
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! ca_require_noerr {
    ($errorCode:ident, $exceptionLabel:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    int evalOnceErrorCode = (errorCode);                                
                    if ( __builtin_expect(0 != evalOnceErrorCode, 0) )                  
                    {                                                                   
                        DebugMessageN5("ca_require_noerr: [%s, %d] (goto %s;) %s:%d",   
                            #errorCode, evalOnceErrorCode,                              
                            #exceptionLabel,                                            
                            __FILE__,                                                   
                            __LINE__);                                                  
                        goto exceptionLabel;                                            
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! ca_verify_noerr {
    ($errorCode:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    int evalOnceErrorCode = (errorCode);                                
                    if ( __builtin_expect(0 != evalOnceErrorCode, 0) )                  
                    {                                                                   
                        DebugMessageN4("ca_verify_noerr: [%s, %d] %s:%d",               
                            #errorCode, evalOnceErrorCode,                              
                            __FILE__,                                                   
                            __LINE__);                                                  
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! ca_debug_string {
    ($message:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    DebugMessageN3("ca_debug_string: %s %s:%d",                         
                        message,                                                        
                        __FILE__,                                                       
                        __LINE__);                                                      
                } while ( 0 )
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! ca_verify {
    ($assertion:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    if ( __builtin_expect(!(assertion), 0) )                            
                    {                                                                   
                        DebugMessageN3("ca_verify: %s %s:%d",                           
                            #assertion,                                                 
                            __FILE__,                                                   
                            __LINE__);                                                  
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! ca_require {
    ($assertion:ident, $exceptionLabel:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    if ( __builtin_expect(!(assertion), 0) )                            
                    {                                                                   
                        DebugMessageN4("ca_require: %s %s %s:%d",                       
                            #assertion,                                                 
                            #exceptionLabel,                                            
                            __FILE__,                                                   
                            __LINE__);                                                  
                        goto exceptionLabel;                                            
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(any(DEBUG,CoreAudio_Debug))]
macro_rules! ca_check {
    ($assertion:ident) => {
        /*
        
              do                                                                        
              {                                                                         
                  if ( __builtin_expect(!(assertion), 0) )                              
                  {                                                                     
                      DebugMessageN3("ca_check: %s %s:%d",                          
                          #assertion,                                                   
                          __FILE__,                                                     
                          __LINE__);                                                    
                  }                                                                     
              } while ( 0 )
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! x_throw_if_error {
    ($error:ident, $operation:ident) => {
        /*
        
                do {                                                                    
                    OSStatus __err = error;                                             
                    if (__err) {                                                        
                        throw CAXException(operation, __err);                           
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! x_throw_if {
    ($condition:ident, $error:ident, $operation:ident) => {
        /*
        
                do {                                                                    
                    if (condition) {                                                    
                        OSStatus __err = error;                                         
                        throw CAXException(operation, __err);                           
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! x_require_no_error {
    ($error:ident, $label:ident) => {
        /*
        
                do {                                                                    
                    OSStatus __err = error;                                             
                    if (__err) {                                                        
                        goto label;                                                     
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! x_assert {
    ($assertion:ident) => {
        /*
        
                do {                                                                    
                    if (!(assertion)) {                                                 
                    }                                                                   
                } while (0)
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! x_assert_no_error {
    ($error:ident) => {
        /*
        
                do {                                                                    
                    /*OSStatus __err =*/ error;                                         
                } while (0)
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! ca_require_noerr {
    ($errorCode:ident, $exceptionLabel:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    if ( __builtin_expect(0 != (errorCode), 0) )                        
                    {                                                                   
                        goto exceptionLabel;                                            
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! ca_verify_noerr {
    ($errorCode:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    if ( 0 != (errorCode) )                                             
                    {                                                                   
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! ca_debug_string { ($message:ident) => { } }

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! ca_verify {
    ($assertion:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    if ( !(assertion) )                                                 
                    {                                                                   
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! ca_require {
    ($assertion:ident, $exceptionLabel:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    if ( __builtin_expect(!(assertion), 0) )                            
                    {                                                                   
                        goto exceptionLabel;                                            
                    }                                                                   
                } while ( 0 )
        */
    }
}

#[cfg(not(any(DEBUG,CoreAudio_Debug)))]
macro_rules! ca_check {
    ($assertion:ident) => {
        /*
        
                do                                                                      
                {                                                                       
                    if ( !(assertion) )                                                 
                    {                                                                   
                    }                                                                   
                } while ( 0 )
        */
    }
}

///-----------------------------
macro_rules! xthrow {
    ($error:ident, $operation:ident) => {
        /*
                XThrowIf(true, error, operation)
        */
    }
}

macro_rules! xthrowiferr {
    ($error:ident) => {
        /*
                XThrowIfError(error, #error)
        */
    }
}
