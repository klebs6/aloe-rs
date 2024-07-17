crate::ix!();

pub struct PhotoOutputDelegateClass {
    base: ObjCClass<NSObject>,
}

impl Default for PhotoOutputDelegateClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject> ("PhotoOutputDelegateClass_")
                        addMethod (@selector (captureOutput:willBeginCaptureForResolvedSettings:),       willBeginCaptureForSettings, "v@:@@");
                        addMethod (@selector (captureOutput:willCapturePhotoForResolvedSettings:),       willCaptureForSettings,      "v@:@@");
                        addMethod (@selector (captureOutput:didCapturePhotoForResolvedSettings:),        didCaptureForSettings,       "v@:@@");
                        addMethod (@selector (captureOutput:didFinishCaptureForResolvedSettings:error:), didFinishCaptureForSettings, "v@:@@@");

                        if (CameraDevicePimpl::getIOSVersion().major >= 11)
                            addMethod (@selector (captureOutput:didFinishProcessingPhoto:error:), didFinishProcessingPhoto, "v@:@@@");
                        else
                            addMethod (@selector (captureOutput:didFinishProcessingPhotoSampleBuffer:previewPhotoSampleBuffer:resolvedSettings:bracketSettings:error:), didFinishProcessingPhotoSampleBuffer, "v@:@@@@@@");

                        addIvar<StillPictureTaker*> ("owner");

                        registerClass();
        */
    }
}

impl PhotoOutputDelegateClass {
    
    pub fn get_owner<'a>(self_: Id<NSObject>) -> &'a mut StillPictureTaker<'a> {
        
