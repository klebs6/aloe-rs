crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         CALLBACK (MidiInput::Pimpl::handleReceive, "handleReceive", "(J[BIIJ)V" )
        */
    }
}

declare_jni_class_with_min_sdk!{
    AloeMidiInputPort, "com/rmsl/aloe/AloeMidiSupport$AloeMidiInputPort", 23
}

//-------------------------------
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         STATICMETHOD (getAndroidMidiDeviceManager, "getAndroidMidiDeviceManager", "(Landroid/content/Context;)Lcom/rmsl/aloe/AloeMidiSupport$MidiDeviceManager;") 
         STATICMETHOD (getAndroidBluetoothManager,  "getAndroidBluetoothManager",  "(Landroid/content/Context;)Lcom/rmsl/aloe/AloeMidiSupport$BluetoothManager;")
        */
    }
}

declare_jni_class_with_bytecode!{
    AloeMidiSupport, 
    "com/rmsl/aloe/AloeMidiSupport", 
    23, 
    javaMidiByteCode, 
    sizeof (javaMidiByteCode)
}

//-------------------------------
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getAloeAndroidMidiInputDeviceNameAndIDs,  "getAloeAndroidMidiInputDeviceNameAndIDs",  "()[Ljava/lang/String;") 
         METHOD (getAloeAndroidMidiOutputDeviceNameAndIDs, "getAloeAndroidMidiOutputDeviceNameAndIDs", "()[Ljava/lang/String;") 
         METHOD (openMidiInputPortWithID,                  "openMidiInputPortWithID",                  "(IJ)Lcom/rmsl/aloe/AloeMidiSupport$AloeMidiPort;") 
         METHOD (openMidiOutputPortWithID,                 "openMidiOutputPortWithID",                 "(I)Lcom/rmsl/aloe/AloeMidiSupport$AloeMidiPort;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    MidiDeviceManager, 
    "com/rmsl/aloe/AloeMidiSupport$MidiDeviceManager", 
    23
}

//-------------------------------
macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (start,    "start",    "()V") 
         METHOD (stop,     "stop",     "()V") 
         METHOD (close,    "close",    "()V") 
         METHOD (sendMidi, "sendMidi", "([BII)V") 
         METHOD (getName,  "getName",  "()Ljava/lang/String;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AloeMidiPort, 
    "com/rmsl/aloe/AloeMidiSupport$AloeMidiPort", 
    23
}
