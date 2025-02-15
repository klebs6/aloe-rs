crate::ix!();

pub struct PostCatalinaPhotoOutputDelegateClass {
    base: ObjCClass<NSObject>,
}

impl Default for PostCatalinaPhotoOutputDelegateClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("PhotoOutputDelegateClass_")
                    addMethod (@selector (captureOutput:didFinishProcessingPhoto:error:), didFinishProcessingPhoto, "v@:@@@");
                    addIvar<CameraDeviceImpl*> ("owner");
                    registerClass();
        */
    }
}

impl PostCatalinaPhotoOutputDelegateClass {
    
    pub fn did_finish_processing_photo(
        self_: Id<Self>,
        _1:    Sel,
        _2:    *mut AVCapturePhotoOutput,
        photo: *mut AVCapturePhoto,
        error: *mut NSError)  {
        
        todo!();
        /*
            if (error != nil)
                    {
                        String errorString = error != nil ? nsStringToAloe (error.localizedDescription) : String();
                        ignoreUnused (errorString);

                        ALOE_CAMERA_LOG ("Still picture capture failed, error: " + errorString);
                        jassertfalse;

                        return;
                    }

                    auto* imageData = [photo fileDataRepresentation];
                    auto image = ImageFileFormat::loadFrom (imageData.bytes, (size_t) imageData.length);

                    getOwner (self).imageCaptureFinished (image);
        */
    }
    
    pub fn get_owner<'a>(self_: Id<Self>) -> &'a mut CameraDeviceImpl<'a> {
        
        todo!();
        /*
            return *getIvar<CameraDeviceImpl*> (self, "owner");
        */
    }
    
    pub fn set_owner<'a>(
        self_: Id<Self>,
        t:     *mut CameraDeviceImpl<'a>)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", t);
        */
    }
}
