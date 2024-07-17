crate::ix!();

#[cfg(not(target_os="windows"))]
macro_rules! aloe_exported_function {
    () => {
        /*
                extern "C" __attribute__ ((visibility("default")))
        */
    }
}

pub fn plugin_entry_point(audio_master: Vst2AudioMasterCallback) -> *mut Vst2AEffect {
    
    todo!();
        /*
            ALOE_AUTORELEASEPOOL
        {
            ScopedAloeInitialiser_GUI libraryInitialiser;

           #if ALOE_LINUX || ALOE_BSD
            SharedResourcePointer<MessageThread> messageThread;
           #endif

            try
            {
                if (audioMaster (nullptr, Vst2::audioMasterVersion, 0, 0, nullptr, 0) != 0)
                {
                   #if ALOE_LINUX || ALOE_BSD
                    MessageManagerLock mmLock;
                   #endif

                    std::unique_ptr<AudioProcessor> processor { createPluginFilterOfType (AudioProcessor::wrapperType_VST) };
                    auto* processorPtr = processor.get();
                    auto* wrapper = new AloeVSTWrapper (audioMaster, std::move (processor));
                    auto* aEffect = wrapper->getAEffect();

                    if (auto* callbackHandler = dynamic_cast<VSTCallbackHandler*> (processorPtr))
                    {
                        callbackHandler->handleVstHostCallbackAvailable ([audioMaster, aEffect] (int32 opcode, int32 index, pointer_sized_int value, void* ptr, float opt)
                        {
                            return audioMaster (aEffect, opcode, index, value, ptr, opt);
                        });
                    }

                    return aEffect;
                }
            }
            catch (...)
            {}
        }

        return nullptr;
        */
}
