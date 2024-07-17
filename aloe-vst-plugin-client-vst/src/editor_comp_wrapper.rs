crate::ix!();

//TODO
pub struct Vst2ERect                 {}
pub struct Vst2AudioMasterCallback   {}
pub struct Vst2AEffect               {}
pub struct Vst2VstSpeakerArrangement {}
pub struct Vst2VstPinProperties      {}

/**
  | A component to hold the
  | AudioProcessorEditor, and cope with some
  | housekeeping chores when it changes or
  | repaints.
  */
#[no_copy]
#[leak_detector]
pub struct AloeVSTWrapperEditorCompWrapper<'a> {
    base:                 Component<'a>,

    #[cfg(all(target_os="windows",ALOE_WIN_PER_MONITOR_DPI_AWARE))]
    base2:                Timer,

    wrapper:              &'a mut AloeVSTWrapper<'a>,
    fake_mouse_generator: FakeMouseMoveGenerator<'a>,
    resizing_child:       bool, // default = false
    resizing_parent:      bool, // default = false
    editor_scale_factor:  f32, // default = 1.0f
    last_bounds:          Rectangle<i32>,

    /**
       = XWindowSystem::getInstance()->getDisplay();
      */
    #[cfg(any(target_os="linux",target_os="bsd"))]
    display:              *mut Display,


    #[cfg(target_os="windows")]
    hooks:                WindowsHooks,

    host_window:          AloeVSTWrapperHostWindowType,
}

impl<'a> Drop for AloeVSTWrapperEditorCompWrapper<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            deleteAllChildren(); // note that we can't use a std::unique_ptr because the editor may
                                     // have been transferred to another parent which takes over ownership.
        */
    }
}

impl<'a> AloeVSTWrapperEditorCompWrapper<'a> {

    pub fn new(
        w:      &mut AloeVSTWrapper,
        editor: &mut AudioProcessorEditor) -> Self {
    
        todo!();
        /*
        : wrapper(w),

            editor.setOpaque (true);
                addAndMakeVisible (editor);

                auto editorBounds = getSizeToContainChild();
                setSize (editorBounds.getWidth(), editorBounds.getHeight());

               #if ALOE_WINDOWS
                if (! getHostType().isReceptor())
                    addMouseListener (this, true);
               #endif

                setOpaque (true);
                ignoreUnused (fakeMouseGenerator);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);
        */
    }
    
    pub fn get_editor_bounds(&mut self, bounds: &mut Vst2ERect)  {
        
        todo!();
        /*
            auto editorBounds = getSizeToContainChild();
                bounds = convertToHostBounds ({ 0, 0, (int16) editorBounds.getHeight(), (int16) editorBounds.getWidth() });
        */
    }
    
    pub fn attach_to_host(&mut self, args: VstOpCodeArguments)  {
        
        todo!();
        /*
            setVisible (false);

               #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
                addToDesktop (0, args.ptr);
                hostWindow = (AloeVSTWrapperHostWindowType) args.ptr;

                #if ALOE_LINUX || ALOE_BSD
                 X11Symbols::getInstance()->xReparentWindow (display,
                                                             (Window) getWindowHandle(),
                                                             (AloeVSTWrapperHostWindowType) hostWindow,
                                                             0, 0);
                #elif ALOE_WINDOWS && ALOE_WIN_PER_MONITOR_DPI_AWARE
                 checkHostWindowScaleFactor();
                 startTimer (500);
                #endif
               #elif ALOE_MAC
                hostWindow = attachComponentToWindowRefVST (this, args.ptr, wrapper.useNSView);
               #endif

                setVisible (true);
        */
    }
    
    pub fn detach_host_window(&mut self)  {
        
        todo!();
        /*
            #if ALOE_MAC
                if (hostWindow != nullptr)
                    detachComponentFromWindowRefVST (this, hostWindow, wrapper.useNSView);
               #endif

                hostWindow = {};
        */
    }
    
    pub fn check_visibility(&mut self)  {
        
        todo!();
        /*
            #if ALOE_MAC
                if (hostWindow != nullptr)
                    checkWindowVisibilityVST (hostWindow, this, wrapper.useNSView);
               #endif
        */
    }
    
