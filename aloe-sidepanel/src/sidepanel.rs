crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_SidePanel.h]

/**
  | A component that is positioned on either
  | the left- or right-hand side of its parent,
  | containing a header and some content.
  | This sort of component is typically
  | used for navigation and forms in mobile
  | applications.
  | 
  | When triggered with the showOrHide()
  | method, the SidePanel will animate
  | itself to its new position. This component
  | also contains some logic to reactively
  | resize and dismiss itself when the user
  | drags it.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct SidePanel<'a> {
    base:                       Component<'a>,

    /**
      | You can assign a lambda to this callback
      | object and it will be called when the
      | panel is moved.
      |
      */
    on_panel_move:              fn() -> (),

    /**
      | You can assign a lambda to this callback object
      | and it will be called when the panel is shown or
      | hidden.
      */
    on_panel_show_hide:         fn(_0: bool) -> (),

    parent:                     *mut Component<'a>, // default = nullptr
    content_component:          OptionalScopedPointer<Component<'a>>,
    title_bar_component:        OptionalScopedPointer<Component<'a>>,
    title_label:                Label<'a>,
    dismiss_button:             ShapeButton<'a>,     //{ "dismissButton", Colours::lightgrey, Colours::lightgrey, Colours::white };
    shadow_area:                Rectangle<i32>,
    is_on_left:                 bool,            // default = false
    is_showing:                 bool,            // default = false
    panel_width:                i32,             // default = 0
    shadow_width:               i32,             // default = 15
    title_bar_height:           i32,             // default = 40
    starting_bounds:            Rectangle<i32>,
    should_resize:              bool,            // default = false
    amount_moved:               i32,             // default = 0
    should_show_dismiss_button: bool,            // default = true
}

impl<'a> ComponentListener          for SidePanel<'a> {}
impl<'a> ComponentEnablementChanged for SidePanel<'a> {}

impl<'a> ComponentMovedOrResized for SidePanel<'a> {

    fn component_moved_or_resized(
        &mut self, 
        component:   &mut Component<'_>,
        was_moved:   bool,
        was_resized: bool

    ) {
        
        todo!();
        /*
            ignoreUnused (wasMoved);

        if (wasResized && (&component == parent))
            setBounds (calculateBoundsInParent (component));
        */
    }
}

impl<'a> ComponentBroughtToFront    for SidePanel<'a> { }
impl<'a> ComponentVisibilityChanged for SidePanel<'a> { }
impl<'a> ComponentChildrenChanged   for SidePanel<'a> { }

impl<'a> ComponentParentHierarchyChanged for SidePanel<'a> { }


impl<'a> ComponentNameChanged  for SidePanel<'a> { }
impl<'a> ComponentBeingDeleted for SidePanel<'a> { }

impl<'a> ChangeListener for SidePanel<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (! Desktop::getInstance().getAnimator().isAnimating (this))
        {
            if (onPanelShowHide != nullptr)
                onPanelShowHide (isShowing);

            if (isVisible() && ! isShowing)
                setVisible (false);
        }
        */
    }
}

impl<'a> Drop for SidePanel<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        auto& desktop = Desktop::getInstance();

        desktop.removeGlobalMouseListener (this);
        desktop.getAnimator().removeChangeListener (this);

        if (parent != nullptr)
            parent->removeComponentListener (this);
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_SidePanel.cpp]
impl<'a> SidePanel<'a> {

    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            auto* newParent = getParentComponent();

