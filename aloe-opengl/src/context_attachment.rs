crate::ix!();

pub struct OpenGLContextAttachment<'a> {
    base:    ComponentMovementWatcher<'a>,
    base2:   Timer,
    context: &'a OpenGLContext<'a>,
}

impl<'a> Drop for OpenGLContextAttachment<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            detach();
        */
    }
}

impl<'a> OpenGLContextAttachment<'a> {

    pub fn new(
        c:    &mut OpenGLContext,
        comp: &mut Component) -> Self {
    
        todo!();
        /*
        : component_movement_watcher(&comp),
        : context(c),

            if (canBeAttached (comp))
                attach();
        */
    }
    
    pub fn detach(&mut self)  {
        
        todo!();
        /*
            auto& comp = *getComponent();
            stop();
            comp.setCachedComponentImage (nullptr);
            context.nativeContext = nullptr;
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            auto& comp = *getComponent();

            if (isAttached (comp) != canBeAttached (comp))
                componentVisibilityChanged();

            if (comp.getWidth() > 0 && comp.getHeight() > 0
                 && context.nativeContext != nullptr)
            {
                if (auto* c = OpenGLContextCachedImage::get (comp))
                    c->handleResize();

                if (auto* peer = comp.getTopLevelComponent()->getPeer())
                    context.nativeContext->updateWindowPosition (peer->getAreaCoveredBy (comp));
            }
        */
    }
    
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
            detach();
            componentVisibilityChanged();
        */
    }
    
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
            auto& comp = *getComponent();

            if (canBeAttached (comp))
            {
                if (isAttached (comp))
                    comp.repaint(); // (needed when windows are un-minimised)
                else
                    attach();
            }
            else
            {
                detach();
            }
        */
    }

    #[cfg(any(ALOE_DEBUG,ALOE_LOG_ASSERTIONS))]
    pub fn component_being_deleted(&mut self, c: &mut Component)  {
        
        todo!();
        /*
            /* You must call detach() or delete your OpenGLContext to remove it
               from a component BEFORE deleting the component that it is using!
            */
            jassertfalse;

            ComponentMovementWatcher::componentBeingDeleted (c);
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            auto& comp = *getComponent();

            if (canBeAttached (comp))
                start();
            else
                stop();
        */
    }
    
    pub fn can_be_attached(&mut self, comp: &Component) -> bool {
        
        todo!();
        /*
            return (! context.overrideCanAttach) && comp.getWidth() > 0 && comp.getHeight() > 0 && isShowingOrMinimised (comp);
        */
    }
    
    pub fn is_showing_or_minimised(c: &Component) -> bool {
        
        todo!();
        /*
            if (! c.isVisible())
                return false;

            if (auto* p = c.getParentComponent())
                return isShowingOrMinimised (*p);

            return c.getPeer() != nullptr;
        */
    }
    
    pub fn is_attached(comp: &Component) -> bool {
        
        todo!();
        /*
            return comp.getCachedComponentImage() != nullptr;
        */
    }
    
    pub fn attach(&mut self)  {
        
        todo!();
        /*
            auto& comp = *getComponent();
            auto* newCachedImage = new OpenGLContextCachedImage (context, comp,
                                                    context.openGLPixelFormat,
                                                    context.contextToShareWith);
            comp.setCachedComponentImage (newCachedImage);

            start();
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            stopTimer();

            auto& comp = *getComponent();

           #if ALOE_MAC
            [[(NSView*) comp.getWindowHandle() window] disableScreenUpdatesUntilFlush];
           #endif

            if (auto* oldCachedImage = OpenGLContextCachedImage::get (comp))
                oldCachedImage->stop(); // (must stop this before detaching it from the component)
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            auto& comp = *getComponent();

            if (auto* cachedImage = OpenGLContextCachedImage::get (comp))
            {
                cachedImage->start(); // (must wait until this is attached before starting its thread)
                cachedImage->updateViewportSize (true);

                startTimer (400);
            }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (auto* cachedImage = OpenGLContextCachedImage::get (*getComponent()))
                cachedImage->checkViewportBounds();
        */
    }
}
