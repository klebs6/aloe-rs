crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AloeVst3Editor<'a> {
    base:  VstEditorView,
    base3: Timer,

    library_initialiser: ScopedAloeInitialiser_GUI,
    editor_host_context: AloeVst3EditControllerEditorHostContext<'a>,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    message_thread:      SharedResourcePointer<MessageThread>,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    event_handler:       SharedResourcePointer<VstEventHandler>,

    owner:               VstComSmartPtr<AloeVst3EditController<'a>>,
    plugin_instance:     &'a mut dyn AudioProcessorInterface,

    #[cfg(any(target_os="linux",target_os="bsd"))]
    component: Box<AloeVst3EditorContentWrapperComponent,AloeVst3EditorMessageManagerLockedDeleter>,

    #[cfg(not(any(target_os="linux",target_os="bsd")))]
    component: Box<AloeVst3EditorContentWrapperComponent<'a>>,

    ///------------------------
    #[cfg(target_os="macos")]
    mac_host_window:     *mut c_void, // default = nullptr

    #[cfg(target_os="macos")]
    is_ns_view:          bool, // default = false

    #[cfg(target_os="macos")]
    cubase_10workaround: Box<AloeVst3EditorCubase10WindowResizeWorkaround<'a>>,

    ///------------------------
    #[cfg(not(target_os="macos"))]
    editor_scale_factor: f32, // default = 1.0f

    #[cfg(not(target_os="macos"))]
    #[cfg(target_os="windows")]
    hooks: WindowsHooks,
}

refcount_methods!{
    VstEditorView
}

impl<'a> IPlugViewContentScaleSupport for AloeVst3Editor<'a> {

    fn set_content_scale_factor(&mut self, _: <Self as aloe_vst_plugview::IPlugViewContentScaleSupport>::ScaleFactor) -> i32 { todo!() }
}

impl<'a> FUnknown for AloeVst3Editor<'a> {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl<'a> AloeVst3Editor<'a> {

    pub fn new(
        ec: &mut AloeVst3EditController,
        p:  &mut AloeAudioProcessor) -> Self {
    
        todo!();
        /*


            : EditorView (&ec, nullptr),
                  editorHostContext (p, ec.getComponentHandler(), this),
                  owner (&ec),
                  pluginInstance (*p.get())
                createContentWrapperComponentIfNeeded();

               #if ALOE_MAC
                if (getHostType().type == PluginHostType::SteinbergCubase10)
                    cubase10Workaround.reset (new AloeVst3EditorCubase10WindowResizeWorkaround (*this));
               #else
                if (! approximatelyEqual (editorScaleFactor, ec.lastScaleFactorReceived))
                    setContentScaleFactor (ec.lastScaleFactorReceived);
               #endif
        */
    }

    #[PLUGIN_API]
    pub fn query_interface(&mut self, 
        targetiid: TUID,
        obj:       *mut *mut c_void) -> tresult {
        
        todo!();
        /*
            const auto result = testFor (*this, targetIID, UniqueBase<IPlugViewContentScaleSupport>{});

                if (result.isOk())
                    return result.extract (obj);

                return VstEditorView::queryInterface (targetIID, obj);
        */
    }
        
