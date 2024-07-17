crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DescriptionFactory<'a> {
    vst_3host_context: VstComSmartPtr<Vst3HostContext<'a>>,
    factory:           VstComSmartPtr<Box<dyn IPluginFactory>>,
}

pub trait DescriptionFactoryInterface: PerformOnDescription {}

impl<'a> DescriptionFactory<'a> {

    pub fn new(
        host:           *mut Vst3HostContext,
        plugin_factory: *mut dyn IPluginFactory) -> Self {
    
        todo!();
        /*
        : vst_3host_context(host),
        : factory(pluginFactory),

            jassert (pluginFactory != nullptr);
        */
    }
    
    pub fn find_descriptions_and_perform(&mut self, file: &File) -> Result<(),()> {
        
        todo!();
        /*
            StringArray foundNames;
            PFactoryInfo factoryInfo;
            factory->getFactoryInfo (&factoryInfo);
            auto companyName = toString (factoryInfo.vendor).trim();

            Result result (Result::ok());

            auto numClasses = factory->countClasses();

            for (i32 i = 0; i < numClasses; ++i)
            {
                PClassInfo info;
                factory->getClassInfo (i, &info);

                if (std::strcmp (info.category, kVstAudioEffectClass) != 0)
                    continue;

                const String name (toString (info.name).trim());

                if (foundNames.contains (name, true))
                    continue;

                std::unique_ptr<PClassInfo2> info2;
                std::unique_ptr<PClassInfoW> infoW;

                {
                    VstComSmartPtr<IPluginFactory2> pf2;
                    VstComSmartPtr<IPluginFactory3> pf3;

                    if (pf2.loadFrom (factory))
                    {
                        info2.reset (new PClassInfo2());
                        pf2->getClassInfo2 (i, info2.get());
                    }

                    if (pf3.loadFrom (factory))
                    {
                        infoW.reset (new PClassInfoW());
                        pf3->getClassInfoUnicode (i, infoW.get());
                    }
                }

                foundNames.add (name);

                PluginDescription desc;

                {
                    VstComSmartPtr<VstIComponent> component;

                    if (component.loadFrom (factory, info.cid))
                    {
                        if (component->initialize (vst3HostContext->getFUnknown()) == kResultOk)
                        {
                            auto numInputs  = getNumSingleDirectionChannelsFor (component, true, true);
                            auto numOutputs = getNumSingleDirectionChannelsFor (component, false, true);

                            createPluginDescription (desc, file, companyName, name,
                                                     info, info2.get(), infoW.get(), numInputs, numOutputs);

                            component->terminate();
                        }
                        else
                        {
                            jassertfalse;
                        }
                    }
                    else
                    {
                        jassertfalse;
                    }
                }

                if (desc.uniqueId != 0)
                    result = performOnDescription (desc);

                if (result.failed())
                    break;
            }

            return result;
        */
    }
}
