crate::ix!();

#[cfg(ALOE_DEBUG)]
pub const JACK_LOGGING_ENABLED: usize = 1;

#[cfg(JACK_LOGGING_ENABLED)]
pub fn jack_log(s: &String)  {
    
    todo!();
    /*
        std::cerr << s << std::endl;
    */
}

#[cfg(JACK_LOGGING_ENABLED)]
pub fn get_jack_error_message(status: JackStatus) -> *const u8 {
    
    todo!();
    /*
        if (status & JackServerFailed
                 || status & JackServerError)   return "Unable to connect to JACK server";
            if (status & JackVersionError)      return "Client's protocol version does not match";
            if (status & JackInvalidOption)     return "The operation contained an invalid or unsupported option";
            if (status & JackNameNotUnique)     return "The desired client name was not unique";
            if (status & JackNoSuchClient)      return "Requested client does not exist";
            if (status & JackInitFailure)       return "Unable to initialize client";
            return nullptr;
    */
}

#[cfg(JACK_LOGGING_ENABLED)]
macro_rules! aloe_jack_log_status {
    ($x:ident) => {
        /*
                { if (const char* m = getJackErrorMessage (x)) jack_Log (m); }
        */
    }
}

#[cfg(JACK_LOGGING_ENABLED)]
macro_rules! aloe_jack_log {
    ($x:ident) => {
        /*
                jack_Log(x)
        */
    }
}

#[cfg(not(JACK_LOGGING_ENABLED))]
macro_rules! aloe_jack_log_status {
    ($x:ident) => {
        /*
                {}
        */
    }
}

#[cfg(not(JACK_LOGGING_ENABLED))]
macro_rules! aloe_jack_log {
    ($x:ident) => {
        /*
                {}
        */
    }
}


#[cfg(not(ALOE_JACK_CLIENT_NAME))]
#[cfg(AloePlugin_Name)]
macro_rules! aloe_jack_client_name {
    () => {
        /*
                AloePlugin_Name
        */
    }
}

#[cfg(not(ALOE_JACK_CLIENT_NAME))]
#[cfg(not(AloePlugin_Name))]
macro_rules! aloe_jack_client_name {
    () => {
        /*
                "ALOEJack"
        */
    }
}
