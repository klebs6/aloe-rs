crate::ix!();

pub enum StillPictureTakerState
{
    idle = 0,
    pendingFocusLock,
    pendingExposurePrecapture,
    pendingExposurePostPrecapture,
    pictureTaken
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
                   METHOD (constructor, "<init>", "(JZ)V")              
                   CALLBACK (cameraCaptureSessionCaptureCompletedCallback,         "cameraCaptureSessionCaptureCompleted",         "(JZLandroid/hardware/camera2/CameraCaptureSession;Landroid/hardware/camera2/CaptureRequest;Landroid/hardware/camera2/TotalCaptureResult;)V") 
                   CALLBACK (cameraCaptureSessionCaptureFailedCallback,            "cameraCaptureSessionCaptureFailed",            "(JZLandroid/hardware/camera2/CameraCaptureSession;Landroid/hardware/camera2/CaptureRequest;Landroid/hardware/camera2/CaptureFailure;)V") 
                   CALLBACK (cameraCaptureSessionCaptureProgressedCallback,        "cameraCaptureSessionCaptureProgressed",        "(JZLandroid/hardware/camera2/CameraCaptureSession;Landroid/hardware/camera2/CaptureRequest;Landroid/hardware/camera2/CaptureResult;)V") 
                   CALLBACK (cameraCaptureSessionCaptureStartedCallback,           "cameraCaptureSessionCaptureStarted",           "(JZLandroid/hardware/camera2/CameraCaptureSession;Landroid/hardware/camera2/CaptureRequest;JJ)V") 
                   CALLBACK (cameraCaptureSessionCaptureSequenceAbortedCallback,   "cameraCaptureSessionCaptureSequenceAborted",   "(JZLandroid/hardware/camera2/CameraCaptureSession;I)V") 
                   CALLBACK (cameraCaptureSessionCaptureSequenceCompletedCallback, "cameraCaptureSessionCaptureSequenceCompleted", "(JZLandroid/hardware/camera2/CameraCaptureSession;IJ)V")
        */
    }
}

declare_jni_class_with_bytecode!{
    CameraCaptureSessionCaptureCallback, 
    "com/rmsl/aloe/CameraCaptureSessionCaptureCallback", 
    21, 
    CameraSupportByteCode, 
    sizeof(CameraSupportByteCode)
}