    pub fn get_editor_comp(&self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return dynamic_cast<AudioProcessorEditor*> (getChildComponent (0));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (auto* pluginEditor = getEditorComp())
                {
                    if (! resizingParent)
                    {
                        auto newBounds = getLocalBounds();

                        {
                            const ScopedValueSetter<bool> resizingChildSetter (resizingChild, true);
                            pluginEditor->setBounds (pluginEditor->getLocalArea (this, newBounds).withPosition (0, 0));
                        }

                        lastBounds = newBounds;
                    }

                    updateWindowSize();
                }

               #if ALOE_MAC && ! ALOE_64BIT
                if (! wrapper.useNSView)
                    updateEditorCompBoundsVST (this);
               #endif
        */
    }
    
    pub fn parent_size_changed(&mut self)  {
        
        todo!();
        /*
            updateWindowSize();
        */
    }
    
    pub fn child_bounds_changed(&mut self, _0: *mut Component)  {
        
        todo!();
        /*
            if (resizingChild)
                    return;

                auto newBounds = getSizeToContainChild();

                if (newBounds != lastBounds)
                {
                    updateWindowSize();
                    lastBounds = newBounds;
                }
        */
    }
    
    pub fn get_size_to_contain_child(&mut self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (auto* pluginEditor = getEditorComp())
                    return getLocalArea (pluginEditor, pluginEditor->getLocalBounds());

                return {};
        */
    }
    
    pub fn update_window_size(&mut self)  {
        
        todo!();
        /*
            if (! resizingParent
                    && getEditorComp() != nullptr
                    && hostWindow != AloeVSTWrapperHostWindowType{})
                {
                    auto editorBounds = getSizeToContainChild();

                    resizeHostWindow (editorBounds.getWidth(), editorBounds.getHeight());

                    {
                        const ScopedValueSetter<bool> resizingParentSetter (resizingParent, true);

                       #if ALOE_LINUX || ALOE_BSD // setSize() on linux causes renoise and energyxt to fail.
                        auto rect = convertToHostBounds ({ 0, 0, (int16) editorBounds.getHeight(), (int16) editorBounds.getWidth() });

                        X11Symbols::getInstance()->xResizeWindow (display, (Window) getWindowHandle(),
                                                                  static_cast<unsigned int> (rect.right - rect.left),
                                                                  static_cast<unsigned int> (rect.bottom - rect.top));
                       #else
                        setSize (editorBounds.getWidth(), editorBounds.getHeight());
                       #endif
                    }

                   #if ALOE_MAC
                    resizeHostWindow (editorBounds.getWidth(), editorBounds.getHeight()); // (doing this a second time seems to be necessary in tracktion)
                   #endif
                }
        */
    }
    
    pub fn resize_host_window(&mut self, 
        new_width:  i32,
        new_height: i32)  {
        
        todo!();
        /*
            auto rect = convertToHostBounds ({ 0, 0, (int16) newHeight, (int16) newWidth });
                newWidth = rect.right - rect.left;
                newHeight = rect.bottom - rect.top;

                bool sizeWasSuccessful = false;

                if (auto host = wrapper.hostCallback)
                {
                    auto status = host (wrapper.getAEffect(), Vst2::audioMasterCanDo, 0, 0, const_cast<char*> ("sizeWindow"), 0);

                    if (status == (pointer_sized_int) 1 || getHostType().isAbletonLive())
                    {
                        const ScopedValueSetter<bool> resizingParentSetter (resizingParent, true);

                        sizeWasSuccessful = (host (wrapper.getAEffect(), Vst2::audioMasterSizeWindow,
                                                   newWidth, newHeight, nullptr, 0) != 0);
                    }
                }

                // some hosts don't support the sizeWindow call, so do it manually..
                if (! sizeWasSuccessful)
                {
                    const ScopedValueSetter<bool> resizingParentSetter (resizingParent, true);

                   #if ALOE_MAC
                    setNativeHostWindowSizeVST (hostWindow, this, newWidth, newHeight, wrapper.useNSView);
                   #elif ALOE_LINUX || ALOE_BSD
                    // (Currently, all linux hosts support sizeWindow, so this should never need to happen)
                    setSize (newWidth, newHeight);
                   #else
                    int dw = 0;
                    int dh = 0;
                    const int frameThickness = GetSystemMetrics (SM_CYFIXEDFRAME);

                    HWND w = (HWND) getWindowHandle();

                    while (w != nullptr)
                    {
                        HWND parent = getWindowParent (w);

                        if (parent == nullptr)
                            break;

                        TCHAR windowType [32] = { 0 };
                        GetClassName (parent, windowType, 31);

                        if (String (windowType).equalsIgnoreCase ("MDIClient"))
                            break;

                        RECT windowPos, parentPos;
                        GetWindowRect (w, &windowPos);
                        GetWindowRect (parent, &parentPos);

                        if (w != (HWND) getWindowHandle())
                            SetWindowPos (w, nullptr, 0, 0, newWidth + dw, newHeight + dh,
                                          SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOOWNERZORDER);

                        dw = (parentPos.right - parentPos.left) - (windowPos.right - windowPos.left);
                        dh = (parentPos.bottom - parentPos.top) - (windowPos.bottom - windowPos.top);

                        w = parent;

                        if (dw == 2 * frameThickness)
                            break;

                        if (dw > 100 || dh > 100)
                            w = nullptr;
                    }

                    if (w != nullptr)
                        SetWindowPos (w, nullptr, 0, 0, newWidth + dw, newHeight + dh,
                                      SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOOWNERZORDER);
                   #endif
                }
        */
    }
    