    #[PLUGIN_API]
    pub fn is_platform_type_supported(&mut self, ty: FIDString) -> tresult {
        
        todo!();
        /*
            if (type != nullptr && pluginInstance.hasEditor())
                {
                   #if ALOE_WINDOWS
                    if (strcmp (type, kPlatformTypeHWND) == 0)
                   #elif ALOE_MAC
                    if (strcmp (type, kPlatformTypeNSView) == 0 || strcmp (type, kPlatformTypeHIView) == 0)
                   #elif ALOE_LINUX || ALOE_BSD
                    if (strcmp (type, kPlatformTypeX11EmbedWindowID) == 0)
                   #endif
                        return kResultTrue;
                }

                return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn attached(&mut self, 
        parent: *mut c_void,
        ty:     FIDString) -> tresult {
        
        todo!();
        /*
            if (parent == nullptr || isPlatformTypeSupported (type) == kResultFalse)
                    return kResultFalse;

               #if ALOE_LINUX || ALOE_BSD
                eventHandler->registerHandlerForFrame (plugFrame);
               #endif

                systemWindow = parent;

                createContentWrapperComponentIfNeeded();

               #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
                component->setOpaque (true);
                component->addToDesktop (0, (void*) systemWindow);
                component->setVisible (true);

                #if ALOE_WINDOWS && ALOE_WIN_PER_MONITOR_DPI_AWARE
                 component->checkHostWindowScaleFactor();
                 component->startTimer (500);
                #endif

               #else
                isNSView = (strcmp (type, kPlatformTypeNSView) == 0);
                macHostWindow = attachComponentToWindowRefVst (component.get(), parent, isNSView);
               #endif

                component->resizeHostWindow();
                attachedToParent();

                // Life's too short to faff around with wave lab
                if (getHostType().isWavelab())
                    startTimer (200);

                return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn removed(&mut self) -> tresult {
        
        todo!();
        /*
            if (component != nullptr)
                {
                   #if ALOE_WINDOWS
                    component->removeFromDesktop();
                   #elif ALOE_MAC
                    if (macHostWindow != nullptr)
                    {
                        detachComponentFromWindowRefVst (component.get(), macHostWindow, isNSView);
                        macHostWindow = nullptr;
                    }
                   #endif

                    component = nullptr;
                }

               #if ALOE_LINUX || ALOE_BSD
                eventHandler->unregisterHandlerForFrame (plugFrame);
               #endif

                return CPluginView::removed();
        */
    }

    #[PLUGIN_API]
    pub fn on_size(&mut self, new_size: *mut ViewRect) -> tresult {
        
        todo!();
        /*
            if (newSize != nullptr)
                {
                    rect = convertFromHostBounds (*newSize);

                    if (component != nullptr)
                    {
                        component->setSize (rect.getWidth(), rect.getHeight());

                       #if ALOE_MAC
                        if (cubase10Workaround != nullptr)
                        {
                            cubase10Workaround->triggerAsyncUpdate();
                        }
                        else
                       #endif
                        {
                            if (auto* peer = component->getPeer())
                                peer->updateBounds();
                        }
                    }

                    return kResultTrue;
                }

                jassertfalse;
                return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn get_size(&mut self, size: *mut ViewRect) -> tresult {
        
        todo!();
        /*
            #if ALOE_WINDOWS && ALOE_WIN_PER_MONITOR_DPI_AWARE
                if (getHostType().isAbletonLive() && systemWindow == nullptr)
                    return kResultFalse;
               #endif

                if (size != nullptr && component != nullptr)
                {
                    auto editorBounds = component->getSizeToContainChild();

                    *size = convertToHostBounds ({ 0, 0, editorBounds.getWidth(), editorBounds.getHeight() });
                    return kResultTrue;
                }

                return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn can_resize(&mut self) -> tresult {
        
        todo!();
        /*
            if (component != nullptr)
                    if (auto* editor = component->pluginEditor.get())
                        if (editor->isResizable())
                            return kResultTrue;

                return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn check_size_constraint(&mut self, rect_to_check: *mut ViewRect) -> tresult {
        
        todo!();
        /*
            if (rectToCheck != nullptr && component != nullptr)
                {
                    if (auto* editor = component->pluginEditor.get())
                    {
                        if (auto* constrainer = editor->getConstrainer())
                        {
                            *rectToCheck = convertFromHostBounds (*rectToCheck);

                            auto editorBounds = editor->getLocalArea (component.get(),
                                                                      Rectangle<int>::leftTopRightBottom (rectToCheck->left, rectToCheck->top,
                                                                                                          rectToCheck->right, rectToCheck->bottom).toFloat());

                            auto minW = (float) constrainer->getMinimumWidth();
                            auto maxW = (float) constrainer->getMaximumWidth();
                            auto minH = (float) constrainer->getMinimumHeight();
                            auto maxH = (float) constrainer->getMaximumHeight();

                            auto width  = jlimit (minW, maxW, editorBounds.getWidth());
                            auto height = jlimit (minH, maxH, editorBounds.getHeight());

                            auto aspectRatio = (float) constrainer->getFixedAspectRatio();

                            if (aspectRatio != 0.0)
                            {
                                bool adjustWidth = (width / height > aspectRatio);

                                if (getHostType().type == PluginHostType::SteinbergCubase9)
                                {
                                    auto currentEditorBounds = editor->getBounds().toFloat();

                                    if (currentEditorBounds.getWidth() == width && currentEditorBounds.getHeight() != height)
                                        adjustWidth = true;
                                    else if (currentEditorBounds.getHeight() == height && currentEditorBounds.getWidth() != width)
                                        adjustWidth = false;
                                }

                                if (adjustWidth)
                                {
                                    width = height * aspectRatio;

                                    if (width > maxW || width < minW)
                                    {
                                        width = jlimit (minW, maxW, width);
                                        height = width / aspectRatio;
                                    }
                                }
                                else
                                {
                                    height = width / aspectRatio;

                                    if (height > maxH || height < minH)
                                    {
                                        height = jlimit (minH, maxH, height);
                                        width = height * aspectRatio;
                                    }
                                }
                            }

                            auto constrainedRect = component->getLocalArea (editor, Rectangle<float> (width, height))
                                                      .getSmallestIntegerContainer();

                            rectToCheck->right  = rectToCheck->left + roundToInt (constrainedRect.getWidth());
                            rectToCheck->bottom = rectToCheck->top  + roundToInt (constrainedRect.getHeight());

                            *rectToCheck = convertToHostBounds (*rectToCheck);
                        }
                    }

                    return kResultTrue;
                }

                jassertfalse;
                return kResultFalse;
        */
    }

