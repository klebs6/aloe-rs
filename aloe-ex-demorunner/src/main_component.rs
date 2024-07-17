crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/UI/MainComponent.h]

#[no_copy]
#[leak_detector]
pub struct MainComponent<'a> {
    base:                         Component<'a>,
    content_component:            Box<DemoContentComponent<'a>>,
    demos_panel:                  SidePanel<'a>, // { "Demos", 250, true };
    open_gl_context:              OpenGLContext<'a>,
    peer:                         *mut ComponentPeer<'a>, // default = nullptr
    rendering_engines:            StringArray,
    current_rendering_engine_idx: i32, // default = -1
    show_demos_button:            TextButton<'a>, // default = "Browse Demos" 
    is_showing_heavyweight_demo:  bool, // default = false
    side_panel_width:             i32, // default = 0
}

impl<'a> Drop for MainComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        contentComponent->clearCurrentDemo();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/UI/MainComponent.cpp]
impl<'a> Default for MainComponent<'a> {

     fn default() -> Self {
    
        todo!();
        /*

            contentComponent.reset (new DemoContentComponent (*this, [this] (bool isHeavyweight)
        {
            demosPanel.showOrHide (false);

            if (isHeavyweight)
            {
               #if ALOE_MAC && USE_COREGRAPHICS_RENDERING
                setRenderingEngine (1);
               #else
                setRenderingEngine (0);
               #endif
            }

            isShowingHeavyweightDemo = isHeavyweight;
            resized();
        }));

        demosPanel.setContent (new DemoList (*contentComponent));
        demosPanel.setTitleBarComponent (new SidePanelHeader (*this), true);

        addAndMakeVisible (contentComponent.get());
        addAndMakeVisible (showDemosButton);
        addAndMakeVisible (demosPanel);

        demosPanel.setTitle ("Demos");
        demosPanel.setFocusContainerType (FocusContainerType::focusContainer);

        showDemosButton.onClick = [this] { demosPanel.showOrHide (true); };

        demosPanel.onPanelMove = [this]
        {
            sidePanelWidth = jmax (0, demosPanel.getRight());

            if (isShowingHeavyweightDemo)
                resized();
        };

        demosPanel.onPanelShowHide = [this] (bool isShowing)
        {
            if (isShowing)
            {
                sidePanelWidth = jmax (0, demosPanel.getWidth());

                if (isShowingHeavyweightDemo)
                    resized();

                if (auto* handler = demosPanel.getAccessibilityHandler())
                    handler->grabFocus();
            }
            else
            {
                sidePanelWidth = 0;

                if (isShowingHeavyweightDemo)
                    Timer::callAfterDelay (250, [this] { resized(); });
            }
        };

        contentComponent->showHomeScreen();

        setOpaque (true);
        setSize (800, 800);
        */
    }
}
    
impl<'a> MainComponent<'a> {
   
    pub fn get_side_panel(&mut self) -> &mut SidePanel {
        
        todo!();
        /*
            return demosPanel;
        */
    }
    
    pub fn get_rendering_engines(&mut self) -> StringArray {
        
        todo!();
        /*
            return renderingEngines;
        */
    }
    
    pub fn get_current_rendering_engine(&mut self) -> i32 {
        
        todo!();
        /*
            return currentRenderingEngineIdx;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto safeBounds = [this]
        {
            auto bounds = getLocalBounds();

            if (auto* display = Desktop::getInstance().getDisplays().getDisplayForRect (getScreenBounds()))
                return display->safeAreaInsets.subtractedFrom (bounds);

            return bounds;
        }();

        showDemosButton.setBounds (safeBounds.getX(), safeBounds.getY(), 150, contentComponent->getTabBarDepth());

        if (isShowingHeavyweightDemo)
        {
            safeBounds.removeFromLeft (sidePanelWidth);
            contentComponent->setTabBarIndent (jmax (0, 150 - sidePanelWidth));
        }
        else
        {
            contentComponent->setTabBarIndent (150);
        }

        contentComponent->setBounds (safeBounds);
        */
    }
    
    pub fn home_button_clicked(&mut self)  {
        
        todo!();
        /*
            if (auto* list = dynamic_cast<DemoList*> (demosPanel.getContent()))
            list->showCategory ({});

        if (contentComponent != nullptr)
        {
            if (contentComponent->isShowingHomeScreen())
                return;

            contentComponent->showHomeScreen();

            if (isShowingHeavyweightDemo)
            {
                isShowingHeavyweightDemo = false;
                resized();
            }
        }
        */
    }
    
    pub fn settings_button_clicked(&mut self)  {
        
        todo!();
        /*
            if (contentComponent != nullptr)
            contentComponent->setCurrentTabIndex (2);
        */
    }
    
    pub fn set_rendering_engine(&mut self, rendering_engine_index: i32)  {
        
        todo!();
        /*
            if (renderingEngineIndex != currentRenderingEngineIdx)
            updateRenderingEngine (renderingEngineIndex);
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            auto* newPeer = getPeer();

        if (peer != newPeer)
        {
            peer = newPeer;

            auto previousRenderingEngine = renderingEngines[currentRenderingEngineIdx];

            renderingEngines.clear();
            if (peer != nullptr)
                renderingEngines = peer->getAvailableRenderingEngines();

            renderingEngines.add ("OpenGL Renderer");

            currentRenderingEngineIdx = renderingEngines.indexOf (previousRenderingEngine);

            if (currentRenderingEngineIdx < 0)
            {
               #if ALOE_ANDROID
                currentRenderingEngineIdx = (renderingEngines.size() - 1);
               #else
                currentRenderingEngineIdx = peer->getCurrentRenderingEngine();
               #endif
            }

            updateRenderingEngine (currentRenderingEngineIdx);
        }
        */
    }
    
    pub fn update_rendering_engine(&mut self, rendering_engine_index: i32)  {
        
        todo!();
        /*
            if (renderingEngineIndex == (renderingEngines.size() - 1))
        {
            if (isShowingHeavyweightDemo)
                return;

            openGLContext.attachTo (*getTopLevelComponent());
        }
        else
        {
            openGLContext.detach();
            peer->setCurrentRenderingEngine (renderingEngineIndex);
        }

        currentRenderingEngineIdx = renderingEngineIndex;
        */
    }
}
