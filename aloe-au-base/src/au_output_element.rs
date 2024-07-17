crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUOutputElement.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUOutputElement.cpp]

pub struct AUOutputElement {
    base: AUIOElement,
}

impl AUElementInterface for AUOutputElement { 

    fn get_number_of_parameters(&mut self) -> u32
    {
        todo!();
    }

    fn get_parameter_list(&mut self, out_list: *mut AudioUnitParameterID)
    {
        todo!();
    }

    fn use_indexed_parameters(&mut self, in_number_of_parameters: i32)
    {
        todo!();
    }

    fn as_io_element(&mut self) -> *mut AUIOElement
    {
        todo!();
    }
}

impl SetStreamFormat for AUOutputElement {

    fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

impl NeedsBufferSpace for AUOutputElement {

    fn needs_buffer_space(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl AUOutputElement {
    
    pub fn new(audio_unit: *mut AUBase) -> Self {
    
        todo!();
        /*
        : auio_element(audioUnit),

            AllocateBuffer();
        */
    }
    
    pub fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
            OSStatus result = AUIOElement::SetStreamFormat(desc);   // inherited
        if (result == AUBase::noErr)
            AllocateBuffer();
        return result;
        */
    }
}
