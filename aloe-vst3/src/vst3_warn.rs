crate::ix!();

#[cfg(not(ALOE_Vst3_DEBUGGING))]
pub const ALOE_Vst3_DEBUGGING: usize = 0;

#[cfg(ALOE_Vst3_DEBUGGING)]
macro_rules! vst3_dbg {
    ($a:ident) => {
        /*
                Logger::writeToLog (a);
        */
    }
}

#[cfg(not(ALOE_Vst3_DEBUGGING))]
macro_rules! vst3_dbg { ($a:ident) => { } }

#[cfg(ALOE_DEBUG)]
pub fn warn_on_failure(result: i32) -> i32 {
    
    todo!();
        /*
            const char* message = "Unknown result!";

        switch (result)
        {
            case kResultOk:         return result;
            case kNotImplemented:   message = "kNotImplemented";  break;
            case kNoInterface:      message = "kNoInterface";     break;
            case kResultFalse:      message = "kResultFalse";     break;
            case kInvalidArgument:  message = "kInvalidArgument"; break;
            case kInternalError:    message = "kInternalError";   break;
            case kNotInitialized:   message = "kNotInitialized";  break;
            case kOutOfMemory:      message = "kOutOfMemory";     break;
            default:                break;
        }

        DBG (message);
        return result;
        */
}

#[cfg(ALOE_DEBUG)]
pub fn warn_on_failure_if_implemented(result: i32) -> i32 {
    
    todo!();
        /*
            if (result != kResultOk && result != kNotImplemented)
            return warnOnFailure (result);

        return result;
        */
}

#[cfg(not(ALOE_DEBUG))]
macro_rules! warn_on_failure {
    ($x:ident) => {
        /*
                x
        */
    }
}

#[cfg(not(ALOE_DEBUG))]
macro_rules! warn_on_failure_if_implemented {
    ($x:ident) => {
        /*
                x
        */
    }
}
