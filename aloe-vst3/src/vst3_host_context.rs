crate::ix!();

#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[no_copy]
#[leak_detector]
pub struct Vst3HostContext<'a> {


    plugin:              *mut Vst3PluginInstance<'a>, // default = nullptr
    ref_count:           Atomic<i32>,
    app_name:            String,
    component_restarter: ComponentRestarter<'a>, //{ *this };
    message_map:         Vst3HostContextMessageMap,
    attribute_list:      VstComSmartPtr<Vst3HostContextAttributeList<'a>>,
}

impl<'a> IUnitHandler for Vst3HostContext<'a> {

    fn notify_unit_selection(&mut self, _0: UnitID) -> tresult {
        
        todo!();
        /*
            jassertfalse;
            return kResultFalse;
        */
    }
    
    fn notify_program_list_change(
        &mut self, 
        _0: ProgramListID,
        _1: i32

    ) -> tresult {
        
        todo!();
        /*
            if (plugin != nullptr)
            plugin->syncProgramNames();

        return kResultTrue;
        */
    }
}

impl<'a> FUnknown for Vst3HostContext<'a> {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl<'a> IHostApplication for Vst3HostContext<'a> {

    fn get_name(&mut self, _: [u16; 128]) -> i32 { todo!() }

    fn create_instance(&mut self, _: [i8; 16], _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }
}

impl<'a> IContextMenuTarget for Vst3HostContext<'a> {

    fn execute_menu_item(&mut self, _: i32) -> i32 { todo!() }
}

/**
  | From Vst V3.0.0
  |
  */
impl<'a> IComponentHandler for Vst3HostContext<'a> {

    fn begin_edit(&mut self, _: u32) -> i32 { todo!() }

    fn perform_edit(&mut self, _: u32, _: f64) -> i32 { todo!() }

    fn end_edit(&mut self, _: u32) -> i32 { todo!() }

    fn restart_component(&mut self, _: i32) -> i32 { todo!() }
}

/**
  | From Vst V3.1.0 (a very well named class,
  | of course!)
  |
  */
impl<'a> IComponentHandler2 for Vst3HostContext<'a> {

    fn set_dirty(&mut self, _: u8) -> i32 { todo!() }

    fn request_open_editor(&mut self, _: &'static str) -> i32 { todo!() }

    fn start_group_edit(&mut self) -> i32 { todo!() }

    fn finish_group_edit(&mut self) -> i32 { todo!() }
}

/**
  | From Vst V3.5.0 (also very well named!)
  |
  */
impl<'a> IComponentHandler3 for Vst3HostContext<'a> {

    fn create_context_menu(&mut self, _: *mut (dyn aloe_vst_plugview::IPlugView + 'static), _: *const u32) -> *mut (dyn aloe_vst_edit::IContextMenu<Item = aloe_vst_edit::IContextMenuItem> + 'static) { todo!() }
}

impl<'a> ComponentRestarterListener for Vst3HostContext<'a> {

    fn restart_component_on_message_thread(&mut self, flags: i32) {
        todo!();
    }
}

impl<'a> Default for Vst3HostContext<'a> {
    
    fn default() -> Self {
        todo!();
        /*

            appName = File::getSpecialLocation (File::currentApplicationFile).getFileNameWithoutExtension();
            attributeList = new Vst3HostContextAttributeList (this);
        */
    }
}

impl<'a> Vst3HostContext<'a> {
    
    pub fn get_funknown(&mut self) -> *mut dyn FUnknown {
        
        todo!();
        /*
            return static_cast<VstIComponentHandler*> (this);
        */
    }
    
    pub fn has_flag(
        source: i32,
        flag:   i32) -> bool {
        
        todo!();
        /*
            return (source & flag) == flag;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_dirty(&mut self, _0: TBool) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn request_open_editor(&mut self, name: FIDString) -> tresult {
        
        todo!();
        /*
            ignoreUnused (name);
            jassertfalse;
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn start_group_edit(&mut self) -> tresult {
        
        todo!();
        /*
            jassertfalse;
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn finish_group_edit(&mut self) -> tresult {
        
        todo!();
        /*
            jassertfalse;
            return kResultFalse;
        */
    }
    
    pub fn set_plugin(&mut self, instance: *mut Vst3PluginInstance)  {
        
        todo!();
        /*
            jassert (plugin == nullptr);
            plugin = instance;
        */
    }
    
    pub fn create_context_menu(
        &mut self, 
        _0: *mut dyn IPlugView,
        _1: *const ParamID

    ) -> *mut dyn IContextMenu<Item = IContextMenuItem> {
        
        todo!();
        /*
            if (plugin != nullptr)
                return new Vst3HostContextContextMenu (*plugin);

            return nullptr;
        */
    }
    
    pub fn execute_menu_item(&mut self, _0: i32) -> tresult {
        
        todo!();
        /*
            jassertfalse;
            return kResultFalse;
        */
    }
    
