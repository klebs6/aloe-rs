crate::ix!();

pub struct AloeCameraDeviceViewerClass {
    base: ObjCClass<UIView>,
}

pub struct UIView {}

impl Default for AloeCameraDeviceViewerClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<UIView> ("AloeCameraDeviceViewerClass_")
                addMethod (@selector (layoutSubviews), layoutSubviews, "v@:");

                registerClass();
        */
    }
}

impl AloeCameraDeviceViewerClass {

    pub fn layout_subviews(
        self_: Id<NSObject>,
        _1:    Sel)  {
        
        todo!();
        /*
            sendSuperclassMessage<void> (self, @selector (layoutSubviews));

                UIView* asUIView = (UIView*) self;

                updateOrientation (self);

                if (auto* previewLayer = getPreviewLayer (self))
                    previewLayer.frame = asUIView.bounds;
        */
    }
    
    pub fn get_preview_layer(self_: Id<NSObject>) -> *mut AVCaptureVideoPreviewLayer {
        
        todo!();
        /*
            UIView* asUIView = (UIView*) self;

                if (asUIView.layer.sublayers != nil && [asUIView.layer.sublayers count] > 0)
                    if ([asUIView.layer.sublayers[0] isKindOfClass: [AVCaptureVideoPreviewLayer class]])
                         return (AVCaptureVideoPreviewLayer*) asUIView.layer.sublayers[0];

                return nil;
        */
    }
    
    pub fn update_orientation(self_: Id<NSObject>)  {
        
        todo!();
        /*
            if (auto* previewLayer = getPreviewLayer (self))
                {
                    UIDeviceOrientation o = [UIDevice currentDevice].orientation;

                    if (UIDeviceOrientationIsPortrait (o) || UIDeviceOrientationIsLandscape (o))
                    {
                        if (previewLayer.connection != nil)
                            previewLayer.connection.videoOrientation = (AVCaptureVideoOrientation) o;
                    }
                }
        */
    }
}
