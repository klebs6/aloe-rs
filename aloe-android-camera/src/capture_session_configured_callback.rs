crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
                        METHOD (constructor, "<init>", "(J)V") 
                        CALLBACK(cameraCaptureSessionActiveCallback,          "cameraCaptureSessionActive",          "(JLandroid/hardware/camera2/CameraCaptureSession;)V") 
                        CALLBACK(cameraCaptureSessionClosedCallback,          "cameraCaptureSessionClosed",          "(JLandroid/hardware/camera2/CameraCaptureSession;)V") 
                        CALLBACK(cameraCaptureSessionConfigureFailedCallback, "cameraCaptureSessionConfigureFailed", "(JLandroid/hardware/camera2/CameraCaptureSession;)V") 
                        CALLBACK(cameraCaptureSessionConfiguredCallback,      "cameraCaptureSessionConfigured",      "(JLandroid/hardware/camera2/CameraCaptureSession;)V") 
                        CALLBACK(cameraCaptureSessionReadyCallback,           "cameraCaptureSessionReady",           "(JLandroid/hardware/camera2/CameraCaptureSession;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CameraCaptureSessionStateCallback, 
    "com/rmsl/aloe/CameraCaptureSessionStateCallback", 21
}
    
pub trait CaptureSessionConfiguredCallback
{
    fn capture_session_configured(&mut self, _0: *mut CaptureSession);
}

