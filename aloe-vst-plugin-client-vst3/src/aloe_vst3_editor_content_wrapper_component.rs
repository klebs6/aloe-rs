crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AloeVst3EditorContentWrapperComponent<'a> {
    base:                 Component,

    #[cfg(all(target_os="windows",ALOE_WIN_PER_MONITOR_DPI_AWARE))]
    base2:                Timer,

    plugin_editor:        Box<AudioProcessorEditor<'a>>,
    owner:                &'a mut AloeVst3Editor<'a>,
    fake_mouse_generator: aloe_component::FakeMouseMoveGenerator<'a>,
    last_bounds:          Rectangle<i32>,
    resizing_child:       bool, // default = false
    resizing_parent:      bool, // default = false
}

impl<'a> Drop for AloeVst3EditorContentWrapperComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (pluginEditor != nullptr)
                {
                    PopupMenu::dismissAllActiveMenus();
                    pluginEditor->processor.editorBeingDeleted (pluginEditor.get());
                }
        */
    }
}

impl<'a> AloeVst3EditorContentWrapperComponent<'a> {

    pub fn new(editor: &mut AloeVst3Editor) -> Self {
    
        todo!();
        /*
        : owner(editor),

            setOpaque (true);
                setBroughtToFrontOnMouseClick (true);

                ignoreUnused (fakeMouseGenerator);
        */
    }
    
    pub fn create_editor(&mut self, plugin: &mut dyn AudioProcessorInterface)  {
        
        todo!();
        /*
            pluginEditor.reset (plugin.createEditorIfNeeded());

                if (pluginEditor != nullptr)
                {
                    pluginEditor->setHostContext (&owner.editorHostContext);

                    addAndMakeVisible (pluginEditor.get());
                    pluginEditor->setTopLeftPosition (0, 0);

                    lastBounds = getSizeToContainChild();

                    {
                        const ScopedValueSetter<bool> resizingParentSetter (resizingParent, true);
                        setBounds (lastBounds);
                    }

                    resizeHostWindow();
                }
                else
                {
                    // if hasEditor() returns true then createEditorIfNeeded has to return a valid editor
                    jassertfalse;
                }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);
        */
    }
    
    pub fn get_size_to_contain_child(&mut self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (pluginEditor != nullptr)
                    return getLocalArea (pluginEditor.get(), pluginEditor->getLocalBounds());

                return {};
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
                    resizeHostWindow();

                   #if ALOE_LINUX || ALOE_BSD
                    if (getHostType().isBitwigStudio())
                        repaint();
                   #endif

                    lastBounds = newBounds;
                }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (pluginEditor != nullptr)
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
                }
        */
    }
    
    pub fn parent_size_changed(&mut self)  {
        
        todo!();
        /*
            if (pluginEditor != nullptr)
                {
                    resizeHostWindow();
                    pluginEditor->repaint();
                }
        */
    }
    
    pub fn resize_host_window(&mut self)  {
        
        todo!();
        /*
            if (pluginEditor != nullptr)
                {
                    if (owner.plugFrame != nullptr)
                    {
                        auto editorBounds = getSizeToContainChild();
                        auto newSize = convertToHostBounds ({ 0, 0, editorBounds.getWidth(), editorBounds.getHeight() });

                        {
                            const ScopedValueSetter<bool> resizingParentSetter (resizingParent, true);
                            owner.plugFrame->resizeView (&owner, &newSize);
                        }

                        auto host = getHostType();

                       #if ALOE_MAC
                        if (host.isWavelab() || host.isReaper())
                       #else
                        if (host.isWavelab() || host.isAbletonLive() || host.isBitwigStudio())
                       #endif
                            setBounds (editorBounds.withPosition (0, 0));
                    }
                }
        */
    }
    
    pub fn set_editor_scale_factor(&mut self, scale: f32)  {
        
        todo!();
        /*
            if (pluginEditor != nullptr)
                {
                    auto prevEditorBounds = pluginEditor->getLocalArea (this, lastBounds);

                    {
                        const ScopedValueSetter<bool> resizingChildSetter (resizingChild, true);

                        pluginEditor->setScaleFactor (scale);
                        pluginEditor->setBounds (prevEditorBounds.withPosition (0, 0));
                    }

                    lastBounds = getSizeToContainChild();

                    resizeHostWindow();
                    repaint();
                }
        */
    }

    #[cfg(all(target_os="windows",ALOE_WIN_PER_MONITOR_DPI_AWARE))]
    pub fn check_host_window_scale_factor(&mut self)  {
        
        todo!();
        /*
            auto hostWindowScale = (float) getScaleFactorForWindow ((HWND) owner.systemWindow);

                if (hostWindowScale > 0.0 && ! approximatelyEqual (hostWindowScale, owner.editorScaleFactor))
                    owner.setContentScaleFactor (hostWindowScale);
        */
    }
    
    #[cfg(all(target_os="windows",ALOE_WIN_PER_MONITOR_DPI_AWARE))]
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            checkHostWindowScaleFactor();
        */
    }
}
