crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/hosting/hostclasses.cpp]

/**
  | Implementation's example of IMessage.
  | \ingroup hostingBase
  |
  */
#[DECLARE_FUNKNOWN_METHODS]
pub struct HostMessage {
    message_id:     *mut u8,
    attribute_list: *mut HostAttributeList,
}

implement_funknown_methods!{
    HostMessage, 
    IMessage, 
    IMessage::iid
}

impl FUnknown for HostMessage {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl IMessage for HostMessage {

    fn get_messageid(&mut self) -> &'static str {
        
        todo!();
        /*
            return messageId;
        */
    }
    
    fn set_messageid(&mut self, mid: &str)  {
        
        todo!();
        /*
            if (messageId)
            delete[] messageId;
        messageId = nullptr;
        if (mid)
        {
            size_t len = strlen (mid) + 1;
            messageId = new char[len];
            strcpy (messageId, mid);
        }
        */
    }
    
    fn get_attributes(&mut self) 
        -> *mut dyn IAttributeList<AttrID = *const u8> 
    {
        todo!();
        /*
            if (!attributeList)
            attributeList = new HostAttributeList;
        return attributeList;
        */
    }
}

impl Drop for HostMessage {

    fn drop(&mut self) {
        todo!();
        /*
            setMessageID (nullptr);
        if (attributeList)
            attributeList->release ();
        FUNKNOWN_DTOR
        */
    }
}

impl Default for HostMessage {

    fn default() -> Self {
    
        todo!();
        /*
        : message_id(nullptr),
        : attribute_list(nullptr),

            FUNKNOWN_CTOR
        */
    }
}