    pub fn get_name(&mut self, name: String128) -> tresult {
        
        todo!();
        /*
            String str (appName.toUTF8());
            str.copyTo (name, 0, 127);
            return kResultOk;
        */
    }
    
    pub fn create_instance(&mut self, 
        cid: TUID,
        iid: TUID,
        obj: *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            *obj = nullptr;

            if (! doUIDsMatch (cid, iid))
            {
                jassertfalse;
                return kInvalidArgument;
            }

            if (doUIDsMatch (cid, VstIMessage::iid) && doUIDsMatch (iid, VstIMessage::iid))
            {
                VstComSmartPtr<Vst3HostContextMessage> m (new Vst3HostContextMessage (attributeList));
                messageMap.add (m);
                m->addRef();
                *obj = m;
                return kResultOk;
            }

            if (doUIDsMatch (cid, VstIAttributeList::iid) && doUIDsMatch (iid, VstIAttributeList::iid))
            {
                VstComSmartPtr<Vst3HostContextAttributeList> l (new Vst3HostContextAttributeList (this));
                l->addRef();
                *obj = l;
                return kResultOk;
            }

            jassertfalse;
            return kNotImplemented;
        */
    }
    
    pub fn query_interface(&mut self, 
        iid: TUID,
        obj: *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            if (doUIDsMatch (iid, VstIAttributeList::iid))
            {
                *obj = attributeList.get();
                return kResultOk;
            }

            return testForMultiple (*this,
                                    iid,
                                    UniqueBase<VstIComponentHandler>{},
                                    UniqueBase<VstIComponentHandler2>{},
                                    UniqueBase<VstIComponentHandler3>{},
                                    UniqueBase<VstIContextMenuTarget>{},
                                    UniqueBase<VstIHostApplication>{},
                                    UniqueBase<VstIUnitHandler>{},
                                    SharedBase<FUnknown, VstIComponentHandler>{}).extract (obj);
        */
    }
    
    #[PLUGIN_API]
    pub fn begin_edit(&mut self, paramid: ParamID) -> tresult {
        
        todo!();
        /*
            if (plugin == nullptr)
            return kResultTrue;

        if (auto* param = plugin->getParameterForID (paramID))
        {
            param->beginChangeGesture();
            return kResultTrue;
        }

        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn perform_edit(
        &mut self, 
        paramid:          ParamID,
        value_normalised: ParamValue

    ) -> tresult {
        
        todo!();
        /*
            if (plugin == nullptr)
            return kResultTrue;

        if (auto* param = plugin->getParameterForID (paramID))
        {
            param->setValueFromEditor ((float) valueNormalised);

            // did the plug-in already update the parameter internally
            if (plugin->editController->getParamNormalized (paramID) != (float) valueNormalised)
                return plugin->editController->setParamNormalized (paramID, valueNormalised);

            return kResultTrue;
        }

        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn end_edit(&mut self, paramid: ParamID) -> tresult {
        
        todo!();
        /*
            if (plugin == nullptr)
            return kResultTrue;

        if (auto* param = plugin->getParameterForID (paramID))
        {
            param->endChangeGesture();
            return kResultTrue;
        }

        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn restart_component(&mut self, flags: i32) -> tresult {
        
        todo!();
        /*
            // If you hit this, the plugin has requested a restart from a thread other than
        // the UI thread. Aloe should be able to cope, but you should consider filing a bug
        // report against the plugin.
        ALOE_ASSERT_MESSAGE_THREAD

        componentRestarter.restart (flags);
        return kResultTrue;
        */
    }
    
    pub fn restart_component_on_message_thread(&mut self, flags: i32)  {
        
        todo!();
        /*
            if (plugin == nullptr)
        {
            jassertfalse;
            return;
        }

        if (hasFlag (flags, VstkReloadComponent))
            plugin->reset();

        if (hasFlag (flags, VstkIoChanged))
        {
            auto sampleRate = plugin->getSampleRate();
            auto blockSize  = plugin->getBlockSize();

            // Have to deactivate here, otherwise prepareToPlay might not pick up the new bus layouts
            plugin->releaseResources();
            plugin->prepareToPlay (sampleRate >= 8000 ? sampleRate : 44100.0,
            blockSize > 0 ? blockSize : 1024);
        }

        if (hasFlag (flags, VstkLatencyChanged))
            if (plugin->processor != nullptr)
                plugin->setLatencySamples (jmax (0, (int) plugin->processor->getLatencySamples()));

        if (hasFlag (flags, VstkMidiCCAssignmentChanged))
            plugin->updateMidiMappings();

        if (hasFlag (flags, VstkParamValuesChanged))
            plugin->resetParameters();

        plugin->updateHostDisplay (AudioProcessorListener::ChangeDetails().withProgramChanged (true)
                                                                          .withParameterInfoChanged (true));
        */
    }
    
}
