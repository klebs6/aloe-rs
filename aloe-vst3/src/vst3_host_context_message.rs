crate::ix!();

#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[ALOE_DECLARE_Vst3_COM_QUERY_METHODS]
#[no_copy]
#[leak_detector]
pub struct Vst3HostContextMessage {
    value:          Var,
    attribute_list: VstComSmartPtr<Box<dyn IAttributeList<AttrID = *const u8>>>,
    message_id:     String,
    ref_count:      Atomic<i32>,
}

impl IMessage for Vst3HostContextMessage {

    fn get_messageid(&mut self) -> &'static str { todo!() }

    fn set_messageid(&mut self, _: &'static str) { todo!() }

    fn get_attributes(&mut self) -> *mut (dyn aloe_vst_attributes::IAttributeList<AttrID = *const u8> + 'static) { todo!() }
}

impl FUnknown for Vst3HostContextMessage {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl Vst3HostContextMessage {
    
    pub fn new_from_list(list: *mut dyn IAttributeList<AttrID = *const u8>) -> Self {
    
        todo!();
        /*
        : attribute_list(list),
        */
    }
    
    pub fn new_from_list_and_id(
        list: *mut dyn IAttributeList<AttrID = *const u8>,
        id:   FIDString) -> Self {
    
        todo!();
        /*
        : attribute_list(list),
        : message_id(toString (id)),

        
        */
    }
    
    pub fn new_from_list_id_and_var(
        list: *mut dyn IAttributeList<AttrID = *const u8>,
        id:   FIDString,
        v:    &Var

    ) -> Self {
    
        todo!();
        /*
        : value(v),
        : attribute_list(list),
        : message_id(toString (id)),

        
        */
    }
    
    pub fn get_messageid(&mut self) -> FIDString {
        
        todo!();
        /*
            return messageId.toRawUTF8();
        */
    }
    
    pub fn set_messageid(&mut self, id: FIDString)  {
        
        todo!();
        /*
            messageId = toString (id);
        */
    }
    
    pub fn get_attributes(&mut self) -> *mut dyn IAttributeList<AttrID = *const u8> {
        
        todo!();
        /*
            return attributeList;
        */
    }
}
