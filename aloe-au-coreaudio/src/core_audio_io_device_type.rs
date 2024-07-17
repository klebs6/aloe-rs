crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_mac_CoreAudio.cpp]
#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct CoreAudioIODeviceType<'a> {
    base:                AudioIODeviceType,
    base2:               AsyncUpdater<'a>,
    input_device_names:  StringArray,
    output_device_names: StringArray,
    input_ids:           Vec<AudioDeviceID>,
    output_ids:          Vec<AudioDeviceID>,
    has_scanned:         bool, // default = false
}

impl<'a> Default for CoreAudioIODeviceType<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : audio_io_device_type("CoreAudio"),

            AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioHardwarePropertyDevices;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = kAudioObjectPropertyElementWildcard;

                AudioObjectAddPropertyListener (kAudioObjectSystemObject, &pa, hardwareListenerProc, this)
        */
    }
}

impl<'a> Drop for CoreAudioIODeviceType<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioHardwarePropertyDevices;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = kAudioObjectPropertyElementWildcard;

                AudioObjectRemovePropertyListener (kAudioObjectSystemObject, &pa, hardwareListenerProc, this);
             */
    }
}

impl<'a> CoreAudioIODeviceType<'a> {

    pub fn scan_for_devices(&mut self)  {
        
        todo!();
        /*
            hasScanned = true;

                inputDeviceNames.clear();
                outputDeviceNames.clear();
                inputIds.clear();
                outputIds.clear();

                UInt32 size;

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioHardwarePropertyDevices;
                pa.mScope = kAudioObjectPropertyScopeWildcard;
                pa.mElement = aloeAudioObjectPropertyElementMain;

                if (AudioObjectGetPropertyDataSize (kAudioObjectSystemObject, &pa, 0, nullptr, &size) == noErr)
                {
                    HeapBlock<AudioDeviceID> devs;
                    devs.calloc (size, 1);

                    if (AudioObjectGetPropertyData (kAudioObjectSystemObject, &pa, 0, nullptr, &size, devs) == noErr)
                    {
                        auto num = (int) size / (int) sizeof (AudioDeviceID);

                        for (int i = 0; i < num; ++i)
                        {
                            char name[1024];
                            size = sizeof (name);
                            pa.mSelector = kAudioDevicePropertyDeviceName;

                            if (AudioObjectGetPropertyData (devs[i], &pa, 0, nullptr, &size, name) == noErr)
                            {
                                auto nameString = String::fromUTF8 (name, (int) strlen (name));
                                auto numIns  = getNumChannels (devs[i], true);
                                auto numOuts = getNumChannels (devs[i], false);

                                if (numIns > 0)
                                {
                                    inputDeviceNames.add (nameString);
                                    inputIds.add (devs[i]);
                                }

                                if (numOuts > 0)
                                {
                                    outputDeviceNames.add (nameString);
                                    outputIds.add (devs[i]);
                                }
                            }
                        }
                    }
                }

                inputDeviceNames.appendNumbersToDuplicates (false, true);
                outputDeviceNames.appendNumbersToDuplicates (false, true);
        */
    }
    
    pub fn get_device_names(&self, want_input_names: bool) -> StringArray {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

                return wantInputNames ? inputDeviceNames
                                      : outputDeviceNames;
        */
    }
    
    pub fn get_default_device_index(&self, for_input: bool) -> i32 {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

                AudioDeviceID deviceID;
                UInt32 size = sizeof (deviceID);

                // if they're asking for any input channels at all, use the default input, so we
                // get the built-in mic rather than the built-in output with no inputs..

                AudioObjectPropertyAddress pa;
                pa.mSelector = forInput ? kAudioHardwarePropertyDefaultInputDevice
                                        : kAudioHardwarePropertyDefaultOutputDevice;
                pa.mScope    = kAudioObjectPropertyScopeWildcard;
                pa.mElement  = aloeAudioObjectPropertyElementMain;

                if (AudioObjectGetPropertyData (kAudioObjectSystemObject, &pa, 0, nullptr, &size, &deviceID) == noErr)
                {
                    if (forInput)
                    {
                        for (int i = inputIds.size(); --i >= 0;)
                            if (inputIds[i] == deviceID)
                                return i;
                    }
                    else
                    {
                        for (int i = outputIds.size(); --i >= 0;)
                            if (outputIds[i] == deviceID)
                                return i;
                    }
                }

                return 0;
        */
    }
    