        if ((newParent != nullptr) && (parent != newParent))
        {
            if (parent != nullptr)
                parent->removeComponentListener (this);

            parent = newParent;
            parent->addComponentListener (this);
        }
        */
    }

    /**
      | Returns the component that's currently
      | being used inside the SidePanel.
      | 
      | @see setViewedComponent
      |
      */
    pub fn get_content(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return contentComponent.get();
        */
    }

    /**
      | Returns the component that is currently
      | being used as the title bar of the SidePanel.
      | 
      | @see setTitleBarComponent
      |
      */
    pub fn get_title_bar_component(&self) -> *mut Component<'a> {
        
        todo!();
        /*
            return titleBarComponent.get();
        */
    }

    /**
      | Returns true if the SidePanel is currently
      | showing.
      |
      */
    pub fn is_panel_showing(&self) -> bool {
        
        todo!();
        /*
            return isShowing;
        */
    }

    /**
      | Returns true if the SidePanel is positioned
      | on the left of its parent.
      |
      */
    pub fn is_panel_on_left(&self) -> bool {
        
        todo!();
        /*
            return isOnLeft;
        */
    }

    /**
      | Sets the width of the shadow that will
      | be drawn on the side of the panel.
      |
      */
    pub fn set_shadow_width(&mut self, new_width: i32)  {
        
        todo!();
        /*
            shadowWidth = newWidth;
        */
    }

    /**
      | Returns the width of the shadow that
      | will be drawn on the side of the panel.
      |
      */
    pub fn get_shadow_width(&self) -> i32 {
        
        todo!();
        /*
            return shadowWidth;
        */
    }

    /**
      | Sets the height of the title bar at the
      | top of the SidePanel.
      |
      */
    pub fn set_title_bar_height(&mut self, new_height: i32)  {
        
        todo!();
        /*
            titleBarHeight = newHeight;
        */
    }

    /**
      | Returns the height of the title bar at
      | the top of the SidePanel.
      |
      */
    pub fn get_title_bar_height(&self) -> i32 {
        
        todo!();
        /*
            return titleBarHeight;
        */
    }

    /**
      | Returns the text that is displayed in
      | the title bar at the top of the SidePanel.
      |
      */
    pub fn get_title_text(&self) -> String {
        
        todo!();
        /*
            return titleLabel.getText();
        */
    }
    
    /**
      | Creates a SidePanel component.
      | 
      | -----------
      | @param title
      | 
      | the text to use for the SidePanel's title
      | bar
      | ----------
      | @param width
      | 
      | the width of the SidePanel
      | ----------
      | @param positionOnLeft
      | 
      | if true, the SidePanel will be positioned
      | on the left of its parent component and
      | if false, the SidePanel will be positioned
      | on the right of its parent component
      | ----------
      | @param contentComponent
      | 
      | the component to add to this SidePanel
      | - this content will take up the full size
      | of the SidePanel, minus the height of
      | the title bar. You can pass nullptr to
      | this if you like and set the content component
      | later using the setContent() method
      | ----------
      | @param deleteComponentWhenNoLongerNeeded
      | 
      | if true, the component will be deleted
      | automatically when the SidePanel is
      | deleted or when a different component
      | is added. If false, the caller must manage
      | the lifetime of the component
      |
      */
    pub fn new(
        title:                                  &str,
        width:                                  i32,
        position_on_left:                       bool,
        content_to_display:                     *mut Component<'a>,
        delete_component_when_no_longer_needed: Option<bool>

    ) -> Self {

        let delete_component_when_no_longer_needed: bool =
                 delete_component_when_no_longer_needed.unwrap_or(true);
    
        todo!();
        /*
        : title_label("titleLabel", title),
        : is_on_left(positionOnLeft),
        : panel_width(width),

            lookAndFeelChanged();

        addAndMakeVisible (titleLabel);

        dismissButton.onClick = [this] { showOrHide (false); };
        addAndMakeVisible (dismissButton);

        auto& desktop = Desktop::getInstance();

        desktop.addGlobalMouseListener (this);
        desktop.getAnimator().addChangeListener (this);

        if (contentToDisplay != nullptr)
            setContent (contentToDisplay, deleteComponentWhenNoLongerNeeded);

        setOpaque (false);
        setVisible (false);
        setAlwaysOnTop (true);
        */
    }
    
    /**
      | Sets the component that this SidePanel
      | will contain.
      | 
      | This will add the given component to
      | this SidePanel and position it below
      | the title bar.
      | 
      | (Don't add or remove any child components
      | directly using the normal
      | 
      | Component::addChildComponent()
      | methods).
      | 
      | -----------
      | @param newContentComponent
      | 
      | the component to add to this SidePanel,
      | or nullptr to remove the current component.
      | ----------
      | @param deleteComponentWhenNoLongerNeeded
      | 
      | if true, the component will be deleted
      | automatically when the SidePanel is
      | deleted or when a different component
      | is added. If false, the caller must manage
      | the lifetime of the component
      | 
      | @see getContent
      |
      */
    pub fn set_content(
        &mut self, 
        new_content:                            *mut Component<'a>,
        delete_component_when_no_longer_needed: Option<bool>

    ) {

        let delete_component_when_no_longer_needed: bool = delete_component_when_no_longer_needed.unwrap_or(true);
        
        todo!();
        /*
            if (contentComponent.get() != newContent)
        {
            if (deleteComponentWhenNoLongerNeeded)
                contentComponent.setOwned (newContent);
            else
                contentComponent.setNonOwned (newContent);

            addAndMakeVisible (contentComponent);

            resized();
        }
        */
    }
    
    /**
      | Sets a custom component to be used for
      | the title bar of this SidePanel, replacing
      | the default. You can pass a nullptr to
      | revert to the default title bar.
      | 
      | -----------
      | @param titleBarComponentToUse
      | 
      | the component to use as the title bar,
      | or nullptr to use the default
      | ----------
      | @param keepDismissButton
      | 
      | if false the specified component will
      | take up the full width of the title bar
      | including the dismiss button but if
      | true, the default dismiss button will
      | be kept
      | ----------
      | @param deleteComponentWhenNoLongerNeeded
      | 
      | if true, the component will be deleted
      | automatically when the SidePanel is
      | deleted or when a different component
      | is added. If false, the caller must manage
      | the lifetime of the component
      | 
      | @see getTitleBarComponent
      |
      */
    pub fn set_title_bar_component(
        &mut self, 
        title_bar_component_to_use:             *mut Component<'a>,
        keep_dismiss_button:                    bool,
        delete_component_when_no_longer_needed: Option<bool>

    ) {

        let delete_component_when_no_longer_needed: bool = delete_component_when_no_longer_needed.unwrap_or(true);
        
        todo!();
        /*
            if (titleBarComponent.get() != titleBarComponentToUse)
        {
            if (deleteComponentWhenNoLongerNeeded)
                titleBarComponent.setOwned (titleBarComponentToUse);
            else
                titleBarComponent.setNonOwned (titleBarComponentToUse);

            addAndMakeVisible (titleBarComponent);

            resized();
        }

        shouldShowDismissButton = keepDismissButton;
        */
    }
    
    /**
      | Shows or hides the SidePanel.
      | 
      | This will animate the SidePanel to either
      | its full width or to be hidden on the left-
      | or right-hand side of its parent component
      | depending on the value of positionOnLeft
      | that was passed to the constructor.
      | 
      | -----------
      | @param show
      | 
      | if true, this will show the SidePanel
      | and if false the SidePanel will be hidden
      |
      */
    pub fn show_or_hide(&mut self, show: bool)  {
        
        todo!();
        /*
            if (parent != nullptr)
        {
            isShowing = show;

            Desktop::getInstance().getAnimator().animateComponent (this, calculateBoundsInParent (*parent),
                                                                   1.0f, 250, true, 1.0, 0.0);

            if (isShowing && ! isVisible())
                setVisible (true);
        }
        */
    }
    
    pub fn moved(&mut self)  {
        
        todo!();
        /*
            if (onPanelMove != nullptr)
            onPanelMove();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

        calculateAndRemoveShadowBounds (bounds);

        auto titleBounds = bounds.removeFromTop (titleBarHeight);

        if (titleBarComponent != nullptr)
        {
            if (shouldShowDismissButton)
                dismissButton.setBounds (isOnLeft ? titleBounds.removeFromRight (30).withTrimmedRight (10)
                                                  : titleBounds.removeFromLeft  (30).withTrimmedLeft  (10));

            titleBarComponent->setBounds (titleBounds);
        }
        else
        {
            dismissButton.setBounds (isOnLeft ? titleBounds.removeFromRight (30).withTrimmedRight (10)
                                              : titleBounds.removeFromLeft  (30).withTrimmedLeft  (10));

            titleLabel.setBounds (isOnLeft ? titleBounds.withTrimmedRight (40)
                                           : titleBounds.withTrimmedLeft (40));
        }

        if (contentComponent != nullptr)
            contentComponent->setBounds (bounds);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        auto bgColour     = lf.findColour (SidePanel::backgroundColour);
        auto shadowColour = lf.findColour (SidePanel::shadowBaseColour);

        g.setGradientFill (ColourGradient (shadowColour.withAlpha (0.7f), (isOnLeft ? shadowArea.getTopLeft()
                                                                                    : shadowArea.getTopRight()).toFloat(),
                                           shadowColour.withAlpha (0.0f), (isOnLeft ? shadowArea.getTopRight()
                                                                                    : shadowArea.getTopLeft()).toFloat(), false));
        g.fillRect (shadowArea);

        g.excludeClipRegion (shadowArea);
        g.fillAll (bgColour);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (shouldResize)
        {
            Point<int> convertedPoint;

            if (getParentComponent() == nullptr)
                convertedPoint = e.eventComponent->localPointToGlobal (e.getPosition());
            else
                convertedPoint = getParentComponent()->getLocalPoint (e.eventComponent, e.getPosition());

            auto currentMouseDragX = convertedPoint.x;

            if (isOnLeft)
            {
                amountMoved = startingBounds.getRight() - currentMouseDragX;
                setBounds (getBounds().withX (startingBounds.getX() - jmax (amountMoved, 0)));
            }
            else
            {
                amountMoved = currentMouseDragX - startingBounds.getX();
                setBounds (getBounds().withX (startingBounds.getX() + jmax (amountMoved, 0)));
            }
        }
        else if (isShowing)
        {
            auto relativeMouseDownPosition = getLocalPoint (e.eventComponent, e.getMouseDownPosition());
            auto relativeMouseDragPosition = getLocalPoint (e.eventComponent, e.getPosition());

            if (! getLocalBounds().contains (relativeMouseDownPosition)
                  && getLocalBounds().contains (relativeMouseDragPosition))
            {
                shouldResize = true;
                startingBounds = getBounds();
            }
        }
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (shouldResize)
        {
            showOrHide (amountMoved < (panelWidth / 2));

            amountMoved = 0;
            shouldResize = false;
        }
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        dismissButton.setShape (lf.getSidePanelDismissButtonShape (*this), false, true, false);

        dismissButton.setColours (lf.findColour (SidePanel::dismissButtonNormalColour),
                                  lf.findColour (SidePanel::dismissButtonOverColour),
                                  lf.findColour (SidePanel::dismissButtonDownColour));

        titleLabel.setFont (lf.getSidePanelTitleFont (*this));
        titleLabel.setColour (Label::textColourId, findColour (SidePanel::titleTextColour));
        titleLabel.setJustificationType (lf.getSidePanelTitleJustification (*this));
        */
    }
    
    pub fn calculate_bounds_in_parent(&self, parent_comp: &mut Component<'a>) -> Rectangle<i32> {
        
        todo!();
        /*
            auto parentBounds = parentComp.getLocalBounds();

        if (isOnLeft)
        {
            return isShowing ? parentBounds.removeFromLeft (panelWidth)
                             : parentBounds.withX (parentBounds.getX() - panelWidth).withWidth (panelWidth);
        }

        return isShowing ? parentBounds.removeFromRight (panelWidth)
                         : parentBounds.withX (parentBounds.getRight()).withWidth (panelWidth);
        */
    }
    
    pub fn calculate_and_remove_shadow_bounds(&mut self, bounds: &mut Rectangle<i32>)  {
        
        todo!();
        /*
            shadowArea = isOnLeft ? bounds.removeFromRight (shadowWidth)
                              : bounds.removeFromLeft  (shadowWidth);
        */
    }
    
    pub fn is_mouse_event_in_this_or_children(&mut self, event_component: *mut Component<'a>) -> bool {
        
        todo!();
        /*
            if (eventComponent == this)
            return true;

        for (auto& child : getChildren())
            if (eventComponent == child)
                return true;

        return false;
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::group);
        */
    }
}
