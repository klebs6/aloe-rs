crate::ix!();

#[no_copy]
#[leak_detector]
#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
pub struct AloeVst3Component<'a> {
    base2:               VstIAudioProcessor,
    base3:               VstIUnitInfo,
    base4:               VstIConnectionPoint,
    base5:               VstIProcessContextRequirements,

    library_initialiser: ScopedAloeInitialiser_GUI,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    message_thread:      SharedResourcePointer<MessageThread>,

    ref_count:           Atomic<i32>, // default = { 1  }
    plugin_instance:     *mut dyn AudioProcessorInterface, // default = nullptr

    ///---------------------------
    #[cfg(any(target_os="linux",target_os="bsd"))]
    host:                     AloeVst3ComponentLockedVstComSmartPtr<VstIHostApplication>,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    com_plugin_instance:      AloeVst3ComponentLockedVstComSmartPtr<AloeAudioProcessor>,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    aloe_vst3edit_controller: AloeVst3ComponentLockedVstComSmartPtr<AloeVst3EditController>,

    ///---------------------------
    #[cfg(not(any(target_os="linux",target_os="bsd")))]
    host:                     VstComSmartPtr<VstIHostApplication>,

    #[cfg(not(any(target_os="linux",target_os="bsd")))]
    com_plugin_instance:      VstComSmartPtr<AloeAudioProcessor>,

    #[cfg(not(any(target_os="linux",target_os="bsd")))]
    aloe_vst3edit_controller: VstComSmartPtr<AloeVst3EditController<'a>>,

    /**
      | Since Vst3 does not provide a way of
      | knowing the buffer size and sample rate at
      | any point, this object needs to be copied
      | on every call to process() to be up-to-date...
      */
    process_context:     VstProcessContext,
    process_setup:       VstProcessSetup,
    midi_buffer:         MidiBuffer,
    channel_list_float:  Vec<*mut f32>,
    channel_list_double: Vec<*mut f64>,
    empty_buffer_float:  AudioBuffer<f32>,
    empty_buffer_double: AudioBuffer<f64>,

    #[cfg(AloePlugin_WantsMidiInput)]
    is_midi_input_bus_enabled: AtomicBool, // default = { true  }

    #[cfg(AloePlugin_ProducesMidiOutput)]
    is_midi_output_bus_enabled: AtomicBool, // default = { true  }
}

impl<'a> AudioPlayHeadInterface for AloeVst3Component<'a> {
    fn get_current_position(&mut self, _: &mut aloe_playhead::AudioPlayHeadCurrentPositionInfo) -> bool { todo!() }
}

impl<'a> VstIComponent for AloeVst3Component<'a> {