        todo!();
        /*
            return *getIvar<StillPictureTaker*> (self, "owner");
        */
    }
    
    pub fn set_owner(
        self_: Id<NSObject>,
        t:     *mut StillPictureTaker)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "owner", t);
        */
    }
    
    pub fn will_begin_capture_for_settings(
        _0: Id<NSObject>,
        _1: Sel,
        _2: *mut AVCapturePhotoOutput,
        _3: *mut AVCaptureResolvedPhotoSettings)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("willBeginCaptureForSettings()");
        */
    }
    
    pub fn will_capture_for_settings(
        _0: Id<NSObject>,
        _1: Sel,
        _2: *mut AVCapturePhotoOutput,
        _3: *mut AVCaptureResolvedPhotoSettings)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("willCaptureForSettings()");
        */
    }
    
    pub fn did_capture_for_settings(
        _0: Id<NSObject>,
        _1: Sel,
        _2: *mut AVCapturePhotoOutput,
        _3: *mut AVCaptureResolvedPhotoSettings)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("didCaptureForSettings()");
        */
    }
    
    pub fn did_finish_capture_for_settings(
        _0:    Id<NSObject>,
        _1:    Sel,
        _2:    *mut AVCapturePhotoOutput,
        _3:    *mut AVCaptureResolvedPhotoSettings,
        error: *mut NSError)  {
        
        todo!();
        /*
            String errorString = error != nil ? nsStringToAloe (error.localizedDescription) : String();
                        ignoreUnused (errorString);

                        ALOE_CAMERA_LOG ("didFinishCaptureForSettings(), error = " + errorString);
        */
    }
    
    pub fn did_finish_processing_photo(
        self_:         Id<NSObject>,
        _1:            Sel,
        _2:            *mut AVCapturePhotoOutput,
        capture_photo: *mut AVCapturePhoto,
        error:         *mut NSError)  {
        
        todo!();
        /*
            getOwner (self).takingPicture = false;

                        String errorString = error != nil ? nsStringToAloe (error.localizedDescription) : String();
                        ignoreUnused (errorString);

                        ALOE_CAMERA_LOG ("didFinishProcessingPhoto(), error = " + errorString);

                        if (error != nil)
                        {
                            ALOE_CAMERA_LOG ("Still picture capture failed, error: " + nsStringToAloe (error.localizedDescription));
                            jassertfalse;
                            return;
                        }

                        auto* imageOrientation = (NSNumber *) capturePhoto.metadata[(NSString*) kCGImagePropertyOrientation];

                        auto* uiImage = getImageWithCorrectOrientation ((CGImagePropertyOrientation) imageOrientation.unsignedIntValue,
                                                                        [capturePhoto CGImageRepresentation]);

                        auto* imageData = UIImageJPEGRepresentation (uiImage, 0.f);

                        auto image = ImageFileFormat::loadFrom (imageData.bytes, (size_t) imageData.length);

                        getOwner (self).callListeners (image);

                        MessageManager::callAsync ([self, image]() { getOwner (self).notifyPictureTaken (image); });
        */
    }
    
    pub fn get_image_with_correct_orientation(
        image_orientation: CGImagePropertyOrientation,
        image_data:        CGImageRef) -> *mut UIImage {
        
        todo!();
        /*
            auto origWidth  = CGImageGetWidth (imageData);
                        auto origHeight = CGImageGetHeight (imageData);

                        auto targetSize = getTargetImageDimensionFor (imageOrientation, imageData);

                        UIGraphicsBeginImageContext (targetSize);
                        CGContextRef context = UIGraphicsGetCurrentContext();

                        switch (imageOrientation)
                        {
                            case kCGImagePropertyOrientationUp:
                                CGContextScaleCTM (context, 1.0, -1.0);
                                CGContextTranslateCTM (context, 0.0, -targetSize.height);
                                break;
                            case kCGImagePropertyOrientationRight:
                                CGContextRotateCTM (context, 90 * MathConstants<CGFloat>::pi / 180);
                                CGContextScaleCTM (context, targetSize.height / origHeight, -targetSize.width / origWidth);
                                break;
                            case kCGImagePropertyOrientationDown:
                                CGContextTranslateCTM (context, targetSize.width, 0.0);
                                CGContextScaleCTM (context, -1.0, 1.0);
                                break;
                            case kCGImagePropertyOrientationLeft:
                                CGContextRotateCTM (context, -90 * MathConstants<CGFloat>::pi / 180);
                                CGContextScaleCTM (context, targetSize.height / origHeight, -targetSize.width / origWidth);
                                CGContextTranslateCTM (context, -targetSize.width, -targetSize.height);
                                break;
                            case kCGImagePropertyOrientationUpMirrored:
                            case kCGImagePropertyOrientationDownMirrored:
                            case kCGImagePropertyOrientationLeftMirrored:
                            case kCGImagePropertyOrientationRightMirrored:
                            default:
                                // Not implemented.
                                jassertfalse;
                                break;
                        }

                        CGContextDrawImage (context, CGRectMake (0, 0, targetSize.width, targetSize.height), imageData);

                        UIImage* correctedImage = UIGraphicsGetImageFromCurrentImageContext();
                        UIGraphicsEndImageContext();

                        return correctedImage;
        */
    }
    
    pub fn get_target_image_dimension_for(
        image_orientation: CGImagePropertyOrientation,
        image_data:        CGImageRef) -> CGSize {
        
        todo!();
        /*
            auto width = CGImageGetWidth (imageData);
                        auto height = CGImageGetHeight (imageData);

                        switch (imageOrientation)
                        {
                            case kCGImagePropertyOrientationUp:
                            case kCGImagePropertyOrientationUpMirrored:
                            case kCGImagePropertyOrientationDown:
                            case kCGImagePropertyOrientationDownMirrored:
                                return CGSizeMake ((CGFloat) width, (CGFloat) height);
                            case kCGImagePropertyOrientationRight:
                            case kCGImagePropertyOrientationRightMirrored:
                            case kCGImagePropertyOrientationLeft:
                            case kCGImagePropertyOrientationLeftMirrored:
                                return CGSizeMake ((CGFloat) height, (CGFloat) width);
                        }

                        jassertfalse;
                        return CGSizeMake ((CGFloat) width, (CGFloat) height);
        */
    }
    
    pub fn did_finish_processing_photo_sample_buffer(
        self_:                Id<NSObject>,
        _1:                   Sel,
        _2:                   *mut AVCapturePhotoOutput,
        image_buffer:         CMSampleBufferRef,
        image_preview_buffer: CMSampleBufferRef,
        _5:                   *mut AVCaptureResolvedPhotoSettings,
        _6:                   *mut AVCaptureBracketedStillImageSettings,
        error:                *mut NSError)  {
        
        todo!();
        /*
            getOwner (self).takingPicture = false;

                        String errorString = error != nil ? nsStringToAloe (error.localizedDescription) : String();
                        ignoreUnused (errorString);

                        ALOE_CAMERA_LOG ("didFinishProcessingPhotoSampleBuffer(), error = " + errorString);

                        if (error != nil)
                        {
                            ALOE_CAMERA_LOG ("Still picture capture failed, error: " + nsStringToAloe (error.localizedDescription));
                            jassertfalse;
                            return;
                        }

                        NSData* origImageData = [AVCapturePhotoOutput JPEGPhotoDataRepresentationForJPEGSampleBuffer: imageBuffer previewPhotoSampleBuffer: imagePreviewBuffer];
                        auto origImage = [UIImage imageWithData: origImageData];
                        auto imageOrientation = uiImageOrientationToCGImageOrientation (origImage.imageOrientation);

                        auto* uiImage = getImageWithCorrectOrientation (imageOrientation, origImage.CGImage);

                        auto* imageData = UIImageJPEGRepresentation (uiImage, 0.f);

                        auto image = ImageFileFormat::loadFrom (imageData.bytes, (size_t) imageData.length);

                        getOwner (self).callListeners (image);

                        MessageManager::callAsync ([self, image]() { getOwner (self).notifyPictureTaken (image); });
        */
    }
    
    pub fn ui_image_orientation_to_cg_image_orientation(orientation: UIImageOrientation) -> CGImagePropertyOrientation {
        
        todo!();
        /*
            switch (orientation)
                        {
                            case UIImageOrientationUp:            return kCGImagePropertyOrientationUp;
                            case UIImageOrientationDown:          return kCGImagePropertyOrientationDown;
                            case UIImageOrientationLeft:          return kCGImagePropertyOrientationLeft;
                            case UIImageOrientationRight:         return kCGImagePropertyOrientationRight;
                            case UIImageOrientationUpMirrored:    return kCGImagePropertyOrientationUpMirrored;
                            case UIImageOrientationDownMirrored:  return kCGImagePropertyOrientationDownMirrored;
                            case UIImageOrientationLeftMirrored:  return kCGImagePropertyOrientationLeftMirrored;
                            case UIImageOrientationRightMirrored: return kCGImagePropertyOrientationRightMirrored;
                        }
        */
    }
}