    pub fn get_index_of_device(&self, 
        device:   *mut AudioIODevice,
        as_input: bool) -> i32 {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

                if (auto* d = dynamic_cast<CoreAudioIODevice*> (device))
                    return asInput ? d->inputIndex
                                   : d->outputIndex;

                if (auto* d = dynamic_cast<AudioIODeviceCombiner*> (device))
                {
                    for (auto* dev : d->getDevices())
                    {
                        auto index = getIndexOfDevice (dev, asInput);

                        if (index >= 0)
                            return index;
                    }
                }

                return -1;
        */
    }
    
    pub fn has_separate_inputs_and_outputs(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn create_device(&mut self, 
        output_device_name: &String,
        input_device_name:  &String) -> *mut AudioIODevice {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

                auto inputIndex  = inputDeviceNames.indexOf (inputDeviceName);
                auto outputIndex = outputDeviceNames.indexOf (outputDeviceName);

                auto inputDeviceID  = inputIds[inputIndex];
                auto outputDeviceID = outputIds[outputIndex];

                if (inputDeviceID == 0 && outputDeviceID == 0)
                    return nullptr;

                auto combinedName = outputDeviceName.isEmpty() ? inputDeviceName
                                                               : outputDeviceName;

                if (inputDeviceID == outputDeviceID)
                    return new CoreAudioIODevice (this, combinedName, inputDeviceID, inputIndex, outputDeviceID, outputIndex);

                std::unique_ptr<CoreAudioIODevice> in, out;

                if (inputDeviceID != 0)
                    in.reset (new CoreAudioIODevice (this, inputDeviceName, inputDeviceID, inputIndex, 0, -1));

                if (outputDeviceID != 0)
                    out.reset (new CoreAudioIODevice (this, outputDeviceName, 0, -1, outputDeviceID, outputIndex));

                if (in == nullptr)   return out.release();
                if (out == nullptr)  return in.release();

                std::unique_ptr<AudioIODeviceCombiner> combo (new AudioIODeviceCombiner (combinedName, this));
                combo->addDevice (in.release(),  true, false);
                combo->addDevice (out.release(), false, true);
                return combo.release();
        */
    }
    
    pub fn audio_device_list_changed(&mut self)  {
        
        todo!();
        /*
            scanForDevices();
                callDeviceChangeListeners();
        */
    }
    
    pub fn trigger_async_audio_device_list_change(&mut self)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    pub fn get_num_channels(
        deviceid: AudioDeviceID,
        input:    bool) -> i32 {
        
        todo!();
        /*
            int total = 0;
                UInt32 size;

                AudioObjectPropertyAddress pa;
                pa.mSelector = kAudioDevicePropertyStreamConfiguration;
                pa.mScope = input ? kAudioDevicePropertyScopeInput : kAudioDevicePropertyScopeOutput;
                pa.mElement = aloeAudioObjectPropertyElementMain;

                if (AudioObjectGetPropertyDataSize (deviceID, &pa, 0, nullptr, &size) == noErr)
                {
                    HeapBlock<AudioBufferList> bufList;
                    bufList.calloc (size, 1);

                    if (AudioObjectGetPropertyData (deviceID, &pa, 0, nullptr, &size, bufList) == noErr)
                    {
                        auto numStreams = (int) bufList->mNumberBuffers;

                        for (int i = 0; i < numStreams; ++i)
                            total += bufList->mBuffers[i].mNumberChannels;
                    }
                }

                return total;
        */
    }
    
    pub fn hardware_listener_proc(
        _0:          AudioDeviceID,
        _1:          u32,
        _2:          *const AudioObjectPropertyAddress,
        client_data: *mut c_void) -> OSStatus {
        
        todo!();
        /*
            static_cast<CoreAudioIODeviceType*> (clientData)->triggerAsyncAudioDeviceListChange();
                return noErr;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            audioDeviceListChanged();
        */
    }
}
