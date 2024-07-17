crate::ix!();

pub struct OboeAudioIODeviceTypeDeviceInfo
{
    name:         String,
    id:           i32, // default = -1
    sample_rates: Vec<i32>,
    num_channels: i32,
}

#[no_copy]
#[leak_detector]
pub struct OboeAudioIODeviceType {
    base:           AudioIODeviceType,
    input_devices:  Vec<OboeAudioIODeviceTypeDeviceInfo>,
    output_devices: Vec<OboeAudioIODeviceTypeDeviceInfo>,
}

pub fn is_oboe_available() -> bool {
    
    todo!();
    /*
        return OboeAudioIODeviceType::isOboeAvailable();
    */
}

impl Default for OboeAudioIODeviceType {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioIODeviceType (OboeAudioIODevice::oboeTypeName)
            // Not using scanForDevices() to maintain behaviour backwards compatible with older APIs
            checkAvailableDevices()
        */
    }
}

impl OboeAudioIODeviceType {

    pub fn scan_for_devices(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_device_names(&self, want_input_names: bool) -> StringArray {
        
        todo!();
        /*
            StringArray names;

            for (auto& device : wantInputNames ? inputDevices : outputDevices)
                names.add (device.name);

            return names;
        */
    }
    
    pub fn get_default_device_index(&self, _0: bool) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_index_of_device(&self, 
        device:   *mut AudioIODevice,
        as_input: bool) -> i32 {
        
        todo!();
        /*
            if (auto oboeDevice = static_cast<OboeAudioIODevice*> (device))
            {
                auto oboeDeviceId = asInput ? oboeDevice->inputDeviceId
                                            : oboeDevice->outputDeviceId;

                auto& devices = asInput ? inputDevices : outputDevices;

                for (int i = 0; i < devices.size(); ++i)
                    if (devices.getReference (i).id == oboeDeviceId)
                        return i;
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
            auto outputDeviceInfo = getDeviceInfoForName (outputDeviceName, false);
            auto inputDeviceInfo  = getDeviceInfoForName (inputDeviceName, true);

            if (outputDeviceInfo.id < 0 && inputDeviceInfo.id < 0)
                return nullptr;

            auto& name = outputDeviceInfo.name.isNotEmpty() ? outputDeviceInfo.name
                                                            : inputDeviceInfo.name;

            return new OboeAudioIODevice (name,
                                          inputDeviceInfo.id, inputDeviceInfo.sampleRates,
                                          inputDeviceInfo.numChannels,
                                          outputDeviceInfo.id, outputDeviceInfo.sampleRates,
                                          outputDeviceInfo.numChannels);
        */
    }
    
    pub fn is_oboe_available() -> bool {
        
        todo!();
        /*
            #if ALOE_USE_ANDROID_OBOE
            return true;
           #else
            return false;
           #endif
        */
    }
    
    pub fn check_available_devices(&mut self)  {
        
        todo!();
        /*
            auto sampleRates = OboeAudioIODevice::getDefaultSampleRates();

            inputDevices .add ({ "System Default (Input)",  OboekUnspecified, sampleRates, 1 });
            outputDevices.add ({ "System Default (Output)", OboekUnspecified, sampleRates, 2 });

            if (! supportsDevicesInfo())
                return;

            auto* env = getEnv();

            jclass audioManagerClass = env->FindClass ("android/media/AudioManager");

            // We should be really entering here only if API supports it.
            jassert (audioManagerClass != nullptr);

            if (audioManagerClass == nullptr)
                return;

            auto audioManager = LocalRef<jobject> (env->CallObjectMethod (getAppContext().get(),
                                                                          AndroidContext.getSystemService,
                                                                          javaString ("audio").get()));

            static jmethodID getDevicesMethod = env->GetMethodID (audioManagerClass, "getDevices",
                                                                  "(I)[Landroid/media/AudioDeviceInfo;");

            static constexpr int allDevices = 3;
            auto devices = LocalRef<jobjectArray> ((jobjectArray) env->CallObjectMethod (audioManager,
                                                                                         getDevicesMethod,
                                                                                         allDevices));

            const int numDevices = env->GetArrayLength (devices.get());

            for (int i = 0; i < numDevices; ++i)
            {
                auto device = LocalRef<jobject> ((jobject) env->GetObjectArrayElement (devices.get(), i));
                addDevice (device, env);
            }

            ALOE_OBOE_LOG ("-----InputDevices:");

            for (auto& device : inputDevices)
            {
                ignoreUnused (device);

                ALOE_OBOE_LOG ("name = " << device.name);
                ALOE_OBOE_LOG ("id = " << String (device.id));
                ALOE_OBOE_LOG ("sample rates size = " << String (device.sampleRates.size()));
                ALOE_OBOE_LOG ("num channels = " + String (device.numChannels));
            }

            ALOE_OBOE_LOG ("-----OutputDevices:");

            for (auto& device : outputDevices)
            {
                ignoreUnused (device);

                ALOE_OBOE_LOG ("name = " << device.name);
                ALOE_OBOE_LOG ("id = " << String (device.id));
                ALOE_OBOE_LOG ("sample rates size = " << String (device.sampleRates.size()));
                ALOE_OBOE_LOG ("num channels = " + String (device.numChannels));
            }
        */
    }
    
    pub fn supports_devices_info(&self) -> bool {
        
        todo!();
        /*
            static auto result = getAndroidSDKVersion() >= 23;
            return result;
        */
    }
    
    pub fn add_device(&mut self, 
        device: &LocalRef<jobject>,
        env:    *mut JNIEnv)  {
        
        todo!();
        /*
            auto deviceClass = LocalRef<jclass> ((jclass) env->FindClass ("android/media/AudioDeviceInfo"));

            jmethodID getProductNameMethod = env->GetMethodID (deviceClass, "getProductName",
                                                               "()Ljava/lang/CharSequence;");

            jmethodID getTypeMethod          = env->GetMethodID (deviceClass, "getType", "()I");
            jmethodID getIdMethod            = env->GetMethodID (deviceClass, "getId", "()I");
            jmethodID getSampleRatesMethod   = env->GetMethodID (deviceClass, "getSampleRates", "()[I");
            jmethodID getChannelCountsMethod = env->GetMethodID (deviceClass, "getChannelCounts", "()[I");
            jmethodID isSourceMethod         = env->GetMethodID (deviceClass, "isSource", "()Z");

            auto deviceTypeString = deviceTypeToString (env->CallIntMethod (device, getTypeMethod));

            if (deviceTypeString.isEmpty()) // unknown device
                return;

            auto name = aloeString ((jstring) env->CallObjectMethod (device, getProductNameMethod)) + " " + deviceTypeString;
            auto id = env->CallIntMethod (device, getIdMethod);

            auto jSampleRates = LocalRef<jintArray> ((jintArray) env->CallObjectMethod (device, getSampleRatesMethod));
            auto sampleRates = jintArrayToAloeArray (jSampleRates);

            auto jChannelCounts = LocalRef<jintArray> ((jintArray) env->CallObjectMethod (device, getChannelCountsMethod));
            auto channelCounts = jintArrayToAloeArray (jChannelCounts);
            int numChannels = channelCounts.isEmpty() ? -1 : channelCounts.getLast();

            auto isInput  = env->CallBooleanMethod (device, isSourceMethod);
            auto& devices = isInput ? inputDevices : outputDevices;

            devices.add ({ name, id, sampleRates, numChannels });
        */
    }
    
    pub fn device_type_to_string(ty: i32) -> String {
        
        todo!();
        /*
            switch (type)
            {
                case 0:   return {};
                case 1:   return "built-in earphone speaker";
                case 2:   return "built-in speaker";
                case 3:   return "wired headset";
                case 4:   return "wired headphones";
                case 5:   return "line analog";
                case 6:   return "line digital";
                case 7:   return "Bluetooth device typically used for telephony";
                case 8:   return "Bluetooth device supporting the A2DP profile";
                case 9:   return "HDMI";
                case 10:  return "HDMI audio return channel";
                case 11:  return "USB device";
                case 12:  return "USB accessory";
                case 13:  return "DOCK";
                case 14:  return "FM";
                case 15:  return "built-in microphone";
                case 16:  return "FM tuner";
                case 17:  return "TV tuner";
                case 18:  return "telephony";
                case 19:  return "auxiliary line-level connectors";
                case 20:  return "IP";
                case 21:  return "BUS";
                case 22:  return "USB headset";
                case 23:  return "hearing aid";
                case 24:  return "built-in speaker safe";
                case 25:  return {};
                default:  jassertfalse; return {}; // type not supported yet, needs to be added!
            }
        */
    }
    
    pub fn jint_array_to_aloe_array(j_array: &LocalRef<jintArray>) -> Vec<i32> {
        
        todo!();
        /*
            auto* env = getEnv();

            jint* jArrayElems = env->GetIntArrayElements (jArray, nullptr);
            int numElems = env->GetArrayLength (jArray);

            Vec<int> aloeArray;

            for (int s = 0; s < numElems; ++s)
                aloeArray.add (jArrayElems[s]);

            env->ReleaseIntArrayElements (jArray, jArrayElems, 0);
            return aloeArray;
        */
    }
    
    pub fn get_device_info_for_name(&mut self, 
        name:     &String,
        is_input: bool) -> OboeAudioIODeviceTypeDeviceInfo {
        
        todo!();
        /*
            if (name.isNotEmpty())
            {
                for (auto& device : isInput ? inputDevices : outputDevices)
                {
                    if (device.name == name)
                        return device;
                }
            }

            return {};
        */
    }
}
