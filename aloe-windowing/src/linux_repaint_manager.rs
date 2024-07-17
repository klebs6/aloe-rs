crate::ix!();

//#[cfg(target_os="linux")]
lazy_static!{
    /*
    static bool isActiveApplication;
    bool LinuxComponentPeer::isActiveApplication = false;
    */
}

///---------------------
#[no_copy]
//#[cfg(target_os="linux")]
pub struct LinuxRepaintManager<'a> {
    base:                          Timer,
    peer:                          &'a mut LinuxComponentPeer<'a>,
    is_semi_transparent_window:    bool,
    image:                         Image,
    last_time_image_used:          u32, // default = 0
    regions_needing_repaint:       RectangleList<i32>,
    use_argb_images_for_rendering: bool, //= XWindowSystem::getInstance()->canUseARGBImages();
}

//#[cfg(target_os="linux")]
pub const LINUX_REPAINT_MANAGER_REPAINT_TIMER_PERIOD: f32 = 1000.0 / 100.0;

//#[cfg(target_os="linux")]
impl<'a> LinuxRepaintManager<'a> {

    pub fn new(p: &mut LinuxComponentPeer) -> Self {
    
        todo!();
        /*

            : peer (p),
                  isSemiTransparentWindow ((peer.getStyleFlags() & ComponentPeer::windowIsSemiTransparent) != 0)
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            XWindowSystem::getInstance()->processPendingPaintsForWindow (peer.windowH);

                if (XWindowSystem::getInstance()->getNumPaintsPendingForWindow (peer.windowH) > 0)
                    return;

                if (! regionsNeedingRepaint.isEmpty())
                {
                    stopTimer();
                    performAnyPendingRepaintsNow();
                }
                else if (Time::getApproximateMillisecondCounter() > lastTimeImageUsed + 3000)
                {
                    stopTimer();
                    image = Image();
                }
        */
    }
    
    pub fn repaint(&mut self, area: Rectangle<i32>)  {
        
        todo!();
        /*
            if (! isTimerRunning())
                    startTimer (repaintTimerPeriod);

                regionsNeedingRepaint.add (area * peer.currentScaleFactor);
        */
    }
    
    pub fn perform_any_pending_repaints_now(&mut self)  {
        
        todo!();
        /*
            if (XWindowSystem::getInstance()->getNumPaintsPendingForWindow (peer.windowH) > 0)
                {
                    startTimer (repaintTimerPeriod);
                    return;
                }

                auto originalRepaintRegion = regionsNeedingRepaint;
                regionsNeedingRepaint.clear();
                auto totalArea = originalRepaintRegion.getBounds();

                if (! totalArea.isEmpty())
                {
                    if (image.isNull() || image.getWidth() < totalArea.getWidth()
                         || image.getHeight() < totalArea.getHeight())
                    {
                        image = XWindowSystem::getInstance()->createImage (isSemiTransparentWindow,
                                                                           totalArea.getWidth(), totalArea.getHeight(),
                                                                           useARGBImagesForRendering);
                    }

                    startTimer (repaintTimerPeriod);

                    RectangleList<int> adjustedList (originalRepaintRegion);
                    adjustedList.offsetAll (-totalArea.getX(), -totalArea.getY());

                    if (XWindowSystem::getInstance()->canUseARGBImages())
                        for (auto& i : originalRepaintRegion)
                            image.clear (i - totalArea.getPosition());

                    {
                        auto context = peer.getComponent().getLookAndFeel()
                                         .createGraphicsContext (image, -totalArea.getPosition(), adjustedList);

                        context->addTransform (AffineTransform::scale ((float) peer.currentScaleFactor));
                        peer.handlePaint (*context);
                    }

                    for (auto& i : originalRepaintRegion)
                       XWindowSystem::getInstance()->blitToWindow (peer.windowH, image, i, totalArea);
                }

                lastTimeImageUsed = Time::getApproximateMillisecondCounter();
                startTimer (repaintTimerPeriod);
        */
    }
}
