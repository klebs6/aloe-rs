crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ConcertinaPanelHolder<'a> {
    base:                    Component<'a>,
    component:               OptionalScopedPointer<Component<'a>>,
    drag_start_sizes:        ConcertinaPanelSizes<'a>,
    mouse_downy:             i32,
    custom_header_component: OptionalScopedPointer<Component<'a>>,
}

impl<'a> ConcertinaPanelHolder<'a> {

    pub fn new(
        comp:           *mut Component<'a>,
        take_ownership: bool) -> Self {
    
        todo!();
        /*
        : component(comp, takeOwnership),

            setRepaintsOnMouseActivity (true);
                setWantsKeyboardFocus (false);
                addAndMakeVisible (comp);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (customHeaderComponent == nullptr)
                {
                    const Rectangle<int> area (getWidth(), getHeaderSize());
                    g.reduceClipRegion (area);

                    getLookAndFeel().drawConcertinaPanelHeader (g, area, isMouseOver(), isMouseButtonDown(),
                                                                getPanel(), *component);
                }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();
                auto headerBounds = bounds.removeFromTop (getHeaderSize());

                if (customHeaderComponent != nullptr)
                    customHeaderComponent->setBounds (headerBounds);

                component->setBounds (bounds);
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            mouseDownY = getY();
                dragStartSizes = getPanel().getFittedSizes();
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.mouseWasDraggedSinceMouseDown())
                {
                    auto& panel = getPanel();
                    panel.setLayout (dragStartSizes.withMovedPanel (panel.holders.indexOf (this),
                                                                    mouseDownY + e.getDistanceFromDragStartY(),
                                                                    panel.getHeight()), false);
                }
        */
    }
    
    pub fn mouse_double_click(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            getPanel().panelHeaderDoubleClicked (component);
        */
    }
    
    pub fn set_custom_header_component(&mut self, 
        header_component:      *mut Component<'a>,
        should_take_ownership: bool)  {
        
        todo!();
        /*
            customHeaderComponent.set (headerComponent, shouldTakeOwnership);

                if (headerComponent != nullptr)
                {
                    addAndMakeVisible (customHeaderComponent);
                    customHeaderComponent->addMouseListener (this, false);
                }
        */
    }
    
    pub fn get_header_size(&self) -> i32 {
        
        todo!();
        /*
            ConcertinaPanel& panel = getPanel();
                auto ourIndex = panel.holders.indexOf (this);
                return panel.currentSizes->get(ourIndex).minSize;
        */
    }
    
    pub fn get_panel(&self) -> &mut ConcertinaPanel {
        
        todo!();
        /*
            auto panel = dynamic_cast<ConcertinaPanel*> (getParentComponent());
                jassert (panel != nullptr);
                return *panel;
        */
    }
}

