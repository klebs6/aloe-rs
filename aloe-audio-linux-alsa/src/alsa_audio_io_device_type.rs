crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ALSAAudioIODeviceType {
    base:                 AudioIODeviceType,
    input_names:          Vec<String>,
    output_names:         Vec<String>,
    input_ids:            Vec<String>,
    output_ids:           Vec<String>,
    has_scanned:          bool, // default = false
    list_only_soundcards: bool,
}

impl Drop for ALSAAudioIODeviceType {

    fn drop(&mut self) {
        todo!();
        /* 
           #if ! ALOE_ALSA_LOGGING
            snd_lib_error_set_handler (nullptr);
           #endif

            snd_config_update_free_global(); // prevent valgrind from screaming about alsa leaks
         */
    }
}

impl ALSAAudioIODeviceType {
    
    pub fn new(
        only_soundcards:  bool,
        device_type_name: &String) -> Self {
    
        todo!();
        /*
        : audio_io_device_type(deviceTypeName),
        : list_only_soundcards(onlySoundcards),

            #if ! ALOE_ALSA_LOGGING
            snd_lib_error_set_handler (&silentErrorHandler);
           #endif
        */
    }
    
    pub fn scan_for_devices(&mut self)  {
        
        todo!();
        /*
            if (hasScanned)
                return;

            hasScanned = true;
            inputNames.clear();
            inputIds.clear();
            outputNames.clear();
            outputIds.clear();

            ALOE_ALSA_LOG ("scanForDevices()");

            if (listOnlySoundcards)
                enumerateAlsaSoundcards();
            else
                enumerateAlsaPCMDevices();

            inputNames.appendNumbersToDuplicates (false, true);
            outputNames.appendNumbersToDuplicates (false, true);
        */
    }
    
    pub fn get_device_names(&self, want_input_names: bool) -> Vec<String> {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

            return wantInputNames ? inputNames : outputNames;
        */
    }
    
    pub fn get_default_device_index(&self, for_input: bool) -> i32 {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

            auto idx = (forInput ? inputIds : outputIds).indexOf ("default");
            return idx >= 0 ? idx : 0;
        */
    }
    
