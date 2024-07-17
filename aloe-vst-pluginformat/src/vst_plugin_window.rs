crate::ix!();

///----------------------------
#[cfg(not(any(target_os="ios",target_os="android")))]
#[no_copy]
#[leak_detector]
pub struct VSTPluginWindow<'a> {
    base:  AudioProcessorEditor<'a>,

    #[cfg(not(target_os="macos"))]
    base2: ComponentMovementWatcher,

    #[cfg(not(target_os="macos"))]
    base3: ComponentPeer::ScaleFactorListener,

    base4: Timer,

    #[cfg(all(target_os="macos",ALOE_SUPPORT_CARBON))]
    carbon_wrapper: Box<VSTPluginWindowCarbonWrapperComponent>,

    #[cfg(target_os="macos")]
    cocoa_wrapper:  Box<NSViewComponentWithParent<'a>>,
    
    plugin:                   &'a mut VSTPluginInstance<'a>,
    is_open:                  bool, // default = false
    recursive_resize:         bool, // default = false
    plugin_wants_keys:        bool, // default = false
    plugin_refuses_to_resize: bool, // default = false
    already_inside:           bool, // default = false

    #[cfg(not(target_os="macos"))]
    plugin_responds_to_dpi_changes: bool, // default = false

    #[cfg(not(target_os="macos"))]
    native_scale_factor:            f32, // default = 1.0f

    #[cfg(not(target_os="macos"))]
    #[cfg(target_os="windows")]
    pluginhwnd:        HWND,

    #[cfg(not(target_os="macos"))]
    #[cfg(target_os="windows")]
    original_wnd_proc: *mut c_void,

    #[cfg(not(target_os="macos"))]
    #[cfg(target_os="windows")]
    size_check_count:  i32, // default = 0

    #[cfg(not(target_os="macos"))]
    #[cfg(any(target_os="linux",target_os="bsd"))]
    display:       *mut Display, //= XWindowSystem::getInstance()->getDisplay();

    #[cfg(not(target_os="macos"))]
    #[cfg(any(target_os="linux",target_os="bsd"))]
    plugin_window: Window, // default = 0
}

#[cfg(not(any(target_os="ios",target_os="android")))]
impl<'a> Drop for VSTPluginWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            closePluginWindow();

           #if ALOE_MAC
            #if ALOE_SUPPORT_CARBON
            carbonWrapper.reset();
            #endif
            cocoaWrapper.reset();
           #else
            removeScaleFactorListeners();
           #endif

            activeVSTWindows.removeFirstMatchingValue (this);
            plugin.editorBeingDeleted (this);
        */
    }
}

