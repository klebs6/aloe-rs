crate::ix!();

pub struct ViewerComponent<'a> {
    base: UIViewComponent<'a>,
}

impl<'a> ViewerComponent<'a> {

    pub fn new(device: &mut CameraDevice) -> Self {
    
        todo!();
        /*


            static AloeCameraDeviceViewerClass cls;

            // Initial size that can be overriden later.
            setSize (640, 480);

            auto view = [cls.createInstance() init];
            setView (view);

            auto* previewLayer = device.impl->captureSession.createPreviewLayer();
            previewLayer.frame = view.bounds;

            UIInterfaceOrientation statusBarOrientation = [UIApplication sharedApplication].statusBarOrientation;
            AVCaptureVideoOrientation videoOrientation = statusBarOrientation != UIInterfaceOrientationUnknown
                                                       ? (AVCaptureVideoOrientation) statusBarOrientation
                                                       : AVCaptureVideoOrientationPortrait;

            previewLayer.connection.videoOrientation = videoOrientation;

            [view.layer addSublayer: previewLayer];
        */
    }
}
