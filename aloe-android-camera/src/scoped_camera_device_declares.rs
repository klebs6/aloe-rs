crate::ix!();

lazy_static!{
    /*
    CameraDevice::CameraDeviceImpl::ScopedCameraDevice::CaptureSession::StillPictureTaker::CameraCaptureSessionCaptureCallback_Class
    CameraDevice::CameraDeviceImpl::ScopedCameraDevice::CaptureSession::StillPictureTaker::CameraCaptureSessionCaptureCallback;

    CameraDevice::CameraDeviceImpl::ScopedCameraDevice::CameraDeviceStateCallback_Class
    CameraDevice::CameraDeviceImpl::ScopedCameraDevice::CameraDeviceStateCallback;

    CameraDevice::CameraDeviceImpl::ScopedCameraDevice::CaptureSession::CameraCaptureSessionStateCallback_Class
    CameraDevice::CameraDeviceImpl::ScopedCameraDevice::CaptureSession::CameraCaptureSessionStateCallback;
    */
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
                    METHOD (constructor, "<init>", "(J)V") 
                    CALLBACK (cameraDeviceStateClosedCallback,       "cameraDeviceStateClosed",       "(JLandroid/hardware/camera2/CameraDevice;)V")  
                    CALLBACK (cameraDeviceStateDisconnectedCallback, "cameraDeviceStateDisconnected", "(JLandroid/hardware/camera2/CameraDevice;)V")  
                    CALLBACK (cameraDeviceStateErrorCallback,        "cameraDeviceStateError",        "(JLandroid/hardware/camera2/CameraDevice;I)V") 
                    CALLBACK (cameraDeviceStateOpenedCallback,       "cameraDeviceStateOpened",       "(JLandroid/hardware/camera2/CameraDevice;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CameraDeviceStateCallback, 
    "com/rmsl/aloe/CameraDeviceStateCallback", 21
}
