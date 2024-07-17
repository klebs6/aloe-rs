crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/aloe_linux_Windowing.cpp]

//#[cfg(target_os="linux")]
lazy_static!{
    /*
    static int numAlwaysOnTopPeers = 0;
    */
}

//#[cfg(target_os="linux")]
pub fn aloe_are_there_any_always_on_top_windows() -> bool {
    
    todo!();
    /*
        return numAlwaysOnTopPeers > 0;
    */
}

///-----------------------
#[no_copy]
#[leak_detector]
//#[cfg(target_os="linux")]
pub struct LinuxComponentPeer<'a> {
    base:                 ComponentPeer<'a>,
    focused:              bool, // default = false
    repainter:            Box<LinuxRepaintManager<'a>>,
    windowh:              Window,
    parent_window:        Window,
    bounds:               Rectangle<i32>,
    window_border:        BorderSize<i32>,
    full_screen:          bool, // default = false
    is_always_on_top:     bool, // default = false
    current_scale_factor: f64, // default = 1.0
    gl_repaint_listeners: Vec<*mut Component<'a>>,
}

//#[cfg(target_os="linux")]
impl<'a> Drop for LinuxComponentPeer<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            // it's dangerous to delete a window on a thread other than the message thread.
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

            repainter = nullptr;
            XWindowSystem::getInstance()->destroyWindow (windowH);

            if (isAlwaysOnTop)
                --numAlwaysOnTopPeers;
         */
    }
}

//#[cfg(target_os="linux")]
impl<'a> LinuxComponentPeer<'a> {

    pub fn new(
        comp:               &mut Component<'a>,
        window_style_flags: i32,
        parent_to_add_to:   Window) -> Self {
    
        todo!();
        /*
        : component_peer(comp, windowStyleFlags),
        : is_always_on_top(comp.isAlwaysOnTop()),

            // it's dangerous to create a window on a thread other than the message thread.
            ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

            if (! XWindowSystem::getInstance()->isX11Available())
                return;

            if (isAlwaysOnTop)
                ++numAlwaysOnTopPeers;

            repainter = std::make_unique<LinuxRepaintManager> (*this);

            windowH = XWindowSystem::getInstance()->createWindow (parentToAddTo, this);
            parentWindow = parentToAddTo;

            setTitle (component.getName());

            getNativeRealtimeModifiers = []() -> ModifierKeys { return XWindowSystem::getInstance()->getNativeRealtimeModifiers(); };
        */
    }
    
    pub fn get_window_handle(&self) -> Window {
        
        todo!();
        /*
            return windowH;
        */
    }
    
    pub fn get_native_handle(&self)  {
        
        todo!();
        /*
            return reinterpret_cast<void*> (getWindowHandle());
        */
    }
    
    pub fn set_bounds(&mut self, 
        new_bounds:         &Rectangle<i32>,
        is_now_full_screen: bool)  {
        
        todo!();
        /*
            const auto correctedNewBounds = newBounds.withSize (jmax (1, newBounds.getWidth()),
                                                                jmax (1, newBounds.getHeight()));

            if (bounds == correctedNewBounds && fullScreen == isNowFullScreen)
                return;

            bounds = correctedNewBounds;

            updateScaleFactorFromNewBounds (bounds, false);

            auto physicalBounds = parentWindow == 0 ? Desktop::getInstance().getDisplays().logicalToPhysical (bounds)
                                                    : bounds * currentScaleFactor;

            WeakReference<Component> deletionChecker (&component);

            XWindowSystem::getInstance()->setBounds (windowH, physicalBounds, isNowFullScreen);

            fullScreen = isNowFullScreen;

            if (deletionChecker != nullptr)
            {
                updateBorderSize();
                handleMovedOrResized();
            }
        */
    }
    
    pub fn get_screen_position(&self, physical: bool) -> Point<i32> {
        
        todo!();
        /*
            auto physicalParentPosition = XWindowSystem::getInstance()->getPhysicalParentScreenPosition();
            auto parentPosition = parentWindow == 0 ? Desktop::getInstance().getDisplays().physicalToLogical (physicalParentPosition)
                                                    : physicalParentPosition / currentScaleFactor;

            auto screenBounds = parentWindow == 0 ? bounds
                                                  : bounds.translated (parentPosition.x, parentPosition.y);

            if (physical)
                return parentWindow == 0 ? Desktop::getInstance().getDisplays().logicalToPhysical (screenBounds.getTopLeft())
                                         : screenBounds.getTopLeft() * currentScaleFactor;

            return screenBounds.getTopLeft();
        */
    }
    