    pub fn set_content_scale_factor(&mut self, scale: f32)  {
        
        todo!();
        /*
            if (! approximatelyEqual (scale, editorScaleFactor))
                {
                    editorScaleFactor = scale;

                    if (auto* pluginEditor = getEditorComp())
                    {
                        auto prevEditorBounds = pluginEditor->getLocalArea (this, lastBounds);

                        {
                            const ScopedValueSetter<bool> resizingChildSetter (resizingChild, true);

                            pluginEditor->setScaleFactor (editorScaleFactor);
                            pluginEditor->setBounds (prevEditorBounds.withPosition (0, 0));
                        }

                        lastBounds = getSizeToContainChild();
                        updateWindowSize();
                    }
                }
        */
    }

    #[cfg(target_os="windows")]
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            broughtToFront();
        */
    }
    
    #[cfg(target_os="windows")]
    pub fn brought_to_front(&mut self)  {
        
        todo!();
        /*
            // for hosts like nuendo, need to also pop the MDI container to the
                // front when our comp is clicked on.
                if (! isCurrentlyBlockedByAnotherModalComponent())
                    if (HWND parent = findMDIParentOf ((HWND) getWindowHandle()))
                        SetWindowPos (parent, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
        */
    }

    #[cfg(target_os="windows")]
    #[cfg(ALOE_WIN_PER_MONITOR_DPI_AWARE)]
    pub fn check_host_window_scale_factor(&mut self)  {
        
        todo!();
        /*
            auto hostWindowScale = (float) getScaleFactorForWindow ((AloeVSTWrapperHostWindowType) hostWindow);

                 if (hostWindowScale > 0.0f && ! approximatelyEqual (hostWindowScale, editorScaleFactor))
                     wrapper.handleSetContentScaleFactor (hostWindowScale);
        */
    }
    
    #[cfg(target_os="windows")]
    #[cfg(ALOE_WIN_PER_MONITOR_DPI_AWARE)]
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            checkHostWindowScaleFactor();
        */
    }

    #[cfg(target_os="macos")]
    pub fn key_pressed(&mut self, _0: &KeyPress) -> bool {
        
        todo!();
        /*
            // If we have an unused keypress, move the key-focus to a host window
                // and re-inject the event..
                return forwardCurrentKeyEventToHostVST (this, wrapper.useNSView);
        */
    }
    
    pub fn convert_to_host_bounds(rect: &Vst2ERect) -> Vst2ERect {
        
        todo!();
        /*
            auto desktopScale = Desktop::getInstance().getGlobalScaleFactor();

                if (approximatelyEqual (desktopScale, 1.0f))
                    return rect;

                return { (int16) roundToInt (rect.top    * desktopScale),
                         (int16) roundToInt (rect.left   * desktopScale),
                         (int16) roundToInt (rect.bottom * desktopScale),
                         (int16) roundToInt (rect.right  * desktopScale) };
        */
    }
}
