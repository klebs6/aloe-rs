crate::ix!();

/**
  | Implementation's example of IAttributeList.
  | \ingroup hostingBase
  |
  */
#[DECLARE_FUNKNOWN_METHODS]
pub struct HostAttributeList {
    list: HashMap<String,*mut HostAttribute>,
}

implement_funknown_methods!{
    HostAttributeList, 
    IAttributeList, 
    IAttributeList::iid
}

impl FUnknown for HostAttributeList {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl IAttributeList for HostAttributeList {

    type AttrID = *const u8;

    #[PLUGIN_API]
    fn set_int(&mut self, 
        aid:   <Self as IAttributeList>::AttrID,
        value: i64) -> tresult {
        
        todo!();
        /*
            remove<Self as IAttributeList>::AttrID (aid);
        list[aid] = new HostAttribute (value);
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn get_int(&mut self, 
        aid:   <Self as IAttributeList>::AttrID,
        value: &mut i64) -> tresult {
        
        todo!();
        /*
            auto it = list.find (aid);
        if (it != list.end () && it->second)
        {
            value = it->second->intValue ();
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn set_float(&mut self, 
        aid:   <Self as IAttributeList>::AttrID,
        value: f64) -> tresult {
        
        todo!();
        /*
            remove<Self as IAttributeList>::AttrID (aid);
        list[aid] = new HostAttribute (value);
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn get_float(&mut self, 
        aid:   <Self as IAttributeList>::AttrID,
        value: &mut f64) -> tresult {
        
        todo!();
        /*
            auto it = list.find (aid);
        if (it != list.end () && it->second)
        {
            value = it->second->floatValue ();
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn set_string(&mut self, 
        aid:    <Self as IAttributeList>::AttrID,
        string: *const TChar) -> tresult {
        
        todo!();
        /*
            remove<Self as IAttributeList>::AttrID (aid);
        // + 1 for the null-terminate
        list[aid] = new HostAttribute (string, String (string).length () + 1);
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn get_string(&mut self, 
        aid:           <Self as IAttributeList>::AttrID,
        string:        *mut TChar,
        size_in_bytes: u32) -> tresult {
        
        todo!();
        /*
            auto it = list.find (aid);
        if (it != list.end () && it->second)
        {
            uint32 sizeInCodeUnit = 0;
            const TChar* _string = it->second->stringValue (sizeInCodeUnit);
            memcpy (string, _string, std::min<uint32> (sizeInCodeUnit * sizeof (TChar), sizeInBytes));
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn set_binary(&mut self, 
        aid:           <Self as IAttributeList>::AttrID,
        data:          *const c_void,
        size_in_bytes: u32) -> tresult {
        
        todo!();
        /*
            remove<Self as IAttributeList>::AttrID (aid);
        list[aid] = new HostAttribute (data, sizeInBytes);
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn get_binary(&mut self, 
        aid:           <Self as IAttributeList>::AttrID,
        data:          &mut *const c_void,
        size_in_bytes: &mut u32) -> tresult {
        
        todo!();
        /*
            auto it = list.find (aid);
        if (it != list.end () && it->second)
        {
            data = it->second->binaryValue (sizeInBytes);
            return kResultTrue;
        }
        sizeInBytes = 0;
        return kResultFalse;
        */
    }
}

impl Drop for HostAttributeList {

    fn drop(&mut self) {
        todo!();
        /*
            auto it = list.rbegin ();
        while (it != list.rend ())
        {
            delete it->second;
            it++;
        }
        FUNKNOWN_DTOR
        */
    }
}

impl Default for HostAttributeList {

    fn default() -> Self {
    
        todo!();
        /*
            FUNKNOWN_CTOR
        */
    }
}

impl HostAttributeList {
    
    pub fn remove_attrid(&mut self, aid: <Self as IAttributeList>::AttrID)  {
        
        todo!();
        /*
            auto it = list.find (aid);
        if (it != list.end ())
        {
            delete it->second;
            list.erase (it);
        }
        */
    }
}