    pub fn get_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return bounds;
        */
    }
    
    pub fn get_frame_size(&self) -> BorderSize<i32> {
        
        todo!();
        /*
            return windowBorder;
        */
    }
    
    pub fn local_to_global(&mut self, relative_position: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            return relativePosition + getScreenPosition (false).toFloat();
        */
    }
    
    pub fn global_to_local(&mut self, screen_position: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            return screenPosition - getScreenPosition (false).toFloat();
        */
    }
    
    pub fn get_available_rendering_engines(&mut self) -> StringArray {
        
        todo!();
        /*
            return { "Software Renderer" };
        */
    }
    
    pub fn set_visible(&mut self, should_be_visible: bool)  {
        
        todo!();
        /*
            XWindowSystem::getInstance()->setVisible (windowH, shouldBeVisible);
        */
    }
    
    pub fn set_title(&mut self, title: &String)  {
        
        todo!();
        /*
            XWindowSystem::getInstance()->setTitle (windowH, title);
        */
    }
    
    pub fn set_minimised(&mut self, should_be_minimised: bool)  {
        
        todo!();
        /*
            if (shouldBeMinimised)
                XWindowSystem::getInstance()->setMinimised (windowH, shouldBeMinimised);
            else
                setVisible (true);
        */
    }
    
    pub fn is_minimised(&self) -> bool {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->isMinimised (windowH);
        */
    }
    
    pub fn set_full_screen(&mut self, should_be_full_screen: bool)  {
        
        todo!();
        /*
            auto r = lastNonFullscreenBounds; // (get a copy of this before de-minimising)

            setMinimised (false);

            if (fullScreen != shouldBeFullScreen)
            {
                const auto usingNativeTitleBar = ((styleFlags & windowHasTitleBar) != 0);

                if (usingNativeTitleBar)
                    XWindowSystem::getInstance()->setMaximised (windowH, shouldBeFullScreen);

                if (shouldBeFullScreen)
                    r = usingNativeTitleBar ? XWindowSystem::getInstance()->getWindowBounds (windowH, parentWindow)
                                            : Desktop::getInstance().getDisplays().getDisplayForRect (bounds)->userArea;

                if (! r.isEmpty())
                    setBounds (ScalingHelpers::scaledScreenPosToUnscaled (component, r), shouldBeFullScreen);

                component.repaint();
            }
        */
    }
    
    pub fn is_full_screen(&self) -> bool {
        
        todo!();
        /*
            return fullScreen;
        */
    }
    
    pub fn contains(&self, 
        local_pos:                Point<i32>,
        true_if_in_achild_window: bool) -> bool {
        
        todo!();
        /*
            if (! bounds.withZeroOrigin().contains (localPos))
                return false;

            for (int i = Desktop::getInstance().getNumComponents(); --i >= 0;)
            {
                auto* c = Desktop::getInstance().getComponent (i);

                if (c == &component)
                    break;

                if (! c->isVisible())
                    continue;

                if (auto* peer = c->getPeer())
                    if (peer->contains (localPos + bounds.getPosition() - peer->getBounds().getPosition(), true))
                        return false;
            }

            if (trueIfInAChildWindow)
                return true;

            return XWindowSystem::getInstance()->contains (windowH, localPos * currentScaleFactor);
        */
    }
    
    pub fn to_front(&mut self, make_active: bool)  {
        
        todo!();
        /*
            if (makeActive)
            {
                setVisible (true);
                grabFocus();
            }

            XWindowSystem::getInstance()->toFront (windowH, makeActive);
            handleBroughtToFront();
        */
    }
    
    pub fn to_behind(&mut self, other: *mut ComponentPeer)  {
        
        todo!();
        /*
            if (auto* otherPeer = dynamic_cast<LinuxComponentPeer*> (other))
            {
                if (otherPeer->styleFlags & windowIsTemporary)
                    return;

                setMinimised (false);
                XWindowSystem::getInstance()->toBehind (windowH, otherPeer->windowH);
            }
            else
            {
                jassertfalse; // wrong type of window?
            }
        */
    }
    
    pub fn is_focused(&self) -> bool {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->isFocused (windowH);
        */
    }
    
    pub fn grab_focus(&mut self)  {
        
        todo!();
        /*
            if (XWindowSystem::getInstance()->grabFocus (windowH))
                isActiveApplication = true;
        */
    }
    
    pub fn repaint(&mut self, area: &Rectangle<i32>)  {
        
        todo!();
        /*
            if (repainter != nullptr)
                repainter->repaint (area.getIntersection (bounds.withZeroOrigin()));
        */
    }
    
    pub fn perform_any_pending_repaints_now(&mut self)  {
        
        todo!();
        /*
            if (repainter != nullptr)
                repainter->performAnyPendingRepaintsNow();
        */
    }
    
    pub fn set_icon(&mut self, new_icon: &Image)  {
        
        todo!();
        /*
            XWindowSystem::getInstance()->setIcon (windowH, newIcon);
        */
    }
    
    pub fn get_platform_scale_factor(&self) -> f64 {
        
        todo!();
        /*
            return currentScaleFactor;
        */
    }
    
    pub fn set_alpha(&mut self, _0: f32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_always_on_top(&mut self, _0: bool) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn text_input_required(
        &mut self, 
        _0: Point<i32>,
        _1: &mut dyn TextInputTarget

    )  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn add_open_gl_repaint_listener(&mut self, dummy: *mut Component)  {
        
        todo!();
        /*
            if (dummy != nullptr)
                glRepaintListeners.addIfNotAlreadyThere (dummy);
        */
    }
    
    pub fn remove_open_gl_repaint_listener(&mut self, dummy: *mut Component)  {
        
        todo!();
        /*
            if (dummy != nullptr)
                glRepaintListeners.removeAllInstancesOf (dummy);
        */
    }
    
    pub fn repaint_open_gl_contexts(&mut self)  {
        
        todo!();
        /*
            for (auto* c : glRepaintListeners)
                c->handleCommandMessage (0);
        */
    }
    
    pub fn get_parent_window(&mut self) -> Window {
        
        todo!();
        /*
            return parentWindow;
        */
    }
    
    pub fn set_parent_window(&mut self, new_parent: Window)  {
        
        todo!();
        /*
            parentWindow = newParent;
        */
    }
    
    pub fn update_window_bounds(&mut self)  {
        
        todo!();
        /*
            jassert (windowH != 0);
            if (windowH != 0)
            {
                auto physicalBounds = XWindowSystem::getInstance()->getWindowBounds (windowH, parentWindow);

                updateScaleFactorFromNewBounds (physicalBounds, true);

                bounds = parentWindow == 0 ? Desktop::getInstance().getDisplays().physicalToLogical (physicalBounds)
                                           : physicalBounds / currentScaleFactor;
            }
        */
    }
    
    pub fn update_border_size(&mut self)  {
        
        todo!();
        /*
            if ((styleFlags & windowHasTitleBar) == 0)
                windowBorder = {};
            else if (windowBorder.getTopAndBottom() == 0 && windowBorder.getLeftAndRight() == 0)
                windowBorder = XWindowSystem::getInstance()->getBorderSize (windowH);
        */
    }
    
    pub fn update_scale_factor_from_new_bounds(&mut self, 
        new_bounds:  &Rectangle<i32>,
        is_physical: bool)  {
        
        todo!();
        /*
            Point<int> translation = (parentWindow != 0 ? getScreenPosition (isPhysical) : Point<int>());
            const auto& desktop = Desktop::getInstance();

            if (auto* display = desktop.getDisplays().getDisplayForRect (newBounds.translated (translation.x, translation.y),
                                                                         isPhysical))
            {
                auto newScaleFactor = display->scale / desktop.getGlobalScaleFactor();

                if (! approximatelyEqual (newScaleFactor, currentScaleFactor))
                {
                    currentScaleFactor = newScaleFactor;
                    scaleFactorListeners.call ([&] (ScaleFactorListener& l) { l.nativeScaleFactorChanged (currentScaleFactor); });
                }
            }
        */
    }
}

