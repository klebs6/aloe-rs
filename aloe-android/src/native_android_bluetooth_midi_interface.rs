crate::ix!();

pub struct AndroidBluetoothMidiInterface {

}

impl AndroidBluetoothMidiInterface {

    pub fn start_stop_scan(start_scanning: bool)  {
        
        todo!();
        /*
            JNIEnv* env = getEnv();
            LocalRef<jobject> btManager (env->CallStaticObjectMethod (AndroidAloeMidiSupport, AndroidAloeMidiSupport.getAndroidBluetoothManager, getAppContext().get()));

            if (btManager.get() != nullptr)
                env->CallVoidMethod (btManager.get(), AndroidBluetoothManager.startStopScan, (jboolean) (startScanning ? 1 : 0));
        */
    }
    
    pub fn get_bluetooth_midi_devices_nearby() -> StringArray {
        
        todo!();
        /*
            StringArray retval;

            JNIEnv* env = getEnv();

            LocalRef<jobject> btManager (env->CallStaticObjectMethod (AndroidAloeMidiSupport, AndroidAloeMidiSupport.getAndroidBluetoothManager, getAppContext().get()));

            // if this is null then bluetooth is not enabled
            if (btManager.get() == nullptr)
                return {};

            jobjectArray jDevices = (jobjectArray) env->CallObjectMethod (btManager.get(),
                                                                          AndroidBluetoothManager.getMidiBluetoothAddresses);
            LocalRef<jobjectArray> devices (jDevices);

            const int count = env->GetArrayLength (devices.get());

            for (int i = 0; i < count; ++i)
            {
                LocalRef<jstring> string ((jstring)  env->GetObjectArrayElement (devices.get(), i));
                retval.add (aloeString (string));
            }

            return retval;
        */
    }
    
    pub fn pair_bluetooth_midi_device(bluetooth_address: &String) -> bool {
        
        todo!();
        /*
            JNIEnv* env = getEnv();

            LocalRef<jobject> btManager (env->CallStaticObjectMethod (AndroidAloeMidiSupport, AndroidAloeMidiSupport.getAndroidBluetoothManager, getAppContext().get()));
            if (btManager.get() == nullptr)
                return false;

            jboolean result = env->CallBooleanMethod (btManager.get(), AndroidBluetoothManager.pairBluetoothMidiDevice,
                                                      javaString (bluetoothAddress).get());

            return result;
        */
    }
    
    pub fn unpair_bluetooth_midi_device(bluetooth_address: &String)  {
        
        todo!();
        /*
            JNIEnv* env = getEnv();

            LocalRef<jobject> btManager (env->CallStaticObjectMethod (AndroidAloeMidiSupport, AndroidAloeMidiSupport.getAndroidBluetoothManager, getAppContext().get()));

            if (btManager.get() != nullptr)
                env->CallVoidMethod (btManager.get(), AndroidBluetoothManager.unpairBluetoothMidiDevice,
                                     javaString (bluetoothAddress).get());
        */
    }
    
    pub fn get_human_readable_string_for_bluetooth_address(address: &String) -> String {
        
        todo!();
        /*
            JNIEnv* env = getEnv();

            LocalRef<jobject> btManager (env->CallStaticObjectMethod (AndroidAloeMidiSupport, AndroidAloeMidiSupport.getAndroidBluetoothManager, getAppContext().get()));

            if (btManager.get() == nullptr)
                return address;

            LocalRef<jstring> string ((jstring) env->CallObjectMethod (btManager.get(),
                                                                       AndroidBluetoothManager.getHumanReadableStringForBluetoothAddress,
                                                                       javaString (address).get()));

            if (string.get() == nullptr)
                return address;

            return aloeString (string);
        */
    }
    
    pub fn is_bluetooth_device_paired(address: &String) -> AndroidBluetoothMidiInterfacePairStatus {
        
        todo!();
        /*
            JNIEnv* env = getEnv();

            LocalRef<jobject> btManager (env->CallStaticObjectMethod (AndroidAloeMidiSupport, AndroidAloeMidiSupport.getAndroidBluetoothManager, getAppContext().get()));

            if (btManager.get() == nullptr)
                return unpaired;

            return static_cast<AndroidBluetoothMidiInterfacePairStatus> (env->CallIntMethod (btManager.get(), AndroidBluetoothManager.getBluetoothDeviceStatus,
                                                                javaString (address).get()));
        */
    }
}
