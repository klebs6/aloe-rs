crate::ix!();

#[no_copy]
pub struct ViewerComponent<'a> {
    base: NSViewComponent<'a>,
}

impl<'a> Drop for ViewerComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            setView (nil);
        */
    }
}

impl<'a> ViewerComponent<'a> {

    pub fn new(device: &mut CameraDevice) -> Self {
    
        todo!();
        /*


            setView (device.impl->createVideoCapturePreview());
        */
    }
}