impl<'a> VSTPluginWindow<'a> {
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn new(plug: &mut VSTPluginInstance) -> Self {
    
        todo!();
        /*


            : AudioProcessorEditor (&plug),
             #if ! ALOE_MAC
              ComponentMovementWatcher (this),
             #endif
              plugin (plug)
           #if ALOE_LINUX || ALOE_BSD
            pluginWindow = None;
            ignoreUnused (pluginRefusesToResize, alreadyInside);
           #elif ALOE_MAC
            ignoreUnused (recursiveResize, pluginRefusesToResize, alreadyInside);

            #if ALOE_SUPPORT_CARBON
            if (! plug.usesCocoaNSView)
            {
                carbonWrapper.reset (new VSTPluginWindowCarbonWrapperComponent (*this));
                addAndMakeVisible (carbonWrapper.get());
            }
            else
            #endif
            {
                cocoaWrapper.reset (new NSViewComponentWithParent (plugin));
                addAndMakeVisible (cocoaWrapper.get());
            }
           #endif

            activeVSTWindows.add (this);

            typename Vst2ERect* rect = nullptr;
            dispatch (typename Vst2EffEditGetRect, 0, 0, &rect, 0);

            if (rect != nullptr)
                updateSizeFromEditor (rect->right - rect->left, rect->bottom - rect->top);
            else
                updateSizeFromEditor (1, 1);

            setOpaque (true);
            setVisible (true);
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn update_size_from_editor(&mut self, w: i32, h: i32)  {
        
        todo!();
        /*
            if (! plugin.updateSizeFromEditor (w, h))
                setSize (w, h);
        */
    }

    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="macos")]
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="macos")]
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            if (cocoaWrapper != nullptr)
            {
                if (isVisible())
                    openPluginWindow ((NSView*) cocoaWrapper->getView());
                else
                    closePluginWindow();
            }
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="macos")]
    pub fn child_bounds_changed(&mut self, _0: *mut Component)  {
        
        todo!();
        /*
            if (cocoaWrapper != nullptr)
            {
                auto w = cocoaWrapper->getWidth();
                auto h = cocoaWrapper->getHeight();

                if (w != getWidth() || h != getHeight())
                    setSize (w, h);
            }
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            #if ALOE_LINUX || ALOE_BSD
            if (isOpen)
            {
                if (pluginWindow != 0)
                {
                    auto clip = g.getClipBounds();

                    X11Symbols::getInstance()->xClearArea (display, pluginWindow, clip.getX(), clip.getY(),
                                                           static_cast<unsigned int> (clip.getWidth()),
                                                           static_cast<unsigned int> (clip.getHeight()), True);
                }
            }
            else
           #endif
            {
                g.fillAll (Colours::black);
            }
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn component_moved_or_resized(&mut self, 
        was_moved:   bool,
        was_resized: bool)  {
        
        todo!();
        /*
            if (recursiveResize)
                return;

            if (auto* peer = getTopLevelComponent()->getPeer())
            {
                const ScopedValueSetter<bool> recursiveResizeSetter (recursiveResize, true);

                const auto pos = (peer->getAreaCoveredBy (*this).toFloat() * nativeScaleFactor).toNearestInt();

               #if ALOE_WINDOWS
                if (pluginHWND != 0)
                {
                    ScopedThreadDPIAwarenessSetter threadDpiAwarenessSetter { pluginHWND };
                    SetWindowPos (pluginHWND,
                                  HWND_BOTTOM,
                                  pos.getX(),
                                  pos.getY(),
                                  0,
                                  0,
                                  SWP_NOSIZE | SWP_NOZORDER | SWP_NOOWNERZORDER);
                }
               #elif ALOE_LINUX || ALOE_BSD
                if (pluginWindow != 0)
                {
                    const auto editorSize = plugin.getEditorSize();
                    auto* symbols = X11Symbols::getInstance();
                    symbols->xMoveResizeWindow (display,
                                                pluginWindow,
                                                pos.getX(),
                                                pos.getY(),
                                                (unsigned int) editorSize.getWidth(),
                                                (unsigned int) editorSize.getHeight());
                    symbols->xMapRaised (display, pluginWindow);
                    symbols->xFlush (display);
                }
               #endif
            }
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn component_visibility_changed(&mut self)  {
        
        todo!();
        /*
            if (isShowing())
                openPluginWindow();
            else if (! shouldAvoidDeletingWindow())
                closePluginWindow();

            if (auto* peer = getTopLevelComponent()->getPeer())
                setScaleFactorAndDispatchMessage (peer->getPlatformScaleFactor());

           #if ALOE_LINUX || ALOE_BSD
            MessageManager::callAsync ([safeThis = SafePointer<VSTPluginWindow> { this }]
            {
                if (safeThis != nullptr)
                    safeThis->componentMovedOrResized (true, true);
            });
           #else
            componentMovedOrResized (true, true);
           #endif
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn component_peer_changed(&mut self)  {
        
        todo!();
        /*
            closePluginWindow();
            openPluginWindow();

            removeScaleFactorListeners();

            if (auto* peer = getTopLevelComponent()->getPeer())
                peer->addScaleFactorListener (this);

            componentMovedOrResized (true, true);
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn native_scale_factor_changed(&mut self, new_scale_factor: f64)  {
        
        todo!();
        /*
            setScaleFactorAndDispatchMessage (newScaleFactor);
            componentMovedOrResized (true, true);
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn set_scale_factor_and_dispatch_message(&mut self, new_scale_factor: f64)  {
        
        todo!();
        /*
            if (approximatelyEqual ((float) newScaleFactor, nativeScaleFactor))
                return;

            nativeScaleFactor = (float) newScaleFactor;

            if (pluginRespondsToDPIChanges)
                dispatch (typename Vst2EffVendorSpecific,
                          (int) ByteOrder::bigEndianInt ("PreS"),
                          (int) ByteOrder::bigEndianInt ("AeCs"),
                          nullptr, nativeScaleFactor);
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn key_state_changed(&mut self, _0: bool) -> bool {
        
        todo!();
        /*
            return pluginWantsKeys;
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn key_pressed(&mut self, _0: &KeyPress) -> bool {
        
        todo!();
        /*
            return pluginWantsKeys;
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (isShowing())
            {
               #if ALOE_WINDOWS
                if (--sizeCheckCount <= 0)
                {
                    sizeCheckCount = 10;
                    checkPluginWindowSize();
                }
               #endif

                static bool reentrantGuard = false;

                if (! reentrantGuard)
                {
                    reentrantGuard = true;
                    plugin.dispatch (typename Vst2EffEditIdle, 0, 0, nullptr, 0);
                    reentrantGuard = false;
                }

               #if ALOE_LINUX || ALOE_BSD
                if (pluginWindow == 0)
                {
                    updatePluginWindowHandle();

                    if (pluginWindow != 0)
                        componentMovedOrResized (true, true);
                }
               #endif
            }
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            ignoreUnused (e);

           #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
            toFront (true);
           #endif
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn brought_to_front(&mut self)  {
        
        todo!();
        /*
            activeVSTWindows.removeFirstMatchingValue (this);
            activeVSTWindows.add (this);

           #if ALOE_MAC
            dispatch (typename Vst2EffEditTop, 0, 0, nullptr, 0);
           #endif
        */
    }

    /**
       This is a workaround for old Mackie plugins
       that crash if their window is deleted more
       than once.
      */
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn should_avoid_deleting_window(&self) -> bool {
        
        todo!();
        /*
            return plugin.getPluginDescription()
                    .manufacturerName.containsIgnoreCase ("Loud Technologies");
        */
    }

    /**
      | This is an old workaround for some plugins
      | that need a repaint when their windows are
      | first created, but it breaks some Izotope
      | plugins..
      */
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn should_repaint_carbon_window_when_created(&mut self) -> bool {
        
        todo!();
        /*
            return ! plugin.getName().containsIgnoreCase ("izotope");
        */
    }

    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="macos")]
    pub fn open_plugin_window(&mut self, parent_window: *mut c_void)  {
        
        todo!();
        /*
            if (isOpen || parentWindow == nullptr)
                return;

            isOpen = true;

            typename Vst2ERect* rect = nullptr;
            dispatch (typename Vst2EffEditGetRect, 0, 0, &rect, 0);
            dispatch (typename Vst2EffEditOpen, 0, 0, parentWindow, 0);

            // do this before and after like in the steinberg example
            dispatch (typename Vst2EffEditGetRect, 0, 0, &rect, 0);
            dispatch (typename Vst2EffGetProgram, 0, 0, nullptr, 0); // also in steinberg code

            // Install keyboard hooks
            pluginWantsKeys = (dispatch (typename Vst2EffKeysRequired, 0, 0, nullptr, 0) == 0);

            // double-check it's not too tiny
            int w = 250, h = 150;

            if (rect != nullptr)
            {
                w = rect->right - rect->left;
                h = rect->bottom - rect->top;

                if (w == 0 || h == 0)
                {
                    w = 250;
                    h = 150;
                }
            }

            w = jmax (w, 32);
            h = jmax (h, 32);

            updateSizeFromEditor (w, h);

            startTimer (18 + Random::getSystemRandom().nextInt (5));
            repaint();
        */
    }

    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn open_plugin_window(&mut self)  {
        
        todo!();
        /*
            if (isOpen || getWindowHandle() == nullptr)
                return;

            ALOE_VST_LOG ("Opening VST UI: " + plugin.getName());
            isOpen = true;

            pluginRespondsToDPIChanges = plugin.pluginCanDo ("supportsViewDpiScaling") > 0;

            if (auto* peer = getTopLevelComponent()->getPeer())
                setScaleFactorAndDispatchMessage (peer->getPlatformScaleFactor());

            typename Vst2ERect* rect = nullptr;

            dispatch (typename Vst2EffEditGetRect, 0, 0, &rect, 0);
            dispatch (typename Vst2EffEditOpen, 0, 0, getWindowHandle(), 0);
            dispatch (typename Vst2EffEditGetRect, 0, 0, &rect, 0);  // do this before and after like in the steinberg example
            dispatch (typename Vst2EffGetProgram, 0, 0, nullptr, 0); // also in steinberg code

            pluginWantsKeys = (dispatch (typename Vst2EffKeysRequired, 0, 0, nullptr, 0) == 0);

           #if ALOE_WINDOWS
            originalWndProc = 0;
            pluginHWND = GetWindow ((HWND) getWindowHandle(), GW_CHILD);

            if (pluginHWND == 0)
            {
                isOpen = false;
                setSize (300, 150);
                return;
            }

            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (4244)

            if (! pluginWantsKeys)
            {
                originalWndProc = (void*) GetWindowLongPtr (pluginHWND, GWLP_WNDPROC);
                SetWindowLongPtr (pluginHWND, GWLP_WNDPROC, (LONG_PTR) vstHookWndProc);
            }

            ALOE_END_IGNORE_WARNINGS_MSVC

            RECT r;

            {
                ScopedThreadDPIAwarenessSetter threadDpiAwarenessSetter { pluginHWND };
                GetWindowRect (pluginHWND, &r);
            }

            auto w = (int) (r.right - r.left);
            auto h = (int) (r.bottom - r.top);

            if (rect != nullptr)
            {
                auto rw = rect->right - rect->left;
                auto rh = rect->bottom - rect->top;

                if ((rw > 50 && rh > 50 && rw < 2000 && rh < 2000 && (! isWithin (w, rw, 2) || ! isWithin (h, rh, 2)))
                    || ((w == 0 && rw > 0) || (h == 0 && rh > 0)))
                {
                    // very dodgy logic to decide which size is right.
                    if (std::abs (rw - w) > 350 || std::abs (rh - h) > 350)
                    {
                        ScopedThreadDPIAwarenessSetter threadDpiAwarenessSetter { pluginHWND };

                        SetWindowPos (pluginHWND, 0,
                                      0, 0, roundToInt (rw * nativeScaleFactor), roundToInt (rh * nativeScaleFactor),
                                      SWP_NOMOVE | SWP_NOACTIVATE | SWP_NOOWNERZORDER | SWP_NOZORDER);

                        GetWindowRect (pluginHWND, &r);

                        w = r.right - r.left;
                        h = r.bottom - r.top;

                        pluginRefusesToResize = (w != rw) || (h != rh);

                        w = rw;
                        h = rh;
                    }
                }
            }
           #elif ALOE_LINUX || ALOE_BSD
            updatePluginWindowHandle();

            int w = 250, h = 150;

            if (rect != nullptr)
            {
                w = rect->right - rect->left;
                h = rect->bottom - rect->top;

                if (w == 0 || h == 0)
                {
                    w = 250;
                    h = 150;
                }
            }

            if (pluginWindow != 0)
                X11Symbols::getInstance()->xMapRaised (display, pluginWindow);
           #endif

            w = roundToInt ((float) w / nativeScaleFactor);
            h = roundToInt ((float) h / nativeScaleFactor);

            // double-check it's not too tiny
            w = jmax (w, 32);
            h = jmax (h, 32);

            updateSizeFromEditor (w, h);

           #if ALOE_WINDOWS
            checkPluginWindowSize();
           #endif

            startTimer (18 + Random::getSystemRandom().nextInt (5));
            repaint();
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(not(target_os="macos"))]
    pub fn remove_scale_factor_listeners(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < ComponentPeer::getNumPeers(); ++i)
                if (auto* peer = ComponentPeer::getPeer (i))
                    peer->removeScaleFactorListener (this);
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn close_plugin_window(&mut self)  {
        
        todo!();
        /*
            if (isOpen)
            {
                // You shouldn't end up hitting this assertion unless the host is trying to do GUI
                // cleanup on a non-GUI thread.. If it does that, bad things could happen in here..
                ALOE_ASSERT_MESSAGE_MANAGER_IS_LOCKED

                ALOE_VST_LOG ("Closing VST UI: " + plugin.getName());
                isOpen = false;
                dispatch (typename Vst2EffEditClose, 0, 0, nullptr, 0);
                stopTimer();

               #if ALOE_WINDOWS
                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (4244)
                if (originalWndProc != 0 && pluginHWND != 0 && IsWindow (pluginHWND))
                    SetWindowLongPtr (pluginHWND, GWLP_WNDPROC, (LONG_PTR) originalWndProc);
                ALOE_END_IGNORE_WARNINGS_MSVC

                originalWndProc = 0;
                pluginHWND = 0;
               #elif ALOE_LINUX || ALOE_BSD
                pluginWindow = 0;
               #endif
            }
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    pub fn dispatch(&mut self, 
        opcode: i32,
        index:  i32,
        value:  i32,
        ptr:    *mut c_void,
        opt:    f32) -> PointerSizedInt {
        
        todo!();
        /*
            return plugin.dispatch (opcode, index, value, ptr, opt);
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="windows")]
    pub fn will_cause_recursive_resize(&mut self, w: i32, h: i32) -> bool {
        
        todo!();
        /*
            auto newScreenBounds = Rectangle<int> (w, h).withPosition (getScreenPosition());
            return Desktop::getInstance().getDisplays().getDisplayForRect (newScreenBounds)->scale != nativeScaleFactor;
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="windows")]
    pub fn is_window_size_correct_for_plugin(&mut self, w: i32, h: i32) -> bool {
        
        todo!();
        /*
            if (! isShowing() || pluginRefusesToResize)
                return true;

            return (isWithin (w, getWidth(), 5) && isWithin (h, getHeight(), 5));
        */
    }
    
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="windows")]
    pub fn check_plugin_window_size(&mut self)  {
        
        todo!();
        /*
            if (! pluginRespondsToDPIChanges)
            {
                typename Vst2ERect* rect = nullptr;
                dispatch (typename Vst2EffEditGetRect, 0, 0, &rect, 0);

                auto w = roundToInt ((rect->right - rect->left) / nativeScaleFactor);
                auto h = roundToInt ((rect->bottom - rect->top) / nativeScaleFactor);

                if (! isWindowSizeCorrectForPlugin (w, h))
                {
                    // If plug-in isn't DPI aware then we need to resize our window, but this may cause a recursive resize
                    // so add a check
                    if (! willCauseRecursiveResize (w, h))
                        updateSizeFromEditor (w, h);

                    sizeCheckCount = 0;
                }
            }
        */
    }

    /**
       hooks to get keyboard events from VST
       windows..
      */
    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[CALLBACK]
    #[cfg(target_os="windows")]
    pub fn vst_hook_wnd_proc(
        hw:      HWND,
        message: u32,
        w_param: WPARAM,
        l_param: LPARAM) -> LRESULT {
        
        todo!();
        /*
            for (int i = activeVSTWindows.size(); --i >= 0;)
            {
                Component::SafePointer<VSTPluginWindow> w (activeVSTWindows[i]);

                if (w != nullptr && w->pluginHWND == hW)
                {
                    if (message == WM_CHAR
                        || message == WM_KEYDOWN
                        || message == WM_SYSKEYDOWN
                        || message == WM_KEYUP
                        || message == WM_SYSKEYUP
                        || message == WM_APPCOMMAND)
                    {
                        SendMessage ((HWND) w->getTopLevelComponent()->getWindowHandle(),
                                     message, wParam, lParam);
                    }

                    if (w != nullptr) // (may have been deleted in SendMessage callback)
                        return CallWindowProc ((WNDPROC) w->originalWndProc,
                                               (HWND) w->pluginHWND,
                                               message, wParam, lParam);
                }
            }

            return DefWindowProc (hW, message, wParam, lParam);
        */
    }

    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(any(target_os="linux",target_os="bsd"))]
    pub fn update_plugin_window_handle(&mut self)  {
        
        todo!();
        /*
            pluginWindow = getChildWindow ((Window) getWindowHandle());
        */
    }

    #[cfg(not(any(target_os="ios",target_os="android")))]
    #[cfg(target_os="macos")]
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            #if ALOE_SUPPORT_CARBON
              if (carbonWrapper != nullptr)
                  carbonWrapper->setSize (getWidth(), getHeight());
             #endif

              if (cocoaWrapper != nullptr)
                  cocoaWrapper->setSize (getWidth(), getHeight());
        */
    }
}
