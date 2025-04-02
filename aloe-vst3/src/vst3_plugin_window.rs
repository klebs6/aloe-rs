crate::ix!();

#[no_copy]
#[leak_detector]
pub struct Vst3PluginWindow<'a> {
    base:                    AudioProcessorEditor<'a>,
    base2:                   ComponentMovementWatcher<'a>,

    ref_count:               Atomic<i32>, // default = { 1  }
    view:                    VstComSmartPtr<Box<dyn IPlugView>>,

    #[cfg(target_os="win32")] 
    embedded_component:      Vst3PluginWindowChildComponent,

    #[cfg(target_os="win32")] 
    peer:                    Box<ComponentPeer>,

    #[cfg(target_os="macos")]
    embedded_component:      NSViewComponentWithParent<'a>,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    run_loop:                SharedResourcePointer<RunLoop>,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    embedded_component:      XEmbedComponent, //{ true, false };

    #[cfg(not(any(target_os="win32",target_os="macos",target_os="linux",target_os="bsd")))]
    embedded_component:      aloe_component::Component<'a>,

    plugin_handle:           Vst3PluginWindowHandleFormat, // default = {}
    recursive_resize:        bool, // default = false
    has_done_initial_resize: bool, // default = false
    is_in_on_size:           bool, // default = false
    current_peer:            *mut ComponentPeer<'a>, // default = nullptr
    scale_interface:         *mut dyn IPlugViewContentScaleSupport<ScaleFactor = f32>, // default = nullptr
    native_scale_factor:     f32, // default = 1.0f
}

impl<'a> ComponentPeerScaleFactorListener for Vst3PluginWindow<'a> {
    fn native_scale_factor_changed(&mut self, _: f64) { todo!() }
}

impl<'a> IPlugFrame for Vst3PluginWindow<'a> {

    fn resize_view(&mut self, _: *mut (dyn aloe_vst_plugview::IPlugView + 'static), _: *mut aloe_vst_plugview::ViewRect) -> i32 { todo!() }
}

impl<'a> FUnknown for Vst3PluginWindow<'a> {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl<'a> Drop for Vst3PluginWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (scaleInterface != nullptr)
                scaleInterface->release();

            removeScaleFactorListener();

            #if ALOE_LINUX || ALOE_BSD
             embeddedComponent.removeClient();
            #endif

            warnOnFailure (view->removed());
            warnOnFailure (view->setFrame (nullptr));

            processor.editorBeingDeleted (this);

           #if ALOE_MAC
            embeddedComponent.setView (nullptr);
           #endif

            view = nullptr;
        */
    }
}

impl<'a> Vst3PluginWindow<'a> {

    pub fn new(
        owner:       *mut AudioPluginInstance,
        plugin_view: *mut dyn IPlugView) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (owner),
            ComponentMovementWatcher (this),
            view (pluginView, false)
           #if ALOE_MAC
          , embeddedComponent (*owner)
           #endif
            setSize (10, 10);
            setOpaque (true);
            setVisible (true);

            warnOnFailure (view->setFrame (this));
            view->queryInterface (IPlugViewContentScaleSupport::iid, (void**) &scaleInterface);
            resizeToFit();
        */
    }

    #[cfg(any(target_os="linux",target_os="bsd"))]
    pub fn query_interface(&mut self, 
        query_iid: TUID,
        obj:       *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            if (doUIDsMatch (queryIid, Linux::IRunLoop::iid))
            {
                *obj = &runLoop.get();
                return kResultTrue;
            }

            jassertfalse;
            *obj = nullptr;

            return kNotImplemented;
        */
    }

    #[cfg(not(any(target_os="linux",target_os="bsd")))]
    aloe_declare_vst3_com_query_methods!{}

