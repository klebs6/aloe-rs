crate::ix!();

lazy_static!{
    /*
    DeviceOrientationChangeListener::OrientationEventListener_Class
    DeviceOrientationChangeListener::OrientationEventListener;
    */
}

pub const SENSOR_DELAYUI: i32 = 2;

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
                    METHOD (canDetectOrientation, "canDetectOrientation", "()Z") 
                    METHOD (constructor,          "<init>",               "(JLandroid/content/Context;I)V") 
                    METHOD (disable,              "disable",              "()V") 
                    METHOD (enable,               "enable",               "()V") 
                    CALLBACK (deviceOrientationChanged, "deviceOrientationChanged", "(JI)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    OrientationEventListener, 
    "com/rmsl/aloe/AloeOrientationEventListener", 21
}