    fn get_controller_class_id(&mut self, _: [i8; 16]) -> i32 { todo!() }
    fn set_io_mode(&mut self, _: i32) -> i32 { todo!() }
    fn get_bus_count(&mut self, _: i32, _: i32) -> i32 { todo!() }
    fn get_bus_info(&mut self, _: i32, _: i32, _: i32, _: &mut aloe_vst_events::VstBusInfo) -> i32 { todo!() }
    fn get_routing_info(&mut self, _: &mut aloe_vst_component::VstRoutingInfo, _: &mut aloe_vst_component::VstRoutingInfo) -> i32 { todo!() }
    fn activate_bus(&mut self, _: i32, _: i32, _: i32, _: u8) -> i32 { todo!() }
    fn set_active(&mut self, _: u8) -> i32 { todo!() }
    fn set_state(&mut self, _: *mut (dyn aloe_vst_stream::IBStream + 'static)) -> i32 { todo!() }
    fn get_state(&mut self, _: *mut (dyn aloe_vst_stream::IBStream + 'static)) -> i32 { todo!() }
}

impl<'a> FUnknown for AloeVst3Component<'a> {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl<'a> IPluginBase for AloeVst3Component<'a> {

    fn initialize(&mut self, _: *mut (dyn aloe_vst_types::FUnknown + 'static)) -> i32 { todo!() }

    fn terminate(&mut self) -> i32 { todo!() }
}

lazy_static!{
    /*
       static const FUID aloe_vst3_component_iid;
       static const char* aloe_vst3_component_kAloePrivateDataIdentifier;
       const char* AloeVst3Component::kAloePrivateDataIdentifier = "ALOEPrivateData";
    */
}

impl<'a> Drop for AloeVst3Component<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (aloeVst3EditController != nullptr)
                aloeVst3EditController->vst3IsPlaying = false;

            if (pluginInstance != nullptr)
                if (pluginInstance->getPlayHead() == this)
                    pluginInstance->setPlayHead (nullptr);
        */
    }
}

impl<'a> AloeVst3Component<'a> {

    pub fn new(h: *mut VstIHostApplication) -> Self {
    
        todo!();
        /*


            : pluginInstance (createPluginFilterOfType (AudioProcessor::wrapperType_Vst3)),
            host (h)

            inParameterChangedCallback = false;

           #ifdef AloePlugin_PreferredChannelConfigurations
            short configs[][2] = { AloePlugin_PreferredChannelConfigurations };
            const int numConfigs = sizeof (configs) / sizeof (short[2]);

            ignoreUnused (numConfigs);
            jassert (numConfigs > 0 && (configs[0][0] > 0 || configs[0][1] > 0));

            pluginInstance->setPlayConfigDetails (configs[0][0], configs[0][1], 44100.0, 1024);
           #endif

            // Vst-3 requires your default layout to be non-discrete!
            // For example, your default layout must be mono, stereo, quadrophonic
            // and not AudioChannelSet::discreteChannels (2) etc.
            jassert (checkBusFormatsAreNotDiscrete());

            comPluginInstance = VstComSmartPtr<AloeAudioProcessor> { new AloeAudioProcessor (pluginInstance) };

            zerostruct (processContext);

            processSetup.maxSamplesPerBlock = 1024;
            processSetup.processMode = VstkRealtime;
            processSetup.sampleRate = 44100.0;
            processSetup.symbolicSampleSize = VstkSample32;

            pluginInstance->setPlayHead (this);

            // Constructing the underlying static object involves dynamic allocation.
            // This call ensures that the construction won't happen on the audio thread.
            getHostType();
        */
    }
    
    pub fn get_plugin_instance(&self) -> &mut dyn AudioProcessorInterface {
        
        todo!();
        /*
            return *pluginInstance;
        */
    }

    #[PLUGIN_API]
    pub fn query_interface(&mut self, 
        targetiid: TUID,
        obj:       *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            const auto userProvidedInterface = queryAdditionalInterfaces (&getPluginInstance(),
                                                                          targetIID,
                                                                          &Vst3ClientExtensions::queryIAudioProcessor);

            const auto aloeProvidedInterface = queryInterfaceInternal (targetIID);

            return extractResult (userProvidedInterface, aloeProvidedInterface, obj);
        */
    }
    
    #[PLUGIN_API]
    pub fn initialize(&mut self, host_context: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            if (host != hostContext)
                host.loadFrom (hostContext);

            processContext.sampleRate = processSetup.sampleRate;
            preparePlugin (processSetup.sampleRate, (int) processSetup.maxSamplesPerBlock);

            return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn terminate(&mut self) -> tresult {
        
        todo!();
        /*
            getPluginInstance().releaseResources();
            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn connect(&mut self, other: *mut dyn IConnectionPoint) -> tresult {
        
        todo!();
        /*
            if (other != nullptr && aloeVst3EditController == nullptr)
                aloeVst3EditController.loadFrom (other);

            return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn disconnect(&mut self, _0: *mut dyn IConnectionPoint) -> tresult {
        
        todo!();
        /*
            if (aloeVst3EditController != nullptr)
                aloeVst3EditController->vst3IsPlaying = false;

            aloeVst3EditController = {};
            return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn notify(&mut self, message: *mut VstIMessage) -> tresult {
        
        todo!();
        /*
            if (message != nullptr && aloeVst3EditController == nullptr)
            {
                int64 value = 0;

                if (message->getAttributes()->getInt ("AloeVst3EditController", value) == kResultTrue)
                {
                    aloeVst3EditController = VstComSmartPtr<AloeVst3EditController> { (AloeVst3EditController*) (pointer_sized_int) value };

                    if (aloeVst3EditController != nullptr)
                        aloeVst3EditController->setAudioProcessor (comPluginInstance);
                    else
                        jassertfalse;
                }
            }

            return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn get_controller_class_id(&mut self, classid: TUID) -> tresult {
        
        todo!();
        /*
            memcpy (classID, AloeVst3EditController::iid, sizeof (TUID));
            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_active(&mut self, state: TBool) -> tresult {
        
        todo!();
        /*
            if (! state)
            {
                getPluginInstance().releaseResources();

                deallocateChannelListAndBuffers (channelListFloat,  emptyBufferFloat);
                deallocateChannelListAndBuffers (channelListDouble, emptyBufferDouble);
            }
            else
            {
                auto sampleRate = getPluginInstance().getSampleRate();
                auto bufferSize = getPluginInstance().getBlockSize();

                sampleRate = processSetup.sampleRate > 0.0
                                ? processSetup.sampleRate
                                : sampleRate;

                bufferSize = processSetup.maxSamplesPerBlock > 0
                                ? (int) processSetup.maxSamplesPerBlock
                                : bufferSize;

                allocateChannelListAndBuffers (channelListFloat,  emptyBufferFloat);
                allocateChannelListAndBuffers (channelListDouble, emptyBufferDouble);

                preparePlugin (sampleRate, bufferSize);
            }

            return kResultOk;
        */
    }

    #[PLUGIN_API]
    pub fn set_io_mode(&mut self, _0: VstIoMode) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }

    #[PLUGIN_API]
    pub fn get_routing_info(&mut self, 
        _0: &mut VstRoutingInfo,
        _1: &mut VstRoutingInfo) -> tresult {
        
        todo!();
        /*
            return kNotImplemented;
        */
    }
    
    pub fn is_bypassed(&mut self) -> bool {
        
        todo!();
        /*
            if (auto* bypassParam = comPluginInstance->getBypassParameter())
                return (bypassParam->getValue() != 0.0f);

            return false;
        */
    }
    
    pub fn set_bypassed(&mut self, should_be_bypassed: bool)  {
        
        todo!();
        /*
            if (auto* bypassParam = comPluginInstance->getBypassParameter())
            {
                auto floatValue = (shouldBeBypassed ? 1.0f : 0.0f);
                bypassParam->setValue (floatValue);

                const InParameterChangedCallbackSetter scopedSetter { inParameterChangedCallback };
                bypassParam->sendValueChangedMessageToListeners (floatValue);
            }
        */
    }
    
    pub fn write_aloe_private_state_information(&mut self, out: &mut MemoryOutputStream)  {
        
        todo!();
        /*
            if (pluginInstance->getBypassParameter() == nullptr)
            {
                ValueTree privateData (kAloePrivateDataIdentifier);

                // for now we only store the bypass value
                privateData.setProperty ("Bypass", var (isBypassed()), nullptr);
                privateData.writeToStream (out);
            }
        */
    }
    
    pub fn set_aloe_private_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            if (pluginInstance->getBypassParameter() == nullptr)
            {
                if (comPluginInstance->getBypassParameter() != nullptr)
                {
                    auto privateData = ValueTree::readFromData (data, static_cast<size_t> (sizeInBytes));
                    setBypassed (static_cast<bool> (privateData.getProperty ("Bypass", var (false))));
                }
            }
        */
    }
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            pluginInstance->getStateInformation (destData);

            // With bypass support, Aloe now needs to store private state data.
            // Put this at the end of the plug-in state and add a few null characters
            // so that plug-ins built with older versions of Aloe will hopefully ignore
            // this data. Additionally, we need to add some sort of magic identifier
            // at the very end of the private data so that Aloe has some sort of
            // way to figure out if the data was stored with a newer Aloe version.
            MemoryOutputStream extraData;

            extraData.writeInt64 (0);
            writeAloePrivateStateInformation (extraData);
            auto privateDataSize = (int64) (extraData.getDataSize() - sizeof (int64));
            extraData.writeInt64 (privateDataSize);
            extraData << kAloePrivateDataIdentifier;

            // write magic string
            destData.append (extraData.getData(), extraData.getDataSize());
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:        *const c_void,
        size_as_int: i32)  {
        
        todo!();
        /*
            auto size = (uint64) sizeAsInt;

            // Check if this data was written with a newer Aloe version
            // and if it has the Aloe private data magic code at the end
            auto aloePrivDataIdentifierSize = std::strlen (kAloePrivateDataIdentifier);

            if ((size_t) size >= aloePrivDataIdentifierSize + sizeof (int64))
            {
                auto buffer = static_cast<const char*> (data);

                String magic (CharPointer_UTF8 (buffer + size - aloePrivDataIdentifierSize),
                              CharPointer_UTF8 (buffer + size));

                if (magic == kAloePrivateDataIdentifier)
                {
                    // found a Aloe private data section
                    uint64 privateDataSize;

                    std::memcpy (&privateDataSize,
                                 buffer + ((size_t) size - aloePrivDataIdentifierSize - sizeof (uint64)),
                                 sizeof (uint64));

                    privateDataSize = ByteOrder::swapIfBigEndian (privateDataSize);
                    size -= privateDataSize + aloePrivDataIdentifierSize + sizeof (uint64);

                    if (privateDataSize > 0)
                        setAloePrivateStateInformation (buffer + size, static_cast<int> (privateDataSize));

                    size -= sizeof (uint64);
                }
            }

            if (size > 0)
                pluginInstance->setStateInformation (data, static_cast<int> (size));
        */
    }

    #[cfg(ALOE_Vst3_CAN_REPLACE_Vst2)]
    pub fn load_vst2vst_wblock(&mut self, 
        data: *const u8,
        size: i32) -> bool {
        
        todo!();
        /*
            jassert (ByteOrder::bigEndianInt ("VstW") == htonl ((uint32) readUnaligned<i32> (data)));
            jassert (1 == htonl ((uint32) readUnaligned<i32> (data + 8))); // version should be 1 according to Steinberg's docs

            auto headerLen = (int) htonl ((uint32) readUnaligned<i32> (data + 4)) + 8;
            return loadVst2CcnKBlock (data + headerLen, size - headerLen);
        */
    }
    
    #[cfg(ALOE_Vst3_CAN_REPLACE_Vst2)]
    pub fn load_vst2ccn_kblock(&mut self, 
        data: *const u8,
        size: i32) -> bool {
        
        todo!();
        /*
            auto* bank = reinterpret_cast<const Vst2::fxBank*> (data);

            jassert (ByteOrder::bigEndianInt ("CcnK") == htonl ((uint32) bank->chunkMagic));
            jassert (ByteOrder::bigEndianInt ("FBCh") == htonl ((uint32) bank->fxMagic));
            jassert (htonl ((uint32) bank->version) == 1 || htonl ((uint32) bank->version) == 2);
            jassert (AloePlugin_VstUniqueID == htonl ((uint32) bank->fxID));

            setStateInformation (bank->content.data.chunk,
                                 jmin ((int) (size - (bank->content.data.chunk - data)),
                                       (int) htonl ((uint32) bank->content.data.size)));
            return true;
        */
    }
    
    #[cfg(ALOE_Vst3_CAN_REPLACE_Vst2)]
    pub fn load_vst3preset_file(&mut self, 
        data: *const u8,
        size: i32) -> bool {
        
        todo!();
        /*
            if (size < 48)
                return false;

            // At offset 4 there's a little-endian version number which seems to typically be 1
            // At offset 8 there's 32 bytes the SDK calls "ASCII-encoded class id"
            auto chunkListOffset = (int) ByteOrder::littleEndianInt (data + 40);
            jassert (memcmp (data + chunkListOffset, "List", 4) == 0);
            auto entryCount = (int) ByteOrder::littleEndianInt (data + chunkListOffset + 4);
            jassert (entryCount > 0);

            for (int i = 0; i < entryCount; ++i)
            {
                auto entryOffset = chunkListOffset + 8 + 20 * i;

                if (entryOffset + 20 > size)
                    return false;

                if (memcmp (data + entryOffset, "Comp", 4) == 0)
                {
                    // "Comp" entries seem to contain the data.
                    auto chunkOffset = ByteOrder::littleEndianInt64 (data + entryOffset + 4);
                    auto chunkSize   = ByteOrder::littleEndianInt64 (data + entryOffset + 12);

                    if (static_cast<uint64> (chunkOffset + chunkSize) > static_cast<uint64> (size))
                    {
                        jassertfalse;
                        return false;
                    }

                    loadVst2VstWBlock (data + chunkOffset, (int) chunkSize);
                }
            }

            return true;
        */
    }
    
    #[cfg(ALOE_Vst3_CAN_REPLACE_Vst2)]
    pub fn load_vst2compatible_state(&mut self, 
        data: *const u8,
        size: i32) -> bool {
        
        todo!();
        /*
            if (size < 4)
                return false;

            auto header = htonl ((uint32) readUnaligned<i32> (data));

            if (header == ByteOrder::bigEndianInt ("VstW"))
                return loadVst2VstWBlock (data, size);

            if (header == ByteOrder::bigEndianInt ("CcnK"))
                return loadVst2CcnKBlock (data, size);

            if (memcmp (data, "Vst3", 4) == 0)
            {
                // In Cubase 5, when loading Vst3 .vstpreset files,
                // we get the whole content of the files to load.
                // In Cubase 7 we get just the contents within and
                // we go directly to the loadVst2VstW codepath instead.
                return loadVst3PresetFile (data, size);
            }

            return false;
        */
    }
    
    pub fn load_state_data(&mut self, 
        data: *const c_void,
        size: i32)  {
        
        todo!();
        /*
            #if ALOE_Vst3_CAN_REPLACE_Vst2
            if (loadVst2CompatibleState ((const char*) data, size))
                return;
           #endif
            setStateInformation (data, size);
        */
    }
    
    pub fn read_from_memory_stream(&mut self, state: *mut dyn IBStream) -> bool {
        
        todo!();
        /*
            FUnknownPtr<ISizeableStream> s (state);
            int64 size = 0;

            if (s != nullptr
                 && s->getStreamSize (size) == kResultOk
                 && size > 0
                 && size < 1024 * 1024 * 100) // (some hosts seem to return junk for the size)
            {
                MemoryBlock block (static_cast<size_t> (size));

                // turns out that Cubase 9 might give you the incorrect stream size :-(
                i32 bytesRead = 1;
                int len;

                for (len = 0; bytesRead > 0 && len < static_cast<int> (block.getSize()); len += bytesRead)
                    if (state->read (block.getData(), static_cast<i32> (block.getSize()), &bytesRead) != kResultOk)
                        break;

                if (len == 0)
                    return false;

                block.setSize (static_cast<size_t> (len));

                // Adobe Audition CS6 hack to avoid trying to use corrupted streams:
                if (getHostType().isAdobeAudition())
                    if (block.getSize() >= 5 && memcmp (block.getData(), "VC2!E", 5) == 0)
                        return false;

                loadStateData (block.getData(), (int) block.getSize());
                return true;
            }

            return false;
        */
    }
    
    pub fn read_from_unknown_stream(&mut self, state: *mut dyn IBStream) -> bool {
        
        todo!();
        /*
            MemoryOutputStream allData;

            {
                const size_t bytesPerBlock = 4096;
                HeapBlock<char> buffer (bytesPerBlock);

                for (;;)
                {
                    i32 bytesRead = 0;
                    auto status = state->read (buffer, (i32) bytesPerBlock, &bytesRead);

                    if (bytesRead <= 0 || (status != kResultTrue && ! getHostType().isWavelab()))
                        break;

                    allData.write (buffer, static_cast<size_t> (bytesRead));
                }
            }

            const size_t dataSize = allData.getDataSize();

            if (dataSize <= 0 || dataSize >= 0x7fffffff)
                return false;

            loadStateData (allData.getData(), (int) dataSize);
            return true;
        */
    }

    #[PLUGIN_API]
    pub fn set_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            if (state == nullptr)
                return kInvalidArgument;

            FUnknownPtr<IBStream> stateRefHolder (state); // just in case the caller hasn't properly ref-counted the stream object

            if (state->seek (0, IBStream::kIBSeekSet, nullptr) == kResultTrue)
            {
                if (! getHostType().isFruityLoops() && readFromMemoryStream (state))
                    return kResultTrue;

                if (readFromUnknownStream (state))
                    return kResultTrue;
            }

            return kResultFalse;
        */
    }

    #[cfg(ALOE_Vst3_CAN_REPLACE_Vst2)]
    pub fn write_vst2header(
        state:    *mut IBStream,
        bypassed: bool) -> tresult {
        
        todo!();
        /*
            auto writeVst2IntToState = [state] (uint32 n)
            {
                auto t = (i32) htonl (n);
                return state->write (&t, 4);
            };

            auto status = writeVst2IntToState (ByteOrder::bigEndianInt ("VstW"));

            if (status == kResultOk) status = writeVst2IntToState (8); // header size
            if (status == kResultOk) status = writeVst2IntToState (1); // version
            if (status == kResultOk) status = writeVst2IntToState (bypassed ? 1 : 0); // bypass

            return status;
        */
    }

    #[PLUGIN_API]
    pub fn get_state(&mut self, state: *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            if (state == nullptr)
               return kInvalidArgument;

            MemoryBlock mem;
            getStateInformation (mem);

          #if ALOE_Vst3_CAN_REPLACE_Vst2
            tresult status = writeVst2Header (state, isBypassed());

            if (status != kResultOk)
                return status;

            const int bankBlockSize = 160;
            Vst2::fxBank bank;

            zerostruct (bank);
            bank.chunkMagic         = (i32) htonl (ByteOrder::bigEndianInt ("CcnK"));
            bank.byteSize           = (i32) htonl (bankBlockSize - 8 + (unsigned int) mem.getSize());
            bank.fxMagic            = (i32) htonl (ByteOrder::bigEndianInt ("FBCh"));
            bank.version            = (i32) htonl (2);
            bank.fxID               = (i32) htonl (AloePlugin_VstUniqueID);
            bank.fxVersion          = (i32) htonl (AloePlugin_VersionCode);
            bank.content.data.size  = (i32) htonl ((unsigned int) mem.getSize());

            status = state->write (&bank, bankBlockSize);

            if (status != kResultOk)
                return status;
           #endif

            return state->write (mem.getData(), (i32) mem.getSize());
        */
    }
    
    #[PLUGIN_API]
    pub fn get_unit_count(&mut self) -> i32 {
        
        todo!();
        /*
            return comPluginInstance->getUnitCount();
        */
    }
    
    #[PLUGIN_API]
    pub fn get_unit_info(&mut self, 
        unit_index: i32,
        info:       &mut VstUnitInfo) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->getUnitInfo (unitIndex, info);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_list_count(&mut self) -> i32 {
        
        todo!();
        /*
            return comPluginInstance->getProgramListCount();
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_list_info(&mut self, 
        list_index: i32,
        info:       &mut VstProgramListInfo) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->getProgramListInfo (listIndex, info);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_name(&mut self, 
        list_id:       VstProgramListID,
        program_index: i32,
        name:          VstString128) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->getProgramName (listId, programIndex, name);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_info(&mut self, 
        list_id:         VstProgramListID,
        program_index:   i32,
        attribute_id:    VstCString,
        attribute_value: VstString128) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->getProgramInfo (listId, programIndex, attributeId, attributeValue);
        */
    }
    
    #[PLUGIN_API]
    pub fn has_program_pitch_names(&mut self, 
        list_id:       VstProgramListID,
        program_index: i32) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->hasProgramPitchNames (listId, programIndex);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_pitch_name(&mut self, 
        list_id:       VstProgramListID,
        program_index: i32,
        midi_pitch:    i16,
        name:          VstString128) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->getProgramPitchName (listId, programIndex, midiPitch, name);
        */
    }
    
    #[PLUGIN_API]
    pub fn select_unit(&mut self, unit_id: VstUnitID) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->selectUnit (unitId);
        */
    }
    
