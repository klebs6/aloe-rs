crate::ix!();

pub struct Vst3ComponentHolder<'a> {
    module:                   Vst3ModuleHandlePtr,
    factory:                  VstComSmartPtr<Box<dyn IPluginFactory>>,
    host:                     VstComSmartPtr<Vst3HostContext<'a>>,
    component:                VstComSmartPtr<Box<dyn VstIComponent>>,
    cid_of_component:         FUID,
    is_component_initialised: bool, // default = false
}

impl<'a> Drop for Vst3ComponentHolder<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            terminate();
        */
    }
}

impl<'a> From<&Vst3ModuleHandlePtr> for Vst3ComponentHolder<'a> {

    fn from(m: &Vst3ModuleHandlePtr) -> Self {
    
        todo!();
        /*
        : module(m),

            host = new Vst3HostContext();
        */
    }
}

impl<'a> Vst3ComponentHolder<'a> {

    pub fn fetch_controller(&mut self, edit_controller: &mut VstComSmartPtr<Box<dyn IEditController>>) -> bool {
        
        todo!();
        /*
            if (! isComponentInitialised && ! initialise())
                return false;

            // Get the IEditController:
            TUID controllerCID = { 0 };

            if (component->getControllerClassId (controllerCID) == kResultTrue && FUID (controllerCID).isValid())
                editController.loadFrom (factory, controllerCID);

            if (editController == nullptr)
            {
                // Try finding the IEditController the long way around:
                auto numClasses = factory->countClasses();

                for (i32 i = 0; i < numClasses; ++i)
                {
                    PClassInfo classInfo;
                    factory->getClassInfo (i, &classInfo);

                    if (std::strcmp (classInfo.category, kVstComponentControllerClass) == 0)
                        editController.loadFrom (factory, classInfo.cid);
                }
            }

            if (editController == nullptr)
                editController.loadFrom (component);

            return (editController != nullptr);
        */
    }
    
    pub fn fill_in_plugin_description(&self, description: &mut PluginDescription)  {
        
        todo!();
        /*
            jassert (module != nullptr && isComponentInitialised);

            PFactoryInfo factoryInfo;
            factory->getFactoryInfo (&factoryInfo);

            auto classIdx = getClassIndex (module->getName());

            if (classIdx >= 0)
            {
                PClassInfo info;
                bool success = (factory->getClassInfo (classIdx, &info) == kResultOk);
                ignoreUnused (success);
                jassert (success);

                VstComSmartPtr<IPluginFactory2> pf2;
                VstComSmartPtr<IPluginFactory3> pf3;

                std::unique_ptr<PClassInfo2> info2;
                std::unique_ptr<PClassInfoW> infoW;

                if (pf2.loadFrom (factory))
                {
                    info2.reset (new PClassInfo2());
                    pf2->getClassInfo2 (classIdx, info2.get());
                }
                else
                {
                    info2.reset();
                }

                if (pf3.loadFrom (factory))
                {
                    pf3->setHostContext (host->getFUnknown());
                    infoW.reset (new PClassInfoW());
                    pf3->getClassInfoUnicode (classIdx, infoW.get());
                }
                else
                {
                    infoW.reset();
                }

                VstBusInfo bus;
                int totalNumInputChannels = 0, totalNumOutputChannels = 0;

                int n = component->getBusCount(VstkAudio, VstkInput);
                for (int i = 0; i < n; ++i)
                    if (component->getBusInfo (VstkAudio, VstkInput, i, bus) == kResultOk)
                        totalNumInputChannels += ((bus.flags & VstBusInfo::kDefaultActive) != 0 ? bus.channelCount : 0);

                n = component->getBusCount(VstkAudio, VstkOutput);
                for (int i = 0; i < n; ++i)
                    if (component->getBusInfo (VstkAudio, VstkOutput, i, bus) == kResultOk)
                        totalNumOutputChannels += ((bus.flags & VstBusInfo::kDefaultActive) != 0 ? bus.channelCount : 0);

                createPluginDescription (description, module->getFile(),
                                         factoryInfo.vendor, module->getName(),
                                         info, info2.get(), infoW.get(),
                                         totalNumInputChannels,
                                         totalNumOutputChannels);

                return;
            }

            jassertfalse;
        */
    }
    
    pub fn initialise(&mut self) -> bool {
        
        todo!();
        /*
            if (isComponentInitialised)
                return true;

            // It's highly advisable to create your plugins using the message thread.
            // The Vst3 spec requires that many of the functions called during
            // initialisation are only called from the message thread.
            ALOE_ASSERT_MESSAGE_THREAD

            factory = VstComSmartPtr<IPluginFactory> (module->getPluginFactory());

            int classIdx;
            if ((classIdx = getClassIndex (module->getName())) < 0)
                return false;

            PClassInfo info;
            if (factory->getClassInfo (classIdx, &info) != kResultOk)
                return false;

            if (! component.loadFrom (factory, info.cid) || component == nullptr)
                return false;

            cidOfComponent = FUID (info.cid);

            if (warnOnFailure (component->initialize (host->getFUnknown())) != kResultOk)
                return false;

            isComponentInitialised = true;

            return true;
        */
    }
    
    pub fn terminate(&mut self)  {
        
        todo!();
        /*
            if (isComponentInitialised)
            {
                component->terminate();
                isComponentInitialised = false;
            }

            component = nullptr;
        */
    }
    
    pub fn get_class_index(&self, class_name: &String) -> i32 {
        
        todo!();
        /*
            PClassInfo info;
            const i32 numClasses = factory->countClasses();

            for (i32 j = 0; j < numClasses; ++j)
                if (factory->getClassInfo (j, &info) == kResultOk
                     && std::strcmp (info.category, kVstAudioEffectClass) == 0
                     && toString (info.name).trim() == className)
                    return j;

            return -1;
        */
    }

    /**
      | transfers ownership to the plugin instance!
      |
      */
    pub fn create_plugin_instance(&mut self) -> *mut AudioPluginInstance {
        
        todo!();
        /*
            if (! initialise())
            return nullptr;

        auto* plugin = new Vst3PluginInstance (this);
        host->setPlugin (plugin);
        return plugin;
        */
    }
}
