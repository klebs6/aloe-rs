crate::ix!();

#[cfg(all(MAC_OS_X_VERSION_10_15,MAC_OS_X_VERSION_MIN_REQUIRED_GTE_MAC_OS_X_VERSION_10_15))]
pub const ALOE_USE_NEW_APPLE_CAMERA_API: usize = 1;

#[cfg(not(all(MAC_OS_X_VERSION_10_15,MAC_OS_X_VERSION_MIN_REQUIRED_GTE_MAC_OS_X_VERSION_10_15)))]
pub const ALOE_USE_NEW_APPLE_CAMERA_API: usize = 0;

#[cfg(ALOE_USE_NEW_APPLE_CAMERA_API)]
pub struct PostCatalinaPhotoOutput {
    image_output: *mut AVCapturePhotoOutput, // default = nil
    delegate:     Box<NSObject,NSObjectDeleter>,
}

#[cfg(ALOE_USE_NEW_APPLE_CAMERA_API)]
impl Default for PostCatalinaPhotoOutput {
    
    fn default() -> Self {
        todo!();
        /*


            static PostCatalinaPhotoOutputDelegateClass cls;
                delegate.reset ([cls.createInstance() init]);
        */
    }
}

#[cfg(ALOE_USE_NEW_APPLE_CAMERA_API)]
impl PostCatalinaPhotoOutput {

    pub fn add_image_capture(&mut self, s: *mut AVCaptureSession)  {
        
        todo!();
        /*
            if (imageOutput != nil)
                    return;

                imageOutput = [[AVCapturePhotoOutput alloc] init];
                [s addOutput: imageOutput];
        */
    }
    
    pub fn remove_image_capture(&mut self, s: *mut AVCaptureSession)  {
        
        todo!();
        /*
            if (imageOutput == nil)
                    return;

                [s removeOutput: imageOutput];
                [imageOutput release];
                imageOutput = nil;
        */
    }
    
    pub fn get_connections(&self) -> *mut NSArray<*mut AVCaptureConnection> {
        
        todo!();
        /*
            if (imageOutput != nil)
                    return imageOutput.connections;

                return nil;
        */
    }
    
    pub fn trigger_image_capture(&mut self, p: &mut Pimpl)  {
        
        todo!();
        /*
            if (imageOutput == nil)
                    return;

                PostCatalinaPhotoOutputDelegateClass::setOwner (delegate.get(), &p);

                [imageOutput capturePhotoWithSettings: [AVCapturePhotoSettings photoSettings]
                                             delegate: id<AVCapturePhotoCaptureDelegate> (delegate.get())];
        */
    }
    
    pub fn get_available_devices() -> *mut NSArray {
        
        todo!();
        /*
            auto* discovery = [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes: @[AVCaptureDeviceTypeBuiltInWideAngleCamera,
                                                                                                      AVCaptureDeviceTypeExternalUnknown]
                                                                                         mediaType: AVMediaTypeVideo
                                                                                          position: AVCaptureDevicePositionUnspecified];
                return [discovery devices];
        */
    }
}
