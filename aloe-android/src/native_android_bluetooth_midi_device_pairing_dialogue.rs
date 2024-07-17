crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/native/aloe_android_BluetoothMidiDevicePairingDialogue.cpp]

pub enum AndroidBluetoothMidiInterfacePairStatus
{
    unpaired = 0,
    paired = 1,
    pairing = 2
}

lazy_static!{
    /*
    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     STATICMETHOD (getAndroidBluetoothManager, "getAndroidBluetoothManager", "(Landroid/content/Context;)Lcom/rmsl/aloe/AloeMidiSupport$BluetoothManager;")

    DECLARE_JNI_CLASS_WITH_MIN_SDK (AndroidAloeMidiSupport, "com/rmsl/aloe/AloeMidiSupport", 23)
    #undef JNI_CLASS_MEMBERS

    #define JNI_CLASS_MEMBERS(METHOD, STATICMETHOD, FIELD, STATICFIELD, CALLBACK) \
     METHOD (getMidiBluetoothAddresses, "getMidiBluetoothAddresses", "()[Ljava/lang/String;") \
     METHOD (pairBluetoothMidiDevice, "pairBluetoothMidiDevice", "(Ljava/lang/String;)Z") \
     METHOD (unpairBluetoothMidiDevice, "unpairBluetoothMidiDevice", "(Ljava/lang/String;)V") \
     METHOD (getHumanReadableStringForBluetoothAddress, "getHumanReadableStringForBluetoothAddress", "(Ljava/lang/String;)Ljava/lang/String;") \
     METHOD (getBluetoothDeviceStatus, "getBluetoothDeviceStatus", "(Ljava/lang/String;)I") \
     METHOD (startStopScan, "startStopScan", "(Z)V")

    DECLARE_JNI_CLASS_WITH_MIN_SDK (AndroidBluetoothManager, "com/rmsl/aloe/AloeMidiSupport$BluetoothManager", 23)
    #undef JNI_CLASS_MEMBERS
    */
}