    #[PLUGIN_API]
    pub fn set_content_scale_factor(
        &mut self, 
        factor: <aloe_vst3_editor::AloeVst3Editor<'_> as aloe_vst_plugview::IPlugViewContentScaleSupport>::ScaleFactor

    ) -> tresult {
        
        todo!();
        /*
            #if ! ALOE_MAC
                if (! approximatelyEqual ((float) factor, editorScaleFactor))
                {
                   #if ALOE_WINDOWS && ALOE_WIN_PER_MONITOR_DPI_AWARE
                    // Cubase 10 only sends integer scale factors, so correct this for fractional scales
                    if (getHostType().type == PluginHostType::SteinbergCubase10)
                    {
                        auto hostWindowScale = (IPlugViewContentScaleSupport::ScaleFactor) getScaleFactorForWindow ((HWND) systemWindow);

                        if (hostWindowScale > 0.0 && ! approximatelyEqual (factor, hostWindowScale))
                            factor = hostWindowScale;
                    }
                   #endif

                    editorScaleFactor = (float) factor;

                    if (owner != nullptr)
                        owner->lastScaleFactorReceived = editorScaleFactor;

                    if (component != nullptr)
                        component->setEditorScaleFactor (editorScaleFactor);
                }

                return kResultTrue;
               #else
                ignoreUnused (factor);
                return kResultFalse;
               #endif
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

                ViewRect viewRect;
                getSize (&viewRect);
                onSize (&viewRect);
        */
    }
    
    pub fn convert_to_host_bounds(plugin_rect: ViewRect) -> ViewRect {
        
        todo!();
        /*
            auto desktopScale = Desktop::getInstance().getGlobalScaleFactor();

                if (approximatelyEqual (desktopScale, 1.0f))
                    return pluginRect;

                return { roundToInt ((float) pluginRect.left   * desktopScale),
                         roundToInt ((float) pluginRect.top    * desktopScale),
                         roundToInt ((float) pluginRect.right  * desktopScale),
                         roundToInt ((float) pluginRect.bottom * desktopScale) };
        */
    }
    
    pub fn convert_from_host_bounds(host_rect: ViewRect) -> ViewRect {
        
        todo!();
        /*
            auto desktopScale = Desktop::getInstance().getGlobalScaleFactor();

                if (approximatelyEqual (desktopScale, 1.0f))
                    return hostRect;

                return { roundToInt ((float) hostRect.left   / desktopScale),
                         roundToInt ((float) hostRect.top    / desktopScale),
                         roundToInt ((float) hostRect.right  / desktopScale),
                         roundToInt ((float) hostRect.bottom / desktopScale) };
        */
    }
    
    pub fn create_content_wrapper_component_if_needed(&mut self)  {
        
        todo!();
        /*
            if (component == nullptr)
                {
                   #if ALOE_LINUX || ALOE_BSD
                    const MessageManagerLock mmLock;
                   #endif

                    component.reset (new AloeVst3EditorContentWrapperComponent (*this));
                    component->createEditor (pluginInstance);
                }
        */
    }
}
