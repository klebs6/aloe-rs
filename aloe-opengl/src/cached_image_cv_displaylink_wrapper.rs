crate::ix!();

#[cfg(target_os="macos")]
pub struct OpenGLContextCachedImageCVDisplayLinkWrapper<'a> {
    cached_image:       &'a mut OpenGLContextCachedImage<'a>,
    continuous_repaint: bool,
    display_link:       CVDisplayLinkRef,
}

#[cfg(target_os="macos")]
impl<'a> Drop for OpenGLContextCachedImageCVDisplayLinkWrapper<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            CVDisplayLinkStop (displayLink);
                CVDisplayLinkRelease (displayLink);
        */
    }
}

#[cfg(target_os="macos")]
impl<'a> OpenGLContextCachedImageCVDisplayLinkWrapper<'a> {

    pub fn new(cached_image_in: &mut OpenGLContextCachedImage) -> Self {
    
        todo!();
        /*
        : cached_image(cachedImageIn),
        : continuous_repaint(cachedImageIn.context.continuousRepaint.load()),

            CVDisplayLinkCreateWithActiveCGDisplays (&displayLink);
                CVDisplayLinkSetOutputCallback (displayLink, &displayLinkCallback, this);
                CVDisplayLinkStart (displayLink);

                const auto topLeftOfCurrentScreen = Desktop::getInstance()
                                                        .getDisplays()
                                                        .getDisplayForRect (cachedImage.component.getTopLevelComponent()->getScreenBounds())
                                                        ->totalArea.getTopLeft();
                updateActiveDisplay (topLeftOfCurrentScreen);
        */
    }
    
    pub fn get_nominal_video_refresh_periods(&self) -> f64 {
        
        todo!();
        /*
            const auto nominalVideoRefreshPeriod = CVDisplayLinkGetNominalOutputVideoRefreshPeriod (displayLink);

                if ((nominalVideoRefreshPeriod.flags & kCVTimeIsIndefinite) == 0)
                    return (double) nominalVideoRefreshPeriod.timeValue / (double) nominalVideoRefreshPeriod.timeScale;

                return 0.0;
        */
    }
    
    pub fn update_active_display(&mut self, top_left_of_display: Point<i32>)  {
        
        todo!();
        /*
            CGPoint point { (CGFloat) topLeftOfDisplay.getX(), (CGFloat) topLeftOfDisplay.getY() };
                CGDirectDisplayID displayID;
                uint32_t numDisplays = 0;
                constexpr uint32_t maxNumDisplays = 1;
                CGGetDisplaysWithPoint (point, maxNumDisplays, &displayID, &numDisplays);

                if (numDisplays == 1)
                    CVDisplayLinkSetCurrentCGDisplay (displayLink, displayID);
        */
    }
    
    pub fn display_link_callback(
        _0:                   CVDisplayLinkRef,
        _1:                   *const CVTimeStamp,
        _2:                   *const CVTimeStamp,
        _3:                   CVOptionFlags,
        _4:                   *mut CVOptionFlags,
        display_link_context: *mut c_void) -> CVReturn {
        
        todo!();
        /*
            auto* self = reinterpret_cast<OpenGLContextCachedImageCVDisplayLinkWrapper*> (displayLinkContext);

                if (self->continuousRepaint)
                    self->cachedImage.repaintEvent.signal();

                return kCVReturnSuccess;
        */
    }
}