//#[cfg(target_os="linux")]
lazy_static!{
    /*
    static bool screenSaverAllowed = true;
    */
}

//#[cfg(target_os="linux")]
pub fn get_peer_for_drag_event(source_comp: *mut Component) -> *mut LinuxComponentPeer {
    
    todo!();
        /*
            if (sourceComp == nullptr)
            if (auto* draggingSource = Desktop::getInstance().getDraggingMouseSource (0))
                sourceComp = draggingSource->getComponentUnderMouse();

        if (sourceComp != nullptr)
            if (auto* lp = dynamic_cast<LinuxComponentPeer*> (sourceComp->getPeer()))
                return lp;

        jassertfalse;  // This method must be called in response to a component's mouseDown or mouseDrag event!
        return nullptr;
        */

}

//#[cfg(target_os="linux")]
pub fn aloe_create_icon_for_file(_0: &File) -> Image {
    
    todo!();
        /*
            return {};
        */

}

//#[cfg(target_os="linux")]
pub fn aloe_linux_add_repaint_listener(
        peer:  *mut ComponentPeer,
        dummy: *mut Component)  {
    
    todo!();
        /*
            if (auto* linuxPeer = dynamic_cast<LinuxComponentPeer*> (peer))
            linuxPeer->addOpenGLRepaintListener (dummy);
        */

}

//#[cfg(target_os="linux")]
pub fn aloe_linux_remove_repaint_listener(
        peer:  *mut ComponentPeer,
        dummy: *mut Component)  {
    
    todo!();
        /*
            if (auto* linuxPeer = dynamic_cast<LinuxComponentPeer*> (peer))
            linuxPeer->removeOpenGLRepaintListener (dummy);
        */

}
