crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAException.h]

pub struct CAException {
    error: OSStatus,
}

impl CAException {

    pub fn new_from_os_status(in_error: OSStatus) -> Self {
    
        todo!();
        /*
        : error(inError),

        
        */
    }
    
    pub fn new(in_exception: &CAException) -> Self {
    
        todo!();
        /*
        : error(inException.mError),

        
        */
    }
    
    pub fn assign_from(&mut self, in_exception: &CAException) -> &mut CAException {
        
        todo!();
        /*
            mError = inException.mError; return *this;
        */
    }
    
    pub fn get_error(&self) -> OSStatus {
        
        todo!();
        /*
            return mError;
        */
    }
}

macro_rules! ca_try {
    () => {
        /*
                try{
        */
    }
}

macro_rules! ca_catch {
    () => {
        /*
                } catch(...) {}
        */
    }
}

macro_rules! ca_swallow_exception {
    ($inExpression:ident) => {
        /*
                try { inExpression; } catch(...) {}
        */
    }
}
