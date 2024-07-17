/*!
  | Description : Vst 3 hostclasses, example
  | implementations for IHostApplication,
  | IAttributeList and IMessage
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/hosting/hostclasses.h]

/**
  | Implementation's example of IHostApplication.
  | \ingroup hostingBase
  |
  */
#[DECLARE_FUNKNOWN_METHODS]
pub struct HostApplication {
    plug_interface_support: IPtr<PlugInterfaceSupport>,
}

impl FUnknown for HostApplication {

    #[PLUGIN_API]
    fn query_interface(
        &mut self, 
        iid: [i8; 16],
        obj: *mut *mut c_void

    ) -> tresult {
        
        todo!();
        /*
            QUERY_INTERFACE (_iid, obj, FUnknown::iid, IHostApplication)
        QUERY_INTERFACE (_iid, obj, IHostApplication::iid, IHostApplication)

        if (mPlugInterfaceSupport && mPlugInterfaceSupport->queryInterface (iid, obj) == kResultTrue)
            return kResultOk;

        *obj = nullptr;
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn add_ref(&mut self) -> u32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    #[PLUGIN_API]
    fn release(&mut self) -> u32 {
        
        todo!();
        /*
            return 1;
        */
    }
}

impl IHostApplication for HostApplication {

    #[PLUGIN_API]
    fn get_name(&mut self, name: String128) -> tresult {
        
        todo!();
        /*
            String str ("My Vst3 HostApplication");
        str.copyTo16 (name, 0, 127);
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn create_instance(&mut self, 
        cid: TUID,
        iid: TUID,
        obj: *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            FUID classID (FUID::fromTUID (cid));
        FUID interfaceID (FUID::fromTUID (_iid));
        if (classID == IMessage::iid && interfaceID == IMessage::iid)
        {
            *obj = new HostMessage;
            return kResultTrue;
        }
        else if (classID == IAttributeList::iid && interfaceID == IAttributeList::iid)
        {
            *obj = new HostAttributeList;
            return kResultTrue;
        }
        *obj = nullptr;
        return kResultFalse;
        */
    }
    
}

impl Drop for HostApplication {

    fn drop(&mut self) {
        todo!();
        /*
            FUNKNOWN_DTOR
        */
    }
}

impl Default for HostApplication {

    fn default() -> Self {
    
        todo!();
        /*


            FUNKNOWN_CTOR

        mPlugInterfaceSupport = owned (NEW PlugInterfaceSupport);
        */
    }
}

impl HostApplication {
    
    //--- IHostApplication ---------------
    pub fn get_plug_interface_support(&self) -> *mut PlugInterfaceSupport {
        
        todo!();
        /*
            return mPlugInterfaceSupport;
        */
    }
}
