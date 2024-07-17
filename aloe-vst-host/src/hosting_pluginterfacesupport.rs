/*!
  | Description : Vst 3 hostclasses, example
  | implementations for IPlugInterfaceSupport
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/hosting/pluginterfacesupport.h]

/**
  | Implementation's example of IPlugInterfaceSupport.
  | \ingroup hostingBase
  |
  */
pub struct PlugInterfaceSupport {
    base:       FObject,
    fuid_array: Vec<FUID>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/hosting/pluginterfacesupport.cpp]
impl IPlugInterfaceSupport for PlugInterfaceSupport {

    #[PLUGIN_API]
    fn is_plug_interface_supported(&mut self, iid: TUID) -> tresult {
        
        todo!();
        /*
            auto uid = FUID::fromTUID (_iid);
        if (std::find (mFUIDArray.begin (), mFUIDArray.end (), uid) != mFUIDArray.end ())
            return kResultTrue;
        return kResultFalse;
        */
    }
}

impl FUnknown for PlugInterfaceSupport {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl PlugInterfaceSupport {

    //--- IPlugInterfaceSupport ---------

    lazy_static!{
        /*
        OBJ_METHODS (PlugInterfaceSupport, FObject)
            REFCOUNT_METHODS (FObject)
            DEFINE_INTERFACES
                DEF_INTERFACE (IPlugInterfaceSupport)
            END_DEFINE_INTERFACES (FObject)
        */
    }
   
    pub fn add_plug_interface_supported(&mut self, iid: TUID)  {
        
        todo!();
        /*
            mFUIDArray.push_back (FUID::fromTUID (_iid));
        */
    }
    
    pub fn remove_plug_interface_supported(&mut self, iid: TUID) -> bool {
        
        todo!();
        /*
            return std::remove (mFUIDArray.begin (), mFUIDArray.end (), FUID::fromTUID (_iid)) !=
               mFUIDArray.end ();
        */
    }
}

impl Default for PlugInterfaceSupport {

    fn default() -> Self {
    
        todo!();
        /*
        // add minimum set

        //---Vst 3.0.0--------------------------------
        addPlugInterfaceSupported (IComponent::iid);
        addPlugInterfaceSupported (IAudioProcessor::iid);
        addPlugInterfaceSupported (IEditController::iid);
        addPlugInterfaceSupported (IConnectionPoint::iid);

        addPlugInterfaceSupported (IUnitInfo::iid);
        addPlugInterfaceSupported (IUnitData::iid);
        addPlugInterfaceSupported (IProgramListData::iid);

        //---Vst 3.0.1--------------------------------
        addPlugInterfaceSupported (IMidiMapping::iid);

        //---Vst 3.1----------------------------------
        addPlugInterfaceSupported (IEditController2::iid);

        /*
        //---Vst 3.0.2--------------------------------
        addPlugInterfaceSupported (IParameterFinder::iid);

        //---Vst 3.1----------------------------------
        addPlugInterfaceSupported (IAudioPresentationLatency::iid);

        //---Vst 3.5----------------------------------
        addPlugInterfaceSupported (IKeyswitchController::iid);
        addPlugInterfaceSupported (IContextMenuTarget::iid);
        addPlugInterfaceSupported (IEditControllerHostEditing::iid);
        addPlugInterfaceSupported (IXmlRepresentationController::iid);
        addPlugInterfaceSupported (INoteExpressionController::iid);

        //---Vst 3.6.5--------------------------------
        addPlugInterfaceSupported (ChannelContext::IInfoListener::iid);
        addPlugInterfaceSupported (IPrefetchableSupport::iid);
        addPlugInterfaceSupported (IAutomationState::iid);

        //---Vst 3.6.11--------------------------------
        addPlugInterfaceSupported (INoteExpressionPhysicalUIMapping::iid);

        //---Vst 3.6.12--------------------------------
        addPlugInterfaceSupported (IMidiLearn::iid);

        //---Vst 3.7-----------------------------------
        addPlugInterfaceSupported (IProcessContextRequirements::iid);
        addPlugInterfaceSupported (IParameterFunctionName::iid);
        addPlugInterfaceSupported (IProgress::iid);
        */
        */
    }
}
