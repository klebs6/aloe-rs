crate::ix!();

#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[ALOE_DECLARE_Vst3_COM_QUERY_METHODS]
#[no_copy]
#[leak_detector]
pub struct Vst3HostContextAttributeList<'a> {
    owner:     *mut Vst3HostContext<'a>,
    ref_count: Atomic<i32>,
}

impl<'a> IAttributeList for Vst3HostContextAttributeList<'a> {

    fn set_int(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: i64) -> i32 { todo!() }

    fn get_int(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: &mut i64) -> i32 { todo!() }

    fn set_float(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: f64) -> i32 { todo!() }

    fn get_float(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: &mut f64) -> i32 { todo!() }

    fn set_string(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: *const u16) -> i32 { todo!() }

    fn get_string(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: *mut u16, _: u32) -> i32 { todo!() }

    fn set_binary(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: *const aloe_deps::c_void, _: u32) -> i32 { todo!() }

    fn get_binary(&mut self, _: <Self as aloe_vst_attributes::IAttributeList>::AttrID, _: &mut *const aloe_deps::c_void, _: &mut u32) -> i32 { todo!() }
}

impl<'a> FUnknown for Vst3HostContextAttributeList<'a> {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl<'a> Vst3HostContextAttributeList<'a> {

    pub fn new(o: *mut Vst3HostContext) -> Self {
    
        todo!();
        /*
        : owner(o),

        
        */
    }
    
    pub fn set_int(&mut self, 
        id:    <Self as IAttributeList>::AttrID,
        value: i64) -> tresult {
        
        todo!();
        /*
            addMessageToQueue (id, value);
                return kResultTrue;
        */
    }
    
    pub fn set_float(&mut self, 
        id:    <Self as IAttributeList>::AttrID,
        value: f64) -> tresult {
        
        todo!();
        /*
            addMessageToQueue (id, value);
                return kResultTrue;
        */
    }
    
    pub fn set_string(&mut self, 
        id:     <Self as IAttributeList>::AttrID,
        string: *const TChar) -> tresult {
        
        todo!();
        /*
            addMessageToQueue (id, toString (string));
                return kResultTrue;
        */
    }
    
    pub fn set_binary(&mut self, 
        id:   <Self as IAttributeList>::AttrID,
        data: *const c_void,
        size: u32) -> tresult {
        
        todo!();
        /*
            jassert (data != nullptr || size == 0);
                addMessageToQueue (id, MemoryBlock (data, (size_t) size));
                return kResultTrue;
        */
    }
    
    pub fn get_int(&mut self, 
        id:     <Self as IAttributeList>::AttrID,
        result: &mut i64) -> tresult {
        
        todo!();
        /*
            jassert (id != nullptr);

                if (findMessageOnQueueWithID (id, result))
                    return kResultTrue;

                jassertfalse;
                return kResultFalse;
        */
    }
    
    pub fn get_float(&mut self, 
        id:     <Self as IAttributeList>::AttrID,
        result: &mut f64) -> tresult {
        
        todo!();
        /*
            jassert (id != nullptr);

                if (findMessageOnQueueWithID (id, result))
                    return kResultTrue;

                jassertfalse;
                return kResultFalse;
        */
    }
    
    pub fn get_string(&mut self, 
        id:     <Self as IAttributeList>::AttrID,
        result: *mut TChar,
        length: u32) -> tresult {
        
        todo!();
        /*
            jassert (id != nullptr);

                String stringToFetch;
                if (findMessageOnQueueWithID (id, stringToFetch))
                {
                    String str (stringToFetch.toRawUTF8());
                    str.copyTo (result, 0, (i32) jmin (length, (uint32) std::numeric_limits<i32>::max()));

                    return kResultTrue;
                }

                jassertfalse;
                return kResultFalse;
        */
    }
    
    pub fn get_binary(&mut self, 
        id:   <Self as IAttributeList>::AttrID,
        data: &mut *const c_void,
        size: &mut u32) -> tresult {
        
        todo!();
        /*
            return owner->messageMap.getBinary (id, data, size);
        */
    }
    
    
    pub fn add_message_to_queue<Type>(&mut self, 
        id:    <Self as IAttributeList>::AttrID,
        value: &Type)  {
    
        todo!();
        /*
            owner->messageMap.addMessageToQueue (id, this, value);
        */
    }
    
    
    pub fn find_message_on_queue_withid<Type>(&mut self, 
        id:    <Self as IAttributeList>::AttrID,
        value: &mut Type) -> bool {
    
        todo!();
        /*
            return owner->messageMap.findMessageOnQueueWithID (id, value);
        */
    }
}