    pub fn has_separate_inputs_and_outputs(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_index_of_device(&self, 
        device:   *mut AudioIODevice,
        as_input: bool) -> i32 {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

            if (auto* d = dynamic_cast<ALSAAudioIODevice*> (device))
                return asInput ? inputIds.indexOf (d->inputId)
                               : outputIds.indexOf (d->outputId);

            return -1;
        */
    }
    
    pub fn create_device(&mut self, 
        output_device_name: &String,
        input_device_name:  &String) -> *mut AudioIODevice {
        
        todo!();
        /*
            jassert (hasScanned); // need to call scanForDevices() before doing this

            auto inputIndex = inputNames.indexOf (inputDeviceName);
            auto outputIndex = outputNames.indexOf (outputDeviceName);

            String deviceName (outputIndex >= 0 ? outputDeviceName
                                                : inputDeviceName);

            if (inputIndex >= 0 || outputIndex >= 0)
                return new ALSAAudioIODevice (deviceName, getTypeName(),
                                              inputIds [inputIndex],
                                              outputIds [outputIndex]);

            return nullptr;
        */
    }
    
    pub fn test_device(&mut self, 
        id:          &String,
        output_name: &String,
        input_name:  &String) -> bool {
        
        todo!();
        /*
            unsigned int minChansOut = 0, maxChansOut = 0;
            unsigned int minChansIn = 0, maxChansIn = 0;
            Vec<double> rates;

            bool isInput = inputName.isNotEmpty(), isOutput = outputName.isNotEmpty();
            getDeviceProperties (id, minChansOut, maxChansOut, minChansIn, maxChansIn, rates, isOutput, isInput);

            isInput  = maxChansIn > 0;
            isOutput = maxChansOut > 0;

            if ((isInput || isOutput) && rates.size() > 0)
            {
                ALOE_ALSA_LOG ("testDevice: '" << id.toUTF8().getAddress() << "' -> isInput: "
                                << (int) isInput << ", isOutput: " << (int) isOutput);

                if (isInput)
                {
                    inputNames.add (inputName);
                    inputIds.add (id);
                }

                if (isOutput)
                {
                    outputNames.add (outputName);
                    outputIds.add (id);
                }

                return isInput || isOutput;
            }

            return false;
        */
    }
    
    pub fn enumerate_alsa_soundcards(&mut self)  {
        
        todo!();
        /*
            snd_ctl_t* handle = nullptr;
            snd_ctl_card_info_t* info = nullptr;
            snd_ctl_card_info_alloca (&info);

            int cardNum = -1;

            while (outputIds.size() + inputIds.size() <= 64)
            {
                snd_card_next (&cardNum);

                if (cardNum < 0)
                    break;

                if (ALOE_CHECKED_RESULT (snd_ctl_open (&handle, ("hw:" + String (cardNum)).toRawUTF8(), SND_CTL_NONBLOCK)) >= 0)
                {
                    if (ALOE_CHECKED_RESULT (snd_ctl_card_info (handle, info)) >= 0)
                    {
                        String cardId (snd_ctl_card_info_get_id (info));

                        if (cardId.removeCharacters ("0123456789").isEmpty())
                            cardId = String (cardNum);

                        String cardName = snd_ctl_card_info_get_name (info);

                        if (cardName.isEmpty())
                            cardName = cardId;

                        int device = -1;

                        snd_pcm_info_t* pcmInfo;
                        snd_pcm_info_alloca (&pcmInfo);

                        for (;;)
                        {
                            if (snd_ctl_pcm_next_device (handle, &device) < 0 || device < 0)
                                break;

                            snd_pcm_info_set_device (pcmInfo, (unsigned int) device);

                            for (unsigned int subDevice = 0, nbSubDevice = 1; subDevice < nbSubDevice; ++subDevice)
                            {
                                snd_pcm_info_set_subdevice (pcmInfo, subDevice);
                                snd_pcm_info_set_stream (pcmInfo, SND_PCM_STREAM_CAPTURE);
                                const bool isInput = (snd_ctl_pcm_info (handle, pcmInfo) >= 0);

                                snd_pcm_info_set_stream (pcmInfo, SND_PCM_STREAM_PLAYBACK);
                                const bool isOutput = (snd_ctl_pcm_info (handle, pcmInfo) >= 0);

                                if (! (isInput || isOutput))
                                    continue;

                                if (nbSubDevice == 1)
                                    nbSubDevice = snd_pcm_info_get_subdevices_count (pcmInfo);

                                String id, name;

                                if (nbSubDevice == 1)
                                {
                                    id << "hw:" << cardId << "," << device;
                                    name << cardName << ", " << snd_pcm_info_get_name (pcmInfo);
                                }
                                else
                                {
                                    id << "hw:" << cardId << "," << device << "," << (int) subDevice;
                                    name << cardName << ", " << snd_pcm_info_get_name (pcmInfo)
                                         << " {" <<  snd_pcm_info_get_subdevice_name (pcmInfo) << "}";
                                }

                                ALOE_ALSA_LOG ("Soundcard ID: " << id << ", name: '" << name
                                                << ", isInput:"  << (int) isInput
                                                << ", isOutput:" << (int) isOutput << "\n");

                                if (isInput)
                                {
                                    inputNames.add (name);
                                    inputIds.add (id);
                                }

                                if (isOutput)
                                {
                                    outputNames.add (name);
                                    outputIds.add (id);
                                }
                            }
                        }
                    }

                    ALOE_CHECKED_RESULT (snd_ctl_close (handle));
                }
            }
        */
    }

    /**
      | Enumerates all ALSA output devices
      | (as output by the command aplay -L) Does
      | not try to open the devices (with "testDevice"
      | for example), so that it also finds devices
      | that are busy and not yet available.
      |
      */
    pub fn enumerate_alsa_pcm_devices(&mut self)  {
        
        todo!();
        /*
            void** hints = nullptr;

            if (ALOE_CHECKED_RESULT (snd_device_name_hint (-1, "pcm", &hints)) == 0)
            {
                for (char** h = (char**) hints; *h; ++h)
                {
                    const String id (hintToString (*h, "NAME"));
                    const String description (hintToString (*h, "DESC"));
                    const String ioid (hintToString (*h, "IOID"));

                    ALOE_ALSA_LOG ("ID: " << id << "; desc: " << description << "; ioid: " << ioid);

                    String ss = id.fromFirstOccurrenceOf ("=", false, false)
                                  .upToFirstOccurrenceOf (",", false, false);

                    if (id.isEmpty()
                         || id.startsWith ("default:") || id.startsWith ("sysdefault:")
                         || id.startsWith ("plughw:") || id == "null")
                        continue;

                    String name (description.replace ("\n", "; "));

                    if (name.isEmpty())
                        name = id;

                    bool isOutput = (ioid != "Input");
                    bool isInput  = (ioid != "Output");

                    // alsa is stupid here, it advertises dmix and dsnoop as input/output devices, but
                    // opening dmix as input, or dsnoop as output will trigger errors..
                    isInput  = isInput  && ! id.startsWith ("dmix");
                    isOutput = isOutput && ! id.startsWith ("dsnoop");

                    if (isInput)
                    {
                        inputNames.add (name);
                        inputIds.add (id);
                    }

                    if (isOutput)
                    {
                        outputNames.add (name);
                        outputIds.add (id);
                    }
                }

                snd_device_name_free_hint (hints);
            }

            // sometimes the "default" device is not listed, but it is nice to see it explicitly in the list
            if (! outputIds.contains ("default"))
                testDevice ("default", "Default ALSA Output", "Default ALSA Input");

            // same for the pulseaudio plugin
            if (! outputIds.contains ("pulse"))
                testDevice ("pulse", "Pulseaudio output", "Pulseaudio input");

            // make sure the default device is listed first, and followed by the pulse device (if present)
            auto idx = outputIds.indexOf ("pulse");
            outputIds.move (idx, 0);
            outputNames.move (idx, 0);

            idx = inputIds.indexOf ("pulse");
            inputIds.move (idx, 0);
            inputNames.move (idx, 0);

            idx = outputIds.indexOf ("default");
            outputIds.move (idx, 0);
            outputNames.move (idx, 0);

            idx = inputIds.indexOf ("default");
            inputIds.move (idx, 0);
            inputNames.move (idx, 0);
        */
    }
    
    pub fn hint_to_string(
        hints: *const c_void,
        ty:    *const u8) -> String {
        
        todo!();
        /*
            char* hint = snd_device_name_get_hint (hints, type);
            auto s = String::fromUTF8 (hint);
            ::free (hint);
            return s;
        */
    }
}

