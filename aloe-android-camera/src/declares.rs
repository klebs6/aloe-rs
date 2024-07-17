crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         STATICMETHOD (valueOf, "valueOf", "(Ljava/lang/String;)Landroid/graphics/Bitmap$CompressFormat;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidBitmapCompressFormat, 
    "android/graphics/Bitmap$CompressFormat", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (close,                "close",                "()V") 
         METHOD (createCaptureRequest, "createCaptureRequest", "(I)Landroid/hardware/camera2/CaptureRequest$Builder;") 
         METHOD (createCaptureSession, "createCaptureSession", "(Ljava/util/List;Landroid/hardware/camera2/CameraCaptureSession$StateCallback;Landroid/os/Handler;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidCameraDevice, 
    "android/hardware/camera2/CameraDevice", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (close,     "close",     "()V") 
         METHOD (getPlanes, "getPlanes", "()[Landroid/media/Image$Plane;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidImage, 
    "android/media/Image", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getBuffer, "getBuffer", "()Ljava/nio/ByteBuffer;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidImagePlane, 
    "android/media/Image$Plane", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (acquireLatestImage,          "acquireLatestImage",          "()Landroid/media/Image;") 
         METHOD (close,                       "close",                       "()V") 
         METHOD (getSurface,                  "getSurface",                  "()Landroid/view/Surface;") 
         METHOD (setOnImageAvailableListener, "setOnImageAvailableListener", "(Landroid/media/ImageReader$OnImageAvailableListener;Landroid/os/Handler;)V") 
         STATICMETHOD (newInstance, "newInstance", "(IIII)Landroid/media/ImageReader;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidImageReader, 
    "android/media/ImageReader", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor,             "<init>",                  "()V") 
         METHOD (getSurface,              "getSurface",              "()Landroid/view/Surface;") 
         METHOD (prepare,                 "prepare",                 "()V") 
         METHOD (release,                 "release",                 "()V") 
         METHOD (setAudioEncoder,         "setAudioEncoder",         "(I)V") 
         METHOD (setAudioSource,          "setAudioSource",          "(I)V") 
         METHOD (setOnErrorListener,      "setOnErrorListener",      "(Landroid/media/MediaRecorder$OnErrorListener;)V") 
         METHOD (setOnInfoListener,       "setOnInfoListener",       "(Landroid/media/MediaRecorder$OnInfoListener;)V") 
         METHOD (setOrientationHint,      "setOrientationHint",      "(I)V") 
         METHOD (setOutputFile,           "setOutputFile",           "(Ljava/lang/String;)V") 
         METHOD (setOutputFormat,         "setOutputFormat",         "(I)V") 
         METHOD (setVideoEncoder,         "setVideoEncoder",         "(I)V") 
         METHOD (setVideoEncodingBitRate, "setVideoEncodingBitRate", "(I)V") 
         METHOD (setVideoFrameRate,       "setVideoFrameRate",       "(I)V") 
         METHOD (setVideoSize,            "setVideoSize",            "(II)V") 
         METHOD (setVideoSource,          "setVideoSource",          "(I)V") 
         METHOD (start,                   "start",                   "()V") 
         METHOD (stop,                    "stop",                    "()V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidMediaRecorder, 
    "android/media/MediaRecorder", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor,               "<init>",                    "(Landroid/content/Context;)V") 
         METHOD (getSurfaceTexture,         "getSurfaceTexture",         "()Landroid/graphics/SurfaceTexture;") 
         METHOD (isAvailable,               "isAvailable",               "()Z") 
         METHOD (setSurfaceTextureListener, "setSurfaceTextureListener", "(Landroid/view/TextureView$SurfaceTextureListener;)V") 
         METHOD (setTransform,              "setTransform",              "(Landroid/graphics/Matrix;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidTextureView, 
    "android/view/TextureView", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor, "<init>", "(Landroid/graphics/SurfaceTexture;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidSurface, 
    "android/view/Surface", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (setDefaultBufferSize, "setDefaultBufferSize", "(II)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidSurfaceTexture, 
    "android/graphics/SurfaceTexture", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getOutputSizesForClass,      "getOutputSizes",       "(Ljava/lang/Class;)[Landroid/util/Size;") 
         METHOD (getOutputSizesForFormat,     "getOutputSizes",       "(I)[Landroid/util/Size;") 
         METHOD (isOutputSupportedFor,        "isOutputSupportedFor", "(I)Z") 
         METHOD (isOutputSupportedForSurface, "isOutputSupportedFor", "(Landroid/view/Surface;)Z")
        */
    }
}

declare_jni_class_with_min_sdk!{
    AndroidStreamConfigurationMap, 
    "android/hardware/camera2/params/StreamConfigurationMap", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (constructor, "<init>",      "()V") 
         METHOD (toByteArray, "toByteArray", "()[B") 
         METHOD (size,        "size",        "()I")
        */
    }
}

declare_jni_class_with_min_sdk!{
    ByteArrayOutputStream, 
    "java/io/ByteArrayOutputStream", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (abortCaptures,       "abortCaptures",       "()V") 
         METHOD (capture,             "capture",             "(Landroid/hardware/camera2/CaptureRequest;Landroid/hardware/camera2/CameraCaptureSession$CaptureCallback;Landroid/os/Handler;)I") 
         METHOD (close,               "close",               "()V") 
         METHOD (setRepeatingRequest, "setRepeatingRequest", "(Landroid/hardware/camera2/CaptureRequest;Landroid/hardware/camera2/CameraCaptureSession$CaptureCallback;Landroid/os/Handler;)I") 
         METHOD (stopRepeating,       "stopRepeating",       "()V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CameraCaptureSession, 
    "android/hardware/camera2/CameraCaptureSession", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (get,     "get",     "(Landroid/hardware/camera2/CameraCharacteristics$Key;)Ljava/lang/Object;") 
         METHOD (getKeys, "getKeys", "()Ljava/util/List;") 
         STATICFIELD (CONTROL_AF_AVAILABLE_MODES,      "CONTROL_AF_AVAILABLE_MODES",      "Landroid/hardware/camera2/CameraCharacteristics$Key;") 
         STATICFIELD (LENS_FACING,                     "LENS_FACING",                     "Landroid/hardware/camera2/CameraCharacteristics$Key;") 
         STATICFIELD (SCALER_STREAM_CONFIGURATION_MAP, "SCALER_STREAM_CONFIGURATION_MAP", "Landroid/hardware/camera2/CameraCharacteristics$Key;") 
         STATICFIELD (SENSOR_ORIENTATION,              "SENSOR_ORIENTATION",              "Landroid/hardware/camera2/CameraCharacteristics$Key;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CameraCharacteristics, 
    "android/hardware/camera2/CameraCharacteristics", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getName, "getName", "()Ljava/lang/String;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CameraCharacteristicsKey, 
    "android/hardware/camera2/CameraCharacteristics$Key", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (getCameraCharacteristics, "getCameraCharacteristics", "(Ljava/lang/String;)Landroid/hardware/camera2/CameraCharacteristics;") 
         METHOD (getCameraIdList,          "getCameraIdList",          "()[Ljava/lang/String;") 
         METHOD (openCamera,               "openCamera",               "(Ljava/lang/String;Landroid/hardware/camera2/CameraDevice$StateCallback;Landroid/os/Handler;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CameraManager, 
    "android/hardware/camera2/CameraManager", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         STATICFIELD (CONTROL_AE_PRECAPTURE_TRIGGER, "CONTROL_AE_PRECAPTURE_TRIGGER", "Landroid/hardware/camera2/CaptureRequest$Key;") 
         STATICFIELD (CONTROL_AF_MODE,               "CONTROL_AF_MODE",               "Landroid/hardware/camera2/CaptureRequest$Key;") 
         STATICFIELD (CONTROL_AF_TRIGGER,            "CONTROL_AF_TRIGGER",            "Landroid/hardware/camera2/CaptureRequest$Key;") 
         STATICFIELD (CONTROL_MODE,                  "CONTROL_MODE",                  "Landroid/hardware/camera2/CaptureRequest$Key;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CaptureRequest, 
    "android/hardware/camera2/CaptureRequest", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (addTarget, "addTarget", "(Landroid/view/Surface;)V") 
         METHOD (build,     "build",     "()Landroid/hardware/camera2/CaptureRequest;") 
         METHOD (set,       "set",       "(Landroid/hardware/camera2/CaptureRequest$Key;Ljava/lang/Object;)V")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CaptureRequestBuilder, 
    "android/hardware/camera2/CaptureRequest$Builder", 21
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
         METHOD (get, "get", "(Landroid/hardware/camera2/CaptureResult$Key;)Ljava/lang/Object;") 
         STATICFIELD (CONTROL_AE_STATE, "CONTROL_AE_STATE", "Landroid/hardware/camera2/CaptureResult$Key;") 
         STATICFIELD (CONTROL_AF_STATE, "CONTROL_AF_STATE", "Landroid/hardware/camera2/CaptureResult$Key;")
        */
    }
}

declare_jni_class_with_min_sdk!{
    CaptureResult, 
    "android/hardware/camera2/CaptureResult", 21
}