    #[PLUGIN_API]
    pub fn set_unit_program_data(&mut self, 
        list_or_unit_id: i32,
        program_index:   i32,
        data:            *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->setUnitProgramData (listOrUnitId, programIndex, data);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_selected_unit(&mut self) -> VstUnitID {
        
        todo!();
        /*
            return comPluginInstance->getSelectedUnit();
        */
    }
    
    #[PLUGIN_API]
    pub fn get_unit_by_bus(&mut self, 
        ty:        VstMediaType,
        dir:       VstBusDirection,
        bus_index: i32,
        channel:   i32,
        unit_id:   &mut VstUnitID) -> tresult {
        
        todo!();
        /*
            return comPluginInstance->getUnitByBus (type, dir, busIndex, channel, unitId);
        */
    }
    
    pub fn get_current_position(&mut self, info: &mut AudioPlayHeadCurrentPositionInfo) -> bool {
        
        todo!();
        /*
            info.timeInSamples              = jmax ((int64) 0, processContext.projectTimeSamples);
            info.timeInSeconds              = static_cast<double> (info.timeInSamples) / processContext.sampleRate;
            info.bpm                        = jmax (1.0, processContext.tempo);
            info.timeSigNumerator           = jmax (1, (int) processContext.timeSigNumerator);
            info.timeSigDenominator         = jmax (1, (int) processContext.timeSigDenominator);
            info.ppqPositionOfLastBarStart  = processContext.barPositionMusic;
            info.ppqPosition                = processContext.projectTimeMusic;
            info.ppqLoopStart               = processContext.cycleStartMusic;
            info.ppqLoopEnd                 = processContext.cycleEndMusic;
            info.isRecording                = (processContext.state & VstProcessContext::kRecording) != 0;
            info.isPlaying                  = (processContext.state & VstProcessContext::kPlaying) != 0;
            info.isLooping                  = (processContext.state & VstProcessContext::kCycleActive) != 0;

            info.frameRate = [&]
            {
                if ((processContext.state & VstProcessContext::kSmpteValid) == 0)
                    return fpsUnknown;

                const auto interpretFlags = [&] (FrameRateType basicRate,
                                                 FrameRateType pullDownRate,
                                                 FrameRateType dropRate,
                                                 FrameRateType pullDownDropRate)
                {
                    switch (processContext.frameRate.flags & (VstFrameRate::kPullDownRate | VstFrameRate::kDropRate))
                    {
                        case VstFrameRate::kPullDownRate | VstFrameRate::kDropRate:
                            return pullDownDropRate;

                        case VstFrameRate::kPullDownRate:
                            return pullDownRate;

                        case VstFrameRate::kDropRate:
                            return dropRate;
                    }

                    return basicRate;
                };

                switch (processContext.frameRate.framesPerSecond)
                {
                    case 24:
                        return interpretFlags (fps24, fps23976, fps24, fps23976);

                    case 25:
                        return interpretFlags (fps25, fps25, fps25, fps25);

                    case 30:
                        return interpretFlags (fps30, fps2997, fps30drop, fps2997drop);

                    case 60:
                        return interpretFlags (fps60, fps60, fps60drop, fps60drop);
                }

                return fpsUnknown;
            }();

            const auto baseFps = (double) processContext.frameRate.framesPerSecond;
            const auto effectiveFps = (processContext.frameRate.flags & VstFrameRate::kPullDownRate) != 0
                                    ? baseFps * 1000.0 / 1001.0
                                    : baseFps;
            info.editOriginTime = (double) processContext.smpteOffsetSubframes / (80.0 * effectiveFps);

            return true;
        */
    }
    
    pub fn get_num_audio_buses(&self, is_input: bool) -> i32 {
        
        todo!();
        /*
            int busCount = pluginInstance->getBusCount (isInput);

          #ifdef AloePlugin_PreferredChannelConfigurations
            short configs[][2] = {AloePlugin_PreferredChannelConfigurations};
            const int numConfigs = sizeof (configs) / sizeof (short[2]);

            bool hasOnlyZeroChannels = true;

            for (int i = 0; i < numConfigs && hasOnlyZeroChannels == true; ++i)
                if (configs[i][isInput ? 0 : 1] != 0)
                    hasOnlyZeroChannels = false;

            busCount = jmin (busCount, hasOnlyZeroChannels ? 0 : 1);
           #endif

            return busCount;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_bus_count(&mut self, 
        ty:  VstMediaType,
        dir: VstBusDirection) -> i32 {
        
        todo!();
        /*
            if (type == VstkAudio)
                return getNumAudioBuses (dir == VstkInput);

            if (type == VstkEvent)
            {
               #if AloePlugin_WantsMidiInput
                if (dir == VstkInput)
                    return 1;
               #endif

               #if AloePlugin_ProducesMidiOutput
                if (dir == VstkOutput)
                    return 1;
               #endif
            }

            return 0;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_bus_info(&mut self, 
        ty:    VstMediaType,
        dir:   VstBusDirection,
        index: i32,
        info:  &mut VstBusInfo) -> tresult {
        
        todo!();
        /*
            if (type == VstkAudio)
            {
                if (index < 0 || index >= getNumAudioBuses (dir == VstkInput))
                    return kResultFalse;

                if (auto* bus = pluginInstance->getBus (dir == VstkInput, index))
                {
                    info.mediaType = VstkAudio;
                    info.direction = dir;
                    info.channelCount = bus->getLastEnabledLayout().size();
                    toString128 (info.name, bus->getName());

                    info.busType = [&]
                    {
                        const auto isFirstBus = (index == 0);

                        if (dir == VstkInput)
                        {
                            if (isFirstBus)
                            {
                                if (auto* extensions = dynamic_cast<Vst3ClientExtensions*> (pluginInstance))
                                    return extensions->getPluginHasMainInput() ? VstkMain : VstkAux;

                                return VstkMain;
                            }

                            return VstkAux;
                        }

                       #if AloePlugin_IsSynth
                        return VstkMain;
                       #else
                        return isFirstBus ? VstkMain : VstkAux;
                       #endif
                    }();

                   #ifdef AloePlugin_PreferredChannelConfigurations
                    info.flags = VstBusInfo::kDefaultActive;
                   #else
                    info.flags = (bus->isEnabledByDefault()) ? VstBusInfo::kDefaultActive : 0;
                   #endif

                    return kResultTrue;
                }
            }

            if (type == VstkEvent)
            {
                info.flags = VstBusInfo::kDefaultActive;

               #if AloePlugin_WantsMidiInput
                if (dir == VstkInput && index == 0)
                {
                    info.mediaType = VstkEvent;
                    info.direction = dir;

                   #ifdef AloePlugin_VstNumMidiInputs
                    info.channelCount = AloePlugin_VstNumMidiInputs;
                   #else
                    info.channelCount = 16;
                   #endif

                    toString128 (info.name, TRANS("MIDI Input"));
                    info.busType = VstkMain;
                    return kResultTrue;
                }
               #endif

               #if AloePlugin_ProducesMidiOutput
                if (dir == VstkOutput && index == 0)
                {
                    info.mediaType = VstkEvent;
                    info.direction = dir;

                   #ifdef AloePlugin_VstNumMidiOutputs
                    info.channelCount = AloePlugin_VstNumMidiOutputs;
                   #else
                    info.channelCount = 16;
                   #endif

                    toString128 (info.name, TRANS("MIDI Output"));
                    info.busType = VstkMain;
                    return kResultTrue;
                }
               #endif
            }

            zerostruct (info);
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn activate_bus(&mut self, 
        ty:    VstMediaType,
        dir:   VstBusDirection,
        index: i32,
        state: TBool) -> tresult {
        
        todo!();
        /*
            if (type == VstkEvent)
            {
               #if AloePlugin_WantsMidiInput
                if (index == 0 && dir == VstkInput)
                {
                    isMidiInputBusEnabled = (state != 0);
                    return kResultTrue;
                }
               #endif

               #if AloePlugin_ProducesMidiOutput
                if (index == 0 && dir == VstkOutput)
                {
                    isMidiOutputBusEnabled = (state != 0);
                    return kResultTrue;
                }
               #endif

                return kResultFalse;
            }

            if (type == VstkAudio)
            {
                if (index < 0 || index >= getNumAudioBuses (dir == VstkInput))
                    return kResultFalse;

                if (auto* bus = pluginInstance->getBus (dir == VstkInput, index))
                {
                   #ifdef AloePlugin_PreferredChannelConfigurations
                    auto newLayout = pluginInstance->getBusesLayout();
                    auto targetLayout = (state != 0 ? bus->getLastEnabledLayout() : AudioChannelSet::disabled());

                    (dir == VstkInput ? newLayout.inputBuses : newLayout.outputBuses).getReference (index) = targetLayout;

                    short configs[][2] = { AloePlugin_PreferredChannelConfigurations };
                    auto compLayout = pluginInstance->getNextBestLayoutInLayoutList (newLayout, configs);

                    if ((dir == VstkInput ? compLayout.inputBuses : compLayout.outputBuses).getReference (index) != targetLayout)
                        return kResultFalse;
                   #endif

                    return bus->enable (state != 0) ? kResultTrue : kResultFalse;
                }
            }

            return kResultFalse;
        */
    }
    
    pub fn check_bus_formats_are_not_discrete(&mut self) -> bool {
        
        todo!();
        /*
            auto numInputBuses  = pluginInstance->getBusCount (true);
            auto numOutputBuses = pluginInstance->getBusCount (false);

            for (int i = 0; i < numInputBuses; ++i)
            {
                auto layout = pluginInstance->getChannelLayoutOfBus (true,  i);

                if (layout.isDiscreteLayout() && ! layout.isDisabled())
                    return false;
            }

            for (int i = 0; i < numOutputBuses; ++i)
            {
                auto layout = pluginInstance->getChannelLayoutOfBus (false,  i);

                if (layout.isDiscreteLayout() && ! layout.isDisabled())
                    return false;
            }

            return true;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_bus_arrangements(&mut self, 
        inputs:   *mut VstSpeakerArrangement,
        num_ins:  i32,
        outputs:  *mut VstSpeakerArrangement,
        num_outs: i32) -> tresult {
        
        todo!();
        /*
            auto numInputBuses  = pluginInstance->getBusCount (true);
            auto numOutputBuses = pluginInstance->getBusCount (false);

            if (numIns > numInputBuses || numOuts > numOutputBuses)
                return false;

            auto requested = pluginInstance->getBusesLayout();

            for (int i = 0; i < numIns; ++i)
                requested.getChannelSet (true,  i) = getChannelSetForSpeakerArrangement (inputs[i]);

            for (int i = 0; i < numOuts; ++i)
                requested.getChannelSet (false, i) = getChannelSetForSpeakerArrangement (outputs[i]);

           #ifdef AloePlugin_PreferredChannelConfigurations
            short configs[][2] = { AloePlugin_PreferredChannelConfigurations };
            if (! AudioProcessor::containsLayout (requested, configs))
                return kResultFalse;
           #endif

            return pluginInstance->setBusesLayoutWithoutEnabling (requested) ? kResultTrue : kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_bus_arrangement(&mut self, 
        dir:   VstBusDirection,
        index: i32,
        arr:   &mut VstSpeakerArrangement) -> tresult {
        
        todo!();
        /*
            if (auto* bus = pluginInstance->getBus (dir == VstkInput, index))
            {
                arr = getVst3SpeakerArrangement (bus->getLastEnabledLayout());
                return kResultTrue;
            }

            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn can_process_sample_size(&mut self, symbolic_sample_size: i32) -> tresult {
        
        todo!();
        /*
            return (symbolicSampleSize == VstkSample32
                     || (getPluginInstance().supportsDoublePrecisionProcessing()
                           && symbolicSampleSize == VstkSample64)) ? kResultTrue : kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_latency_samples(&mut self) -> u32 {
        
        todo!();
        /*
            return (uint32) jmax (0, getPluginInstance().getLatencySamples());
        */
    }
    
    #[PLUGIN_API]
    pub fn setup_processing(&mut self, new_setup: &mut VstProcessSetup) -> tresult {
        
        todo!();
        /*
            AloeVst3ComponentScopedInSetupProcessingSetter inSetupProcessingSetter (aloeVst3EditController);

            if (canProcessSampleSize (newSetup.symbolicSampleSize) != kResultTrue)
                return kResultFalse;

            processSetup = newSetup;
            processContext.sampleRate = processSetup.sampleRate;

            getPluginInstance().setProcessingPrecision (newSetup.symbolicSampleSize == VstkSample64
                                                            ? AudioProcessor::doublePrecision
                                                            : AudioProcessor::singlePrecision);
            getPluginInstance().setNonRealtime (newSetup.processMode == VstkOffline);

            preparePlugin (processSetup.sampleRate, processSetup.maxSamplesPerBlock);

            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn set_processing(&mut self, state: TBool) -> tresult {
        
        todo!();
        /*
            if (! state)
                getPluginInstance().reset();

            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_tail_samples(&mut self) -> u32 {
        
        todo!();
        /*
            auto tailLengthSeconds = getPluginInstance().getTailLengthSeconds();

            if (tailLengthSeconds <= 0.0 || processSetup.sampleRate <= 0.0)
                return VstkNoTail;

            if (tailLengthSeconds == std::numeric_limits<double>::infinity())
                return VstkInfiniteTail;

            return (uint32) roundToIntAccurate (tailLengthSeconds * processSetup.sampleRate);
        */
    }
    
    pub fn process_parameter_changes(&mut self, param_changes: &mut VstIParameterChanges)  {
        
        todo!();
        /*
            jassert (pluginInstance != nullptr);

            auto numParamsChanged = paramChanges.getParameterCount();

            for (i32 i = 0; i < numParamsChanged; ++i)
            {
                if (auto* paramQueue = paramChanges.getParameterData (i))
                {
                    auto numPoints = paramQueue->getPointCount();

                    i32 offsetSamples = 0;
                    double value = 0.0;

                    if (paramQueue->getPoint (numPoints - 1, offsetSamples, value) == kResultTrue)
                    {
                        auto vstParamID = paramQueue->getParameterId();

                       #if ALOE_Vst3_EMULATE_MIDI_CC_WITH_PARAMETERS
                        if (aloeVst3EditController != nullptr && aloeVst3EditController->isMidiControllerParamID (vstParamID))
                            addParameterChangeToMidiBuffer (offsetSamples, vstParamID, value);
                        else
                       #endif
                        {
                            auto floatValue = static_cast<float> (value);

                            if (auto* param = comPluginInstance->getParamForVstParamID (vstParamID))
                            {
                                param->setValue (floatValue);

                                const InParameterChangedCallbackSetter scopedSetter { inParameterChangedCallback };
                                param->sendValueChangedMessageToListeners (floatValue);
                            }
                        }
                    }
                }
            }
        */
    }
    
    pub fn add_parameter_change_to_midi_buffer(&mut self, 
        offset_samples: i32,
        id:             VstParamID,
        value:          f64)  {
        
        todo!();
        /*
            // If the parameter is mapped to a MIDI CC message then insert it into the midiBuffer.
            int channel, ctrlNumber;

            if (aloeVst3EditController->getMidiControllerForParameter (id, channel, ctrlNumber))
            {
                if (ctrlNumber == VstkAfterTouch)
                    midiBuffer.addEvent (MidiMessage::channelPressureChange (channel,
                                                                             jlimit (0, 127, (int) (value * 128.0))), offsetSamples);
                else if (ctrlNumber == VstkPitchBend)
                    midiBuffer.addEvent (MidiMessage::pitchWheel (channel,
                                                                  jlimit (0, 0x3fff, (int) (value * 0x4000))), offsetSamples);
                else
                    midiBuffer.addEvent (MidiMessage::controllerEvent (channel,
                                                                       jlimit (0, 127, ctrlNumber),
                                                                       jlimit (0, 127, (int) (value * 128.0))), offsetSamples);
            }
        */
    }
    
    #[PLUGIN_API]
    pub fn process(&mut self, data: &mut VstProcessData) -> tresult {
        
        todo!();
        /*
            if (pluginInstance == nullptr)
                return kResultFalse;

            if ((processSetup.symbolicSampleSize == VstkSample64) != pluginInstance->isUsingDoublePrecision())
                return kResultFalse;

            if (data.processContext != nullptr)
            {
                processContext = *data.processContext;

                if (aloeVst3EditController != nullptr)
                    aloeVst3EditController->vst3IsPlaying = (processContext.state & VstProcessContext::kPlaying) != 0;
            }
            else
            {
                zerostruct (processContext);

                if (aloeVst3EditController != nullptr)
                    aloeVst3EditController->vst3IsPlaying = false;
            }

            midiBuffer.clear();

            if (data.inputParameterChanges != nullptr)
                processParameterChanges (*data.inputParameterChanges);

           #if AloePlugin_WantsMidiInput
            if (isMidiInputBusEnabled && data.inputEvents != nullptr)
                MidiEventList::toMidiBuffer (midiBuffer, *data.inputEvents);
           #endif

            if (getHostType().isWavelab())
            {
                const int numInputChans  = (data.inputs  != nullptr && data.inputs[0].channelBuffers32 != nullptr)  ? (int) data.inputs[0].numChannels  : 0;
                const int numOutputChans = (data.outputs != nullptr && data.outputs[0].channelBuffers32 != nullptr) ? (int) data.outputs[0].numChannels : 0;

                if ((pluginInstance->getTotalNumInputChannels() + pluginInstance->getTotalNumOutputChannels()) > 0
                     && (numInputChans + numOutputChans) == 0)
                    return kResultFalse;
            }

            if      (processSetup.symbolicSampleSize == VstkSample32) processAudio<float>  (data, channelListFloat);
            else if (processSetup.symbolicSampleSize == VstkSample64) processAudio<double> (data, channelListDouble);
            else jassertfalse;

           #if AloePlugin_ProducesMidiOutput
            if (isMidiOutputBusEnabled && data.outputEvents != nullptr)
                MidiEventList::pluginToHostEventList (*data.outputEvents, midiBuffer);
           #endif

            return kResultTrue;
        */
    }
    
    pub fn query_interface_internal(&mut self, targetiid: TUID) -> InterfaceResultWithDeferredAddRef {
        
        todo!();
        /*
            const auto result = testForMultiple (*this,
                                                 targetIID,
                                                 UniqueBase<IPluginBase>{},
                                                 UniqueBase<AloeVst3Component>{},
                                                 UniqueBase<VstIComponent>{},
                                                 UniqueBase<VstIAudioProcessor>{},
                                                 UniqueBase<VstIUnitInfo>{},
                                                 UniqueBase<VstIConnectionPoint>{},
                                                 UniqueBase<VstIProcessContextRequirements>{},
                                                 SharedBase<FUnknown, VstIComponent>{});

            if (result.isOk())
                return result;

            if (doUIDsMatch (targetIID, AloeAudioProcessor::iid))
                return { kResultOk, comPluginInstance.get() };

            return {};
        */
    }
    
    
    pub fn process_audio<FloatType>(&mut self, 
        data:         &mut VstProcessData,
        channel_list: &mut Vec<*mut FloatType>)  {
    
        todo!();
        /*
            int totalInputChans = 0, totalOutputChans = 0;
            bool tmpBufferNeedsClearing = false;

            auto plugInInputChannels  = pluginInstance->getTotalNumInputChannels();
            auto plugInOutputChannels = pluginInstance->getTotalNumOutputChannels();

            // Wavelab workaround: wave-lab lies on the number of inputs/outputs so re-count here
            const auto countValidChannels = [] (VstAudioBusBuffers* buffers, i32 num)
            {
                return int (std::distance (buffers, std::find_if (buffers, buffers + num, [] (VstAudioBusBuffers& buf)
                {
                    return getPointerForAudioBus<FloatType> (buf) == nullptr && buf.numChannels > 0;
                })));
            };

            const auto vstInputs  = countValidChannels (data.inputs,  data.numInputs);
            const auto vstOutputs = countValidChannels (data.outputs, data.numOutputs);

            {
                auto n = jmax (vstOutputs, getNumAudioBuses (false));

                for (int bus = 0; bus < n && totalOutputChans < plugInOutputChannels; ++bus)
                {
                    if (auto* busObject = pluginInstance->getBus (false, bus))
                        if (! busObject->isEnabled())
                            continue;

                    if (bus < vstOutputs)
                    {
                        if (auto** const busChannels = getPointerForAudioBus<FloatType> (data.outputs[bus]))
                        {
                            auto numChans = jmin ((int) data.outputs[bus].numChannels, plugInOutputChannels - totalOutputChans);

                            for (int i = 0; i < numChans; ++i)
                            {
                                if (auto dst = busChannels[i])
                                {
                                    if (totalOutputChans >= plugInInputChannels)
                                        FloatVectorOperations::clear (dst, (int) data.numSamples);

                                    channelList.set (totalOutputChans++, busChannels[i]);
                                }
                            }
                        }
                    }
                    else
                    {
                        const int numChans = jmin (pluginInstance->getChannelCountOfBus (false, bus), plugInOutputChannels - totalOutputChans);

                        for (int i = 0; i < numChans; ++i)
                        {
                            if (auto* tmpBuffer = getTmpBufferForChannel<FloatType> (totalOutputChans, data.numSamples))\
                            {
                                tmpBufferNeedsClearing = true;
                                channelList.set (totalOutputChans++, tmpBuffer);
                            }
                            else
                                return;
                        }
                    }
                }
            }

            {
                auto n = jmax (vstInputs, getNumAudioBuses (true));

                for (int bus = 0; bus < n && totalInputChans < plugInInputChannels; ++bus)
                {
                    if (auto* busObject = pluginInstance->getBus (true, bus))
                        if (! busObject->isEnabled())
                            continue;

                    if (bus < vstInputs)
                    {
                        if (auto** const busChannels = getPointerForAudioBus<FloatType> (data.inputs[bus]))
                        {
                            const int numChans = jmin ((int) data.inputs[bus].numChannels, plugInInputChannels - totalInputChans);

                            for (int i = 0; i < numChans; ++i)
                            {
                                if (busChannels[i] != nullptr)
                                {
                                    if (totalInputChans >= totalOutputChans)
                                        channelList.set (totalInputChans, busChannels[i]);
                                    else
                                    {
                                        auto* dst = channelList.getReference (totalInputChans);
                                        auto* src = busChannels[i];

                                        if (dst != src)
                                            FloatVectorOperations::copy (dst, src, (int) data.numSamples);
                                    }
                                }

                                ++totalInputChans;
                            }
                        }
                    }
                    else
                    {
                        auto numChans = jmin (pluginInstance->getChannelCountOfBus (true, bus), plugInInputChannels - totalInputChans);

                        for (int i = 0; i < numChans; ++i)
                        {
                            if (auto* tmpBuffer = getTmpBufferForChannel<FloatType> (totalInputChans, data.numSamples))
                            {
                                tmpBufferNeedsClearing = true;
                                channelList.set (totalInputChans++, tmpBuffer);
                            }
                            else
                                return;
                        }
                    }
                }
            }

            if (tmpBufferNeedsClearing)
                ChooseBufferHelper<FloatType>::impl (emptyBufferFloat, emptyBufferDouble).clear();

            AudioBuffer<FloatType> buffer;

            if (int totalChans = jmax (totalOutputChans, totalInputChans))
                buffer.setDataToReferTo (channelList.getRawDataPointer(), totalChans, (int) data.numSamples);

            {
                const ScopedLock sl (pluginInstance->getCallbackLock());

                pluginInstance->setNonRealtime (data.processMode == VstkOffline);

               #if ALOE_DEBUG && ! AloePlugin_ProducesMidiOutput
                const int numMidiEventsComingIn = midiBuffer.getNumEvents();
               #endif

                if (pluginInstance->isSuspended())
                {
                    buffer.clear();
                }
                else
                {
                    if (totalInputChans == pluginInstance->getTotalNumInputChannels()
                     && totalOutputChans == pluginInstance->getTotalNumOutputChannels())
                    {
                        if (isBypassed())
                            pluginInstance->processBlockBypassed (buffer, midiBuffer);
                        else
                            pluginInstance->processBlock (buffer, midiBuffer);
                    }
                }

               #if ALOE_DEBUG && (! AloePlugin_ProducesMidiOutput)
                /*  This assertion is caused when you've added some events to the
                    midiMessages array in your processBlock() method, which usually means
                    that you're trying to send them somewhere. But in this case they're
                    getting thrown away.

                    If your plugin does want to send MIDI messages, you'll need to set
                    the AloePlugin_ProducesMidiOutput macro to 1 in your
                    AloePluginCharacteristics.h file.

                    If you don't want to produce any MIDI output, then you should clear the
                    midiMessages array at the end of your processBlock() method, to
                    indicate that you don't want any of the events to be passed through
                    to the output.
                */
                jassert (midiBuffer.getNumEvents() <= numMidiEventsComingIn);
               #endif
            }

            if (auto* changes = data.outputParameterChanges)
            {
                comPluginInstance->forAllChangedParameters ([&] (VstParamID paramID, float value)
                {
                    i32 queueIndex = 0;

                    if (auto* queue = changes->addParameterData (paramID, queueIndex))
                    {
                        i32 pointIndex = 0;
                        queue->addPoint (0, value, pointIndex);
                    }
                });
            }
        */
    }
    
    
    pub fn allocate_channel_list_and_buffers<FloatType>(&mut self, 
        channel_list: &mut Vec<*mut FloatType>,
        buffer:       &mut AudioBuffer<FloatType>)  {
    
        todo!();
        /*
            channelList.clearQuick();
            channelList.insertMultiple (0, nullptr, 128);

            auto& p = getPluginInstance();
            buffer.setSize (jmax (p.getTotalNumInputChannels(), p.getTotalNumOutputChannels()), p.getBlockSize() * 4);
            buffer.clear();
        */
    }
    
    
    pub fn deallocate_channel_list_and_buffers<FloatType>(&mut self, 
        channel_list: &mut Vec<*mut FloatType>,
        buffer:       &mut AudioBuffer<FloatType>)  {
    
        todo!();
        /*
            channelList.clearQuick();
            channelList.resize (0);
            buffer.setSize (0, 0);
        */
    }
    
    
    pub fn get_pointer_for_audio_bus<FloatType>(data: &mut VstAudioBusBuffers) -> *mut *mut FloatType {
    
        todo!();
        /*
            return AudioBusPointerHelper<FloatType>::impl (data);
        */
    }
    
    
    pub fn get_tmp_buffer_for_channel<FloatType>(&mut self, 
        channel:     i32,
        num_samples: i32) -> *mut FloatType {
    
        todo!();
        /*
            auto& buffer = ChooseBufferHelper<FloatType>::impl (emptyBufferFloat, emptyBufferDouble);

            // we can't do anything if the host requests to render many more samples than the
            // block size, we need to bail out
            if (numSamples > buffer.getNumSamples() || channel >= buffer.getNumChannels())
                return nullptr;

            return buffer.getWritePointer (channel);
        */
    }
    
    #[PLUGIN_API]
    pub fn get_process_context_requirements(&mut self) -> u32 {
        
        todo!();
        /*
            return kNeedSystemTime
                 | kNeedContinousTimeSamples
                 | kNeedProjectTimeMusic
                 | kNeedBarPositionMusic
                 | kNeedCycleMusic
                 | kNeedSamplesToNextClock
                 | kNeedTempo
                 | kNeedTimeSignature
                 | kNeedChord
                 | kNeedFrameRate
                 | kNeedTransportState;
        */
    }
    
    pub fn prepare_plugin(&mut self, 
        sample_rate: f64,
        buffer_size: i32)  {
        
        todo!();
        /*
            auto& p = getPluginInstance();

            p.setRateAndBufferSizeDetails (sampleRate, bufferSize);
            p.prepareToPlay (sampleRate, bufferSize);

            midiBuffer.ensureSize (2048);
            midiBuffer.clear();
        */
    }
}
