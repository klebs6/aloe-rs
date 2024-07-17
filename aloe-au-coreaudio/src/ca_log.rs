crate::ix!();

#[cfg(ALOE_COREAUDIO_LOGGING_ENABLED)]
macro_rules! aloe_coreaudiolog {
    ($a:ident) => {
        /*
                { String camsg ("CoreAudio: "); camsg << a; Logger::writeToLog (camsg); }
        */
    }
}

#[cfg(not(ALOE_COREAUDIO_LOGGING_ENABLED))]
macro_rules! aloe_coreaudiolog { ($a:ident) => { } }