    aloe_declare_vst3_com_ref_methods!{}

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        _0:    &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            view->onWheel (wheel.deltaY);
        */
    }
    
    pub fn focus_gained(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            view->onFocus (true);
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            view->onFocus (false);
        */
    }

    /**
      | It seems that most, if not all, plugins
      | do their own keyboard hooks, but IPlugView
      | does have a set of keyboard related methods...
      |
      */
    pub fn key_state_changed(&mut self, is_key_down: bool) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
            removeScaleFactorListener();
            currentPeer = getTopLevelComponent()->getPeer();

            if (currentPeer != nullptr)
            {
                currentPeer->addScaleFactorListener (this);
                nativeScaleFactor = (float) currentPeer->getPlatformScaleFactor();
            }
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        _0:          bool,
        was_resized: bool)  {
        
        todo!();
        /*
            if (recursiveResize || ! wasResized || getTopLevelComponent()->getPeer() == nullptr)
                return;

            ViewRect rect;

            if (view->canResize() == kResultTrue)
            {
                rect.right  = (i32) roundToInt ((float) getWidth()  * nativeScaleFactor);
                rect.bottom = (i32) roundToInt ((float) getHeight() * nativeScaleFactor);

                view->checkSizeConstraint (&rect);

                {
                    const ScopedValueSetter<bool> recursiveResizeSetter (recursiveResize, true);

                    setSize (roundToInt ((float) rect.getWidth()  / nativeScaleFactor),
                             roundToInt ((float) rect.getHeight() / nativeScaleFactor));
                }

               #if ALOE_WINDOWS
                setPluginWindowPos (rect);
               #else
                embeddedComponent.setBounds (getLocalBounds());
               #endif

                view->onSize (&rect);
            }
            else
            {
                warnOnFailure (view->getSize (&rect));

               #if ALOE_WINDOWS
                setPluginWindowPos (rect);
               #else
                resizeWithRect (embeddedComponent, rect, nativeScaleFactor);
               #endif
            }

            // Some plugins don't update their cursor correctly when mousing out the window
            Desktop::getInstance().getMainMouseSource().forceMouseCursorUpdate();
        */
    }
    
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
            attachPluginWindow();

            if (! hasDoneInitialResize)
                resizeToFit();

            componentMovedOrResized (true, true);
        */
    }
    
    pub fn native_scale_factor_changed(&mut self, new_scale_factor: f64)  {
        
        todo!();
        /*
            if (approximatelyEqual ((float) newScaleFactor, nativeScaleFactor))
                return;

            nativeScaleFactor = (float) newScaleFactor;

            if (pluginHandle != Vst3PluginWindowHandleFormat{} && scaleInterface != nullptr)
                scaleInterface->setContentScaleFactor ((IPlugViewContentScaleSupport::ScaleFactor) nativeScaleFactor);
            else
                resizeToFit();
        */
    }
    
    pub fn resize_to_fit(&mut self)  {
        
        todo!();
        /*
            ViewRect rect;
            warnOnFailure (view->getSize (&rect));
            resizeWithRect (*this, rect, nativeScaleFactor);

            hasDoneInitialResize = true;
        */
    }

    #[PLUGIN_API]
    pub fn resize_view(&mut self, 
        incoming_view: *mut dyn IPlugView,
        new_size:      *mut ViewRect) -> tresult {
        
        todo!();
        /*
            if (incomingView != nullptr && newSize != nullptr && incomingView == view)
            {
                auto scaleToViewRect = [this] (int dimension)
                {
                    return (i32) roundToInt ((float) dimension * nativeScaleFactor);
                };

                auto oldWidth  = scaleToViewRect (getWidth());
                auto oldHeight = scaleToViewRect (getHeight());

                resizeWithRect (embeddedComponent, *newSize, nativeScaleFactor);
                setSize (embeddedComponent.getWidth(), embeddedComponent.getHeight());

                // According to the Vst3 Workflow Diagrams, a resizeView from the plugin should
                // always trigger a response from the host which confirms the new size.
                ViewRect rect { 0, 0,
                                scaleToViewRect (getWidth()),
                                scaleToViewRect (getHeight()) };

                if (rect.right != oldWidth || rect.bottom != oldHeight
                    || ! isInOnSize)
                {
                    // Guard against plug-ins immediately calling resizeView() with the same size
                    const ScopedValueSetter<bool> inOnSizeSetter (isInOnSize, true);
                    view->onSize (&rect);
                }

                return kResultTrue;
            }

            jassertfalse;
            return kInvalidArgument;
        */
    }
    
    pub fn resize_with_rect(
        comp:         &mut aloe_component::Component,
        rect:         &ViewRect,
        scale_factor: f32)  {
        
        todo!();
        /*
            comp.setBounds (roundToInt ((float) rect.left / scaleFactor),
                            roundToInt ((float) rect.top  / scaleFactor),
                            jmax (10, std::abs (roundToInt ((float) rect.getWidth()  / scaleFactor))),
                            jmax (10, std::abs (roundToInt ((float) rect.getHeight() / scaleFactor))));
        */
    }
    
    pub fn attach_plugin_window(&mut self)  {
        
        todo!();
        /*
            if (pluginHandle == Vst3PluginWindowHandleFormat{})
            {
                #if ALOE_WINDOWS
                 if (auto* topComp = getTopLevelComponent())
                 {
                     peer.reset (embeddedComponent.createNewPeer (0, topComp->getWindowHandle()));
                     pluginHandle = (Vst3PluginWindowHandleFormat) peer->getNativeHandle();
                 }
                #else
                 embeddedComponent.setBounds (getLocalBounds());
                 addAndMakeVisible (embeddedComponent);
                 #if ALOE_MAC
                  pluginHandle = (Vst3PluginWindowHandleFormat) embeddedComponent.getView();
                 #elif ALOE_LINUX || ALOE_BSD
                  pluginHandle = (Vst3PluginWindowHandleFormat) embeddedComponent.getHostWindowID();
                 #endif
                #endif

                if (pluginHandle == Vst3PluginWindowHandleFormat{})
                {
                    jassertfalse;
                    return;
                }

                warnOnFailure (view->attached ((void*) pluginHandle, defaultVst3WindowType));

                if (scaleInterface != nullptr)
                    scaleInterface->setContentScaleFactor ((IPlugViewContentScaleSupport::ScaleFactor) nativeScaleFactor);
                else
                    resizeToFit();
            }
        */
    }
    
    pub fn remove_scale_factor_listener(&mut self)  {
        
        todo!();
        /*
            if (currentPeer == nullptr)
                return;

             for (int i = 0; i < ComponentPeer::getNumPeers(); ++i)
                 if (ComponentPeer::getPeer (i) == currentPeer)
                     currentPeer->removeScaleFactorListener (this);
        */
    }
    
    #[cfg(target_os="win32")]
    pub fn set_plugin_window_pos(&mut self, rect: ViewRect)  {
        
        todo!();
        /*
            if (auto* topComp = getTopLevelComponent())
            {
                auto pos = (topComp->getLocalPoint (this, Point<int>()) * nativeScaleFactor).roundToInt();

                ScopedThreadDPIAwarenessSetter threadDpiAwarenessSetter { pluginHandle };

                SetWindowPos (pluginHandle, nullptr,
                              pos.x, pos.y,
                              rect.getWidth(), rect.getHeight(),
                              isVisible() ? SWP_SHOWWINDOW : SWP_HIDEWINDOW);
            }
        */
    }
}
