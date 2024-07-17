crate::ix!();

/**
  | no leak detector here to prevent it firing
  | on shutdown when running in hosts that
  | don't release the factory object
  | correctly...
  */
#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[no_copy]
pub struct AloePluginFactory {
    ref_count:    Atomic<i32>, // default = { 1  }
    factory_info: PFactoryInfo,
    host:         VstComSmartPtr<VstIHostApplication>,
    classes:      Vec<Box<AloePluginFactoryClassEntry>>,
}

impl FUnknown for AloePluginFactory {
    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }
    fn add_ref(&mut self) -> u32 { todo!() }
    fn release(&mut self) -> u32 { todo!() }

}

impl IPluginFactory for AloePluginFactory {

    fn get_factory_info(&mut self, _: *mut aloe_vst_plugin::PFactoryInfo) -> i32 { todo!() }
    fn count_classes(&mut self) -> i32 { todo!() }
    fn get_class_info(&mut self, _: i32, _: *mut aloe_vst_plugin::PClassInfo) -> i32 { todo!() }
    fn create_instance(&mut self, _: &'static str, _: &'static str, _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }
}

impl IPluginFactory2 for AloePluginFactory {
    fn get_class_info2(&mut self, _: i32, _: *mut aloe_vst_plugin::PClassInfo2) -> i32 { todo!() }
}

impl IPluginFactory3 for AloePluginFactory {

    fn get_class_info_unicode(&mut self, _: i32, _: *mut aloe_vst_plugin::PClassInfoW) -> i32 { todo!() }

    fn set_host_context(&mut self, _: *mut (dyn aloe_vst_types::FUnknown + 'static)) -> i32 { todo!() }
}

impl Default for AloePluginFactory {
    
    fn default() -> Self {
        todo!();
        /*


            : factoryInfo (AloePlugin_Manufacturer, AloePlugin_ManufacturerWebsite,
                           AloePlugin_ManufacturerEmail, VstkDefaultFactoryFlags)
        */
    }
}

impl Drop for AloePluginFactory {

    fn drop(&mut self) {
        todo!();
        /*
            if (globalFactory == this)
                globalFactory = nullptr;
        */
    }
}

impl AloePluginFactory {

    pub fn register_class(&mut self, 
        info:            &PClassInfo2,
        create_function: CreateFunction) -> bool {
        
        todo!();
        /*
            if (createFunction == nullptr)
            {
                jassertfalse;
                return false;
            }

            auto entry = std::make_unique<AloePluginFactoryClassEntry> (info, createFunction);
            entry->infoW.fromAscii (info);

            classes.push_back (std::move (entry));

            return true;
        */
    }
    
    #[PLUGIN_API]
    pub fn query_interface(&mut self, 
        targetiid: TUID,
        obj:       *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            const auto result = testForMultiple (*this,
                                                 targetIID,
                                                 UniqueBase<IPluginFactory3>{},
                                                 UniqueBase<IPluginFactory2>{},
                                                 UniqueBase<IPluginFactory>{},
                                                 UniqueBase<FUnknown>{});

            if (result.isOk())
                return result.extract (obj);

            jassertfalse; // Something new?
            *obj = nullptr;
            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn count_classes(&mut self) -> i32 {
        
        todo!();
        /*
            return (i32) classes.size();
        */
    }
    
    #[PLUGIN_API]
    pub fn get_factory_info(&mut self, info: *mut PFactoryInfo) -> tresult {
        
        todo!();
        /*
            if (info == nullptr)
                return kInvalidArgument;

            memcpy (info, &factoryInfo, sizeof (PFactoryInfo));
            return kResultOk;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_class_info(&mut self, 
        index: i32,
        info:  *mut PClassInfo) -> tresult {
        
        todo!();
        /*
            return getPClassInfo<PClassInfo> (index, info);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_class_info2(&mut self, 
        index: i32,
        info:  *mut PClassInfo2) -> tresult {
        
        todo!();
        /*
            return getPClassInfo<PClassInfo2> (index, info);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_class_info_unicode(&mut self, 
        index: i32,
        info:  *mut PClassInfoW) -> tresult {
        
        todo!();
        /*
            if (info != nullptr)
            {
                if (auto& entry = classes[(size_t) index])
                {
                    memcpy (info, &entry->infoW, sizeof (PClassInfoW));
                    return kResultOk;
                }
            }

            return kInvalidArgument;
        */
    }
    
    #[PLUGIN_API]
    pub fn create_instance(&mut self, 
        cid:        FIDString,
        source_iid: FIDString,
        obj:        *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            ScopedAloeInitialiser_GUI libraryInitialiser;

           #if ALOE_LINUX || ALOE_BSD
            SharedResourcePointer<MessageThread> messageThread;
           #endif

            *obj = nullptr;

            TUID tuid;
            memcpy (tuid, sourceIid, sizeof (TUID));

           #if Vst_VERSION >= 0x030608
            auto sourceFuid = FUID::fromTUID (tuid);
           #else
            FUID sourceFuid;
            sourceFuid = tuid;
           #endif

            if (cid == nullptr || sourceIid == nullptr || ! sourceFuid.isValid())
            {
                jassertfalse; // The host you're running in has severe implementation issues!
                return kInvalidArgument;
            }

            TUID iidToQuery;
            sourceFuid.toTUID (iidToQuery);

            for (auto& entry : classes)
            {
                if (doUIDsMatch (entry->infoW.cid, cid))
                {
                    if (auto* instance = entry->createFunction (host))
                    {
                        const FReleaser releaser (instance);

                        if (instance->queryInterface (iidToQuery, obj) == kResultOk)
                            return kResultOk;
                    }

                    break;
                }
            }

            return kNoInterface;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_host_context(&mut self, context: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            host.loadFrom (context);

            if (host != nullptr)
            {
                VstString128 name;
                host->getName (name);

                return kResultTrue;
            }

            return kNotImplemented;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_pclass_info<PClassInfoType>(&mut self, 
        index: i32,
        info:  *mut PClassInfoType) -> tresult {
    
        todo!();
        /*
            if (info != nullptr)
            {
                zerostruct (*info);

                if (auto& entry = classes[(size_t) index])
                {
                    if (entry->isUnicode)
                        return kResultFalse;

                    memcpy (info, (PClassInfoType*) &entry->info2, sizeof (PClassInfoType));
                    return kResultOk;
                }
            }

            jassertfalse;
            return kInvalidArgument;
        */
    }
}
