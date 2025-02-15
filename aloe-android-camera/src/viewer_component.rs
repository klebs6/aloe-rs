crate::ix!();

#[no_copy]
pub struct ViewerComponent<'a> {
    base:                Component<'a>,
    base2:               ComponentMovementWatcher<'a>,
    viewer_component:    AndroidViewComponent<'a>,
    target_aspect_ratio: f32, // default = 1.0f
}

impl<'a> ViewerComponent<'a> {

    pub fn new(device: &mut CameraDevice) -> Self {
    
        todo!();
        /*
        : component_movement_watcher(this),

            auto previewSize = device.impl->streamConfigurationMap.getDefaultPreviewSize();

            targetAspectRatio = (float) previewSize.getWidth() / (float) previewSize.getHeight();

            if (isOrientationLandscape())
                setBounds (previewSize);
            else
                setBounds (0, 0, previewSize.getHeight(), previewSize.getWidth());

            addAndMakeVisible (viewerComponent);
            viewerComponent.setView (device.impl->previewDisplay.getNativeView());
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        _0: bool,
        _1: bool)  {
        
        todo!();
        /*
            auto b = getLocalBounds().toFloat();

            auto targetWidth  = b.getWidth();
            auto targetHeight = b.getHeight();

            if (isOrientationLandscape())
            {
                auto currentAspectRatio = b.getWidth() / b.getHeight();

                if (currentAspectRatio > targetAspectRatio)
                    targetWidth = targetWidth * targetAspectRatio / currentAspectRatio;
                else
                    targetHeight = targetHeight * currentAspectRatio / targetAspectRatio;
            }
            else
            {
                auto currentAspectRatio = b.getHeight() / b.getWidth();

                if (currentAspectRatio > targetAspectRatio)
                    targetHeight = targetHeight * targetAspectRatio / currentAspectRatio;
                else
                    targetWidth = targetWidth * currentAspectRatio / targetAspectRatio;
            }

            viewerComponent.setBounds (Rectangle<float> (targetWidth, targetHeight).withCentre (b.getCentre()).toNearestInt());
        */
    }
    
    pub fn is_orientation_landscape(&self) -> bool {
        
        todo!();
        /*
            auto o = Desktop::getInstance().getCurrentOrientation();
            return o == Desktop::rotatedClockwise || o == Desktop::rotatedAntiClockwise;
        */
    }
    
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
