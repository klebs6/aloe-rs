crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_Viewport.h]

/**
  | A Viewport is used to contain a larger
  | child component, and allows the child
  | to be automatically scrolled around.
  | 
  | To use a Viewport, just create one and
  | set the component that goes inside it
  | using the setViewedComponent() method.
  | When the child component changes size,
  | the Viewport will adjust its scrollbars
  | accordingly.
  | 
  | A subclass of the viewport can be created
  | which will receive calls to its visibleAreaChanged()
  | method when the subcomponent changes
  | position or size.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Viewport<'a> {
    base:                               Component<'a>,
    vertical_scroll_bar:                Box<ScrollBar<'a>>,
    horizontal_scroll_bar:              Box<ScrollBar<'a>>,
    content_holder:                     Component<'a>,
    content_comp:                       WeakReference<Component<'a>>,
    last_visible_area:                  Rectangle<i32>,
    scroll_bar_thickness:               i32, // default = 0
    single_stepx:                       i32, // default = 16
    single_stepy:                       i32, // default = 16
    show_hscrollbar:                    bool, // default = true
    show_vscrollbar:                    bool, // default = true
    delete_content:                     bool, // default = true
    custom_scroll_bar_thickness:        bool, // default = false
    allow_scrolling_without_scrollbarv: bool, // default = false
    allow_scrolling_without_scrollbarh: bool, // default = false
    scrollbar_right:                    bool, // default = true
    h_scrollbar_bottom:                 bool, // default = true
    drag_to_scroll_listener:            Box<ViewportDragToScrollListener<'a>>,
}

impl<'a> ComponentListener for Viewport<'a> {

}

impl<'a> ComponentMovedOrResized for Viewport<'a> {

    fn component_moved_or_resized(&mut self, 
        _0: &mut Component,
        _1: bool,
        _2: bool)  {
        
        todo!();
        /*
            updateVisibleArea();
        */
    }
}

impl<'a> ComponentBroughtToFront for Viewport<'a> {

}

impl<'a> ComponentVisibilityChanged      for Viewport<'a> {}
impl<'a> ComponentChildrenChanged        for Viewport<'a> {}
impl<'a> ComponentParentHierarchyChanged for Viewport<'a> {}
impl<'a> ComponentNameChanged            for Viewport<'a> {}
impl<'a> ComponentBeingDeleted           for Viewport<'a> {}
impl<'a> ComponentEnablementChanged      for Viewport<'a> {}

impl<'a> ScrollBarListener for Viewport<'a> {

    fn scroll_bar_moved(&mut self, 
        scroll_bar_that_has_moved: *mut ScrollBar,
        new_range_start:           f64)  {
        
        todo!();
        /*
            auto newRangeStartInt = roundToInt (newRangeStart);

        if (scrollBarThatHasMoved == horizontalScrollBar.get())
        {
            setViewPosition (newRangeStartInt, getViewPositionY());
        }
        else if (scrollBarThatHasMoved == verticalScrollBar.get())
        {
            setViewPosition (getViewPositionX(), newRangeStartInt);
        }
        */
    }
}

impl<'a> Drop for Viewport<'a> {

    fn drop(&mut self) {

        todo!();

        /* 
        setScrollOnDragEnabled (false);
        deleteOrRemoveContentComp();
         */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_Viewport.cpp]
impl<'a> Viewport<'a> {
    
    /**
      | Returns the component that's currently
      | being used inside the Viewport.
      | 
      | @see setViewedComponent
      |
      */
    pub fn get_viewed_component(&self) -> *mut Component {
        
        todo!();
        /*
            return contentComp.get();
        */
    }

    /**
      | Returns the position within the child
      | component of the top-left of its visible
      | area.
      |
      */
    pub fn get_view_position(&self) -> Point<i32> {
        
        todo!();
        /*
            return lastVisibleArea.getPosition();
        */
    }

    /**
      | Returns the visible area of the child
      | component, relative to its top-left
      |
      */
    pub fn get_view_area(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return lastVisibleArea;
        */
    }

    /**
      | Returns the position within the child
      | component of the top-left of its visible
      | area. @see getViewWidth, setViewPosition
      |
      */
    pub fn get_view_positionx(&self) -> i32 {
        
        todo!();
        /*
            return lastVisibleArea.getX();
        */
    }

    /**
      | Returns the position within the child
      | component of the top-left of its visible
      | area. @see getViewHeight, setViewPosition
      |
      */
    pub fn get_view_positiony(&self) -> i32 {
        
        todo!();
        /*
            return lastVisibleArea.getY();
        */
    }

    /**
      | Returns the width of the visible area
      | of the child component.
      | 
      | This may be less than the width of this
      | Viewport if there's a vertical scrollbar
      | or if the child component is itself smaller.
      |
      */
    pub fn get_view_width(&self) -> i32 {
        
        todo!();
        /*
            return lastVisibleArea.getWidth();
        */
    }

    /**
      | Returns the height of the visible area
      | of the child component.
      | 
      | This may be less than the height of this
      | Viewport if there's a horizontal scrollbar
      | or if the child component is itself smaller.
      |
      */
    pub fn get_view_height(&self) -> i32 {
        
        todo!();
        /*
            return lastVisibleArea.getHeight();
        */
    }

    /**
      | True if the vertical scrollbar will
      | appear on the right side of the content
      |
      */
    pub fn is_vertical_scrollbar_on_the_right(&self) -> bool {
        
        todo!();
        /*
            return vScrollbarRight;
        */
    }

    /**
      | True if the horizontal scrollbar will
      | appear at the bottom of the content
      |
      */
    pub fn is_horizontal_scrollbar_at_bottom(&self) -> bool {
        
        todo!();
        /*
            return hScrollbarBottom;
        */
    }

    /**
      | True if the vertical scrollbar is enabled.
      | @see setScrollBarsShown
      |
      */
    pub fn is_vertical_scroll_bar_shown(&self) -> bool {
        
        todo!();
        /*
            return showVScrollbar;
        */
    }

    /**
      | True if the horizontal scrollbar is
      | enabled. @see setScrollBarsShown
      |
      */
    pub fn is_horizontal_scroll_bar_shown(&self) -> bool {
        
        todo!();
        /*
            return showHScrollbar;
        */
    }

    /**
      | Returns a reference to the scrollbar
      | component being used.
      | 
      | Handy if you need to customise the bar
      | somehow.
      |
      */
    pub fn get_vertical_scroll_bar(&mut self) -> &mut ScrollBar {
        
        todo!();
        /*
            return *verticalScrollBar;
        */
    }

    /**
      | Returns a reference to the scrollbar
      | component being used.
      | 
      | Handy if you need to customise the bar
      | somehow.
      |
      */
    pub fn get_horizontal_scroll_bar(&mut self) -> &mut ScrollBar {
        
        todo!();
        /*
            return *horizontalScrollBar;
        */
    }

    /**
      | Creates a Viewport.
      | 
      | The viewport is initially empty - use
      | the setViewedComponent() method to
      | add a child component for it to manage.
      |
      */
    pub fn new(name: Option<&String>) -> Self {

        let name: &String = name.unwrap_or(&String::new());
    
        todo!();
        /*
        : component(name),

            // content holder is used to clip the contents so they don't overlap the scrollbars
        addAndMakeVisible (contentHolder);
        contentHolder.setInterceptsMouseClicks (false, true);

        scrollBarThickness = getLookAndFeel().getDefaultScrollbarWidth();

        setInterceptsMouseClicks (false, true);
        setWantsKeyboardFocus (true);
        setScrollOnDragEnabled (Desktop::getInstance().getMainMouseSource().isTouch());

        recreateScrollbars();
        */
    }
    
    pub fn visible_area_changed(&mut self, _0: &Rectangle<i32>)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn viewed_component_changed(&mut self, _0: *mut Component)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn delete_or_remove_content_comp(&mut self)  {
        
        todo!();
        /*
            if (contentComp != nullptr)
        {
            contentComp->removeComponentListener (this);

            if (deleteContent)
            {
                // This sets the content comp to a null pointer before deleting the old one, in case
                // anything tries to use the old one while it's in mid-deletion..
                std::unique_ptr<Component> oldCompDeleter (contentComp.get());
                contentComp = nullptr;
            }
            else
            {
                contentHolder.removeChildComponent (contentComp);
                contentComp = nullptr;
            }
        }
        */
    }
    
    /**
      | Sets the component that this viewport
      | will contain and scroll around.
      | 
      | This will add the given component to
      | this Viewport and position it at (0,
      | 0).
      | 
      | (Don't add or remove any child components
      | directly using the normal
      | 
      | Component::addChildComponent()
      | methods).
      | 
      | -----------
      | @param newViewedComponent
      | 
      | the component to add to this viewport,
      | or null to remove the current component.
      | ----------
      | @param deleteComponentWhenNoLongerNeeded
      | 
      | if true, the component will be deleted
      | automatically when the viewport is
      | deleted or when a different component
      | is added. If false, the caller must manage
      | the lifetime of the component @see getViewedComponent
      |
      */
    pub fn set_viewed_component(
        &mut self, 
        new_viewed_component:                   *mut Component,
        delete_component_when_no_longer_needed: Option<bool>

    ) {

        let delete_component_when_no_longer_needed: bool = delete_component_when_no_longer_needed.unwrap_or(true);
        
        todo!();
        /*
            if (contentComp.get() != newViewedComponent)
        {
            deleteOrRemoveContentComp();
            contentComp = newViewedComponent;
            deleteContent = deleteComponentWhenNoLongerNeeded;

            if (contentComp != nullptr)
            {
                contentHolder.addAndMakeVisible (contentComp);
                setViewPosition (Point<int>());
                contentComp->addComponentListener (this);
            }

            viewedComponentChanged (contentComp);
            updateVisibleArea();
        }
        */
    }
    
    /**
      | Re-instantiates the scrollbars, which
      | is only really useful if you've overridden
      | createScrollBarComponent().
      |
      */
    pub fn recreate_scrollbars(&mut self)  {
        
        todo!();
        /*
            verticalScrollBar.reset();
        horizontalScrollBar.reset();

        verticalScrollBar  .reset (createScrollBarComponent (true));
        horizontalScrollBar.reset (createScrollBarComponent (false));

        addChildComponent (verticalScrollBar.get());
        addChildComponent (horizontalScrollBar.get());

        getVerticalScrollBar().addListener (this);
        getHorizontalScrollBar().addListener (this);

        resized();
        */
    }
    
    /**
      | Returns the width available within
      | this component for the contents.
      | 
      | This will be the width of the viewport
      | component minus the width of a vertical
      | scrollbar (if visible).
      |
      */
    pub fn get_maximum_visible_width(&self) -> i32 {
        
        todo!();
        /*
            return contentHolder.getWidth();
        */
    }
    
    /**
      | Returns the height available within
      | this component for the contents.
      | 
      | This will be the height of the viewport
      | component minus the space taken up by
      | a horizontal scrollbar (if visible).
      |
      */
    pub fn get_maximum_visible_height(&self) -> i32 {
        
        todo!();
        /*
            return contentHolder.getHeight();
        */
    }
    
    /**
      | True if there's any off-screen content
      | that could be scrolled vertically,
      | or false if everything is currently
      | visible.
      |
      */
    pub fn can_scroll_vertically(&self) -> bool {
        
        todo!();
        /*
            return contentComp->getY() < 0 || contentComp->getBottom() > getHeight();
        */
    }
    
    /**
      | True if there's any off-screen content
      | that could be scrolled horizontally,
      | or false if everything is currently
      | visible.
      |
      */
    pub fn can_scroll_horizontally(&self) -> bool {
        
        todo!();
        /*
            return contentComp->getX() < 0 || contentComp->getRight()  > getWidth();
        */
    }
    
    pub fn viewport_pos_to_comp_pos(&self, pos: Point<i32>) -> Point<i32> {
        
        todo!();
        /*
            jassert (contentComp != nullptr);

        auto contentBounds = contentHolder.getLocalArea (contentComp.get(), contentComp->getLocalBounds());

        Point<int> p (jmax (jmin (0, contentHolder.getWidth()  - contentBounds.getWidth()),  jmin (0, -(pos.x))),
                      jmax (jmin (0, contentHolder.getHeight() - contentBounds.getHeight()), jmin (0, -(pos.y))));

        return p.transformedBy (contentComp->getTransform().inverted());
        */
    }
    
    /**
      | Changes the position of the viewed component.
      | 
      | The inner component will be moved so
      | that the pixel at the top left of the viewport
      | will be the pixel at position (xPixelsOffset,
      | yPixelsOffset) within the inner component.
      | 
      | This will update the scrollbars and
      | might cause a call to visibleAreaChanged().
      | 
      | @see getViewPositionX, getViewPositionY,
      | setViewPositionProportionately
      |
      */
    pub fn set_view_position_with_xy(
        &mut self, 
        x_pixels_offset: i32,
        y_pixels_offset: i32

    ) {
        
        todo!();
        /*
            setViewPosition ({ xPixelsOffset, yPixelsOffset });
        */
    }
    
    /**
      | Changes the position of the viewed component.
      | 
      | The inner component will be moved so
      | that the pixel at the top left of the viewport
      | will be the pixel at the specified coordinates
      | within the inner component.
      | 
      | This will update the scrollbars and
      | might cause a call to visibleAreaChanged().
      | 
      | @see getViewPositionX, getViewPositionY,
      | setViewPositionProportionately
      |
      */
    pub fn set_view_position(&mut self, new_position: Point<i32>)  {
        
        todo!();
        /*
            if (contentComp != nullptr)
            contentComp->setTopLeftPosition (viewportPosToCompPos (newPosition));
        */
    }
    
    /**
      | Changes the view position as a proportion
      | of the distance it can move.
      | 
      | The values here are from 0.0 to 1.0 - where
      | (0, 0) would put the visible area in the
      | top-left, and (1, 1) would put it as far
      | down and to the right as it's possible
      | to go whilst keeping the child component
      | on-screen.
      |
      */
    pub fn set_view_position_proportionately(&mut self, x: f64, y: f64)  {
        
        todo!();
        /*
            if (contentComp != nullptr)
            setViewPosition (jmax (0, roundToInt (x * (contentComp->getWidth()  - getWidth()))),
                             jmax (0, roundToInt (y * (contentComp->getHeight() - getHeight()))));
        */
    }
    
    /**
      | If the specified position is at the edges
      | of the viewport, this method scrolls
      | the viewport to bring that position
      | nearer to the centre.
      | 
      | Call this if you're dragging an object
      | inside a viewport and want to make it
      | scroll when the user approaches an edge.
      | You might also find Component::beginDragAutoRepeat()
      | useful when auto-scrolling.
      | 
      | -----------
      | @param mouseX
      | 
      | the x position, relative to the Viewport's
      | top-left
      | ----------
      | @param mouseY
      | 
      | the y position, relative to the Viewport's
      | top-left
      | ----------
      | @param distanceFromEdge
      | 
      | specifies how close to an edge the position
      | needs to be before the viewport should
      | scroll in that direction
      | ----------
      | @param maximumSpeed
      | 
      | the maximum number of pixels that the
      | viewport is allowed to scroll by.
      | 
      | -----------
      | @return
      | 
      | true if the viewport was scrolled
      |
      */
    pub fn auto_scroll(&mut self, 
        mousex:                  i32,
        mousey:                  i32,
        active_border_thickness: i32,
        maximum_speed:           i32) -> bool {
        
        todo!();
        /*
            if (contentComp != nullptr)
        {
            int dx = 0, dy = 0;

            if (getHorizontalScrollBar().isVisible() || canScrollHorizontally())
            {
                if (mouseX < activeBorderThickness)
                    dx = activeBorderThickness - mouseX;
                else if (mouseX >= contentHolder.getWidth() - activeBorderThickness)
                    dx = (contentHolder.getWidth() - activeBorderThickness) - mouseX;

                if (dx < 0)
                    dx = jmax (dx, -maximumSpeed, contentHolder.getWidth() - contentComp->getRight());
                else
                    dx = jmin (dx, maximumSpeed, -contentComp->getX());
            }

            if (getVerticalScrollBar().isVisible() || canScrollVertically())
            {
                if (mouseY < activeBorderThickness)
                    dy = activeBorderThickness - mouseY;
                else if (mouseY >= contentHolder.getHeight() - activeBorderThickness)
                    dy = (contentHolder.getHeight() - activeBorderThickness) - mouseY;

                if (dy < 0)
                    dy = jmax (dy, -maximumSpeed, contentHolder.getHeight() - contentComp->getBottom());
                else
                    dy = jmin (dy, maximumSpeed, -contentComp->getY());
            }

            if (dx != 0 || dy != 0)
            {
                contentComp->setTopLeftPosition (contentComp->getX() + dx,
                                                 contentComp->getY() + dy);

                return true;
            }
        }

        return false;
        */
    }
    
    /**
      | Enables or disables drag-to-scroll
      | functionality in the viewport.
      | 
      | If your viewport contains a Component
      | that you don't want to receive mouse
      | events when the user is drag-scrolling,
      | you can disable this with the Component::setViewportIgnoreDragFlag()
      | method.
      |
      */
    pub fn set_scroll_on_drag_enabled(&mut self, should_scroll_on_drag: bool)  {
        
        todo!();
        /*
            if (isScrollOnDragEnabled() != shouldScrollOnDrag)
        {
            if (shouldScrollOnDrag)
                dragToScrollListener.reset (new ViewportDragToScrollListener (*this));
            else
                dragToScrollListener.reset();
        }
        */
    }
    
    /**
      | Returns true if drag-to-scroll functionality
      | is enabled.
      |
      */
    pub fn is_scroll_on_drag_enabled(&self) -> bool {
        
        todo!();
        /*
            return dragToScrollListener != nullptr;
        */
    }
    
    /**
      | Returns true if the user is currently
      | dragging-to-scroll. @see setScrollOnDragEnabled
      |
      */
    pub fn is_currently_scrolling_on_drag(&self) -> bool {
        
        todo!();
        /*
            return dragToScrollListener != nullptr && dragToScrollListener->isDragging;
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            if (! customScrollBarThickness)
        {
            scrollBarThickness = getLookAndFeel().getDefaultScrollbarWidth();
            resized();
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            updateVisibleArea();
        */
    }
    
    pub fn update_visible_area(&mut self)  {
        
        todo!();
        /*
            auto scrollbarWidth = getScrollBarThickness();
        const bool canShowAnyBars = getWidth() > scrollbarWidth && getHeight() > scrollbarWidth;
        const bool canShowHBar = showHScrollbar && canShowAnyBars;
        const bool canShowVBar = showVScrollbar && canShowAnyBars;

        bool hBarVisible = false, vBarVisible = false;
        Rectangle<int> contentArea;

        for (int i = 3; --i >= 0;)
        {
            hBarVisible = canShowHBar && ! getHorizontalScrollBar().autoHides();
            vBarVisible = canShowVBar && ! getVerticalScrollBar().autoHides();
            contentArea = getLocalBounds();

            if (contentComp != nullptr && ! contentArea.contains (contentComp->getBounds()))
            {
                hBarVisible = canShowHBar && (hBarVisible || contentComp->getX() < 0 || contentComp->getRight() > contentArea.getWidth());
                vBarVisible = canShowVBar && (vBarVisible || contentComp->getY() < 0 || contentComp->getBottom() > contentArea.getHeight());

                if (vBarVisible)
                    contentArea.setWidth (getWidth() - scrollbarWidth);

                if (hBarVisible)
                    contentArea.setHeight (getHeight() - scrollbarWidth);

                if (! contentArea.contains (contentComp->getBounds()))
                {
                    hBarVisible = canShowHBar && (hBarVisible || contentComp->getRight() > contentArea.getWidth());
                    vBarVisible = canShowVBar && (vBarVisible || contentComp->getBottom() > contentArea.getHeight());
                }
            }

            if (vBarVisible)  contentArea.setWidth  (getWidth()  - scrollbarWidth);
            if (hBarVisible)  contentArea.setHeight (getHeight() - scrollbarWidth);

            if (! vScrollbarRight  && vBarVisible)
                contentArea.setX (scrollbarWidth);

            if (! hScrollbarBottom && hBarVisible)
                contentArea.setY (scrollbarWidth);

            if (contentComp == nullptr)
            {
                contentHolder.setBounds (contentArea);
                break;
            }

            auto oldContentBounds = contentComp->getBounds();
            contentHolder.setBounds (contentArea);

            // If the content has changed its size, that might affect our scrollbars, so go round again and re-calculate..
            if (oldContentBounds == contentComp->getBounds())
                break;
        }

        Rectangle<int> contentBounds;

        if (auto cc = contentComp.get())
            contentBounds = contentHolder.getLocalArea (cc, cc->getLocalBounds());

        auto visibleOrigin = -contentBounds.getPosition();

        auto& hbar = getHorizontalScrollBar();
        auto& vbar = getVerticalScrollBar();

        hbar.setBounds (contentArea.getX(), hScrollbarBottom ? contentArea.getHeight() : 0, contentArea.getWidth(), scrollbarWidth);
        hbar.setRangeLimits (0.0, contentBounds.getWidth());
        hbar.setCurrentRange (visibleOrigin.x, contentArea.getWidth());
        hbar.setSingleStepSize (singleStepX);

        if (canShowHBar && ! hBarVisible)
            visibleOrigin.setX (0);

        vbar.setBounds (vScrollbarRight ? contentArea.getWidth() : 0, contentArea.getY(), scrollbarWidth, contentArea.getHeight());
        vbar.setRangeLimits (0.0, contentBounds.getHeight());
        vbar.setCurrentRange (visibleOrigin.y, contentArea.getHeight());
        vbar.setSingleStepSize (singleStepY);

        if (canShowVBar && ! vBarVisible)
            visibleOrigin.setY (0);

        // Force the visibility *after* setting the ranges to avoid flicker caused by edge conditions in the numbers.
        hbar.setVisible (hBarVisible);
        vbar.setVisible (vBarVisible);

        if (contentComp != nullptr)
        {
            auto newContentCompPos = viewportPosToCompPos (visibleOrigin);

            if (contentComp->getBounds().getPosition() != newContentCompPos)
            {
                contentComp->setTopLeftPosition (newContentCompPos);  // (this will re-entrantly call updateVisibleArea again)
                return;
            }
        }

        const Rectangle<int> visibleArea (visibleOrigin.x, visibleOrigin.y,
                                          jmin (contentBounds.getWidth()  - visibleOrigin.x, contentArea.getWidth()),
                                          jmin (contentBounds.getHeight() - visibleOrigin.y, contentArea.getHeight()));

        if (lastVisibleArea != visibleArea)
        {
            lastVisibleArea = visibleArea;
            visibleAreaChanged (visibleArea);
        }

        hbar.handleUpdateNowIfNeeded();
        vbar.handleUpdateNowIfNeeded();
        */
    }
    
    /**
      | Changes the distance that a single-step
      | click on a scrollbar button will move
      | the viewport.
      |
      */
    pub fn set_single_step_sizes(&mut self, 
        stepx: i32,
        stepy: i32)  {
        
        todo!();
        /*
            if (singleStepX != stepX || singleStepY != stepY)
        {
            singleStepX = stepX;
            singleStepY = stepY;
            updateVisibleArea();
        }
        */
    }
    
    /**
      | Turns scrollbars on or off.
      | 
      | If set to false, the scrollbars won't
      | ever appear. When true (the default)
      | they will appear only when needed.
      | 
      | The allowVerticalScrollingWithoutScrollbar
      | parameters allow you to enable mouse-wheel
      | scrolling even when there the scrollbars
      | are hidden. When the scrollbars are
      | visible, these parameters are ignored.
      |
      */
    pub fn set_scroll_bars_shown(
        &mut self, 
        show_vertical_scrollbar_if_needed:            bool,
        show_horizontal_scrollbar_if_needed:          bool,
        allow_vertical_scrolling_without_scrollbar:   Option<bool>,
        allow_horizontal_scrolling_without_scrollbar: Option<bool>
    )  {

        let allow_vertical_scrolling_without_scrollbar:   bool = allow_vertical_scrolling_without_scrollbar.unwrap_or(false);
        let allow_horizontal_scrolling_without_scrollbar: bool = allow_horizontal_scrolling_without_scrollbar.unwrap_or(false);
        
        todo!();
        /*
            allowScrollingWithoutScrollbarV = allowVerticalScrollingWithoutScrollbar;
        allowScrollingWithoutScrollbarH = allowHorizontalScrollingWithoutScrollbar;

        if (showVScrollbar != showVerticalScrollbarIfNeeded
             || showHScrollbar != showHorizontalScrollbarIfNeeded)
        {
            showVScrollbar = showVerticalScrollbarIfNeeded;
            showHScrollbar = showHorizontalScrollbarIfNeeded;
            updateVisibleArea();
        }
        */
    }
    
    /**
      | Changes the width of the scrollbars.
      | 
      | If this isn't specified, the default
      | width from the LookAndFeel class will
      | be used. @see LookAndFeel::getDefaultScrollbarWidth
      |
      */
    pub fn set_scroll_bar_thickness(&mut self, thickness: i32)  {
        
        todo!();
        /*
            int newThickness;

        // To stay compatible with the previous code: use the
        // default thickness if thickness parameter is zero
        // or negative
        if (thickness <= 0)
        {
            customScrollBarThickness = false;
            newThickness = getLookAndFeel().getDefaultScrollbarWidth();
        }
        else
        {
            customScrollBarThickness = true;
            newThickness = thickness;
        }

        if (scrollBarThickness != newThickness)
        {
            scrollBarThickness = newThickness;
            updateVisibleArea();
        }
        */
    }
    
    /**
      | Returns the thickness of the scrollbars.
      | @see setScrollBarThickness
      |
      */
    pub fn get_scroll_bar_thickness(&self) -> i32 {
        
        todo!();
        /*
            return scrollBarThickness;
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            if (! useMouseWheelMoveIfNeeded (e, wheel))
            Component::mouseWheelMove (e, wheel);
        */
    }
    
    pub fn use_mouse_wheel_move_if_needed(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails) -> bool {
        
        todo!();
        /*
            if (! (e.mods.isAltDown() || e.mods.isCtrlDown() || e.mods.isCommandDown()))
        {
            const bool canScrollVert = (allowScrollingWithoutScrollbarV || getVerticalScrollBar().isVisible());
            const bool canScrollHorz = (allowScrollingWithoutScrollbarH || getHorizontalScrollBar().isVisible());

            if (canScrollHorz || canScrollVert)
            {
                auto deltaX = rescaleMouseWheelDistance (wheel.deltaX, singleStepX);
                auto deltaY = rescaleMouseWheelDistance (wheel.deltaY, singleStepY);

                auto pos = getViewPosition();

                if (deltaX != 0 && deltaY != 0 && canScrollHorz && canScrollVert)
                {
                    pos.x -= deltaX;
                    pos.y -= deltaY;
                }
                else if (canScrollHorz && (deltaX != 0 || e.mods.isShiftDown() || ! canScrollVert))
                {
                    pos.x -= deltaX != 0 ? deltaX : deltaY;
                }
                else if (canScrollVert && deltaY != 0)
                {
                    pos.y -= deltaY;
                }

                if (pos != getViewPosition())
                {
                    setViewPosition (pos);
                    return true;
                }
            }
        }

        return false;
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            const bool isUpDownKey = isUpDownKeyPress (key);

        if (getVerticalScrollBar().isVisible() && isUpDownKey)
            return getVerticalScrollBar().keyPressed (key);

        const bool isLeftRightKey = isLeftRightKeyPress (key);

        if (getHorizontalScrollBar().isVisible() && (isUpDownKey || isLeftRightKey))
            return getHorizontalScrollBar().keyPressed (key);

        return false;
        */
    }
    
    pub fn responds_to_key(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            return isUpDownKeyPress (key) || isLeftRightKeyPress (key);
        */
    }
    
    pub fn create_scroll_bar_component(&mut self, is_vertical: bool) -> *mut ScrollBar {
        
        todo!();
        /*
            return new ScrollBar (isVertical);
        */
    }
    
    /**
      | Changes where the scroll bars are positioned
      | 
      | If verticalScrollbarOnRight is set
      | to true, then the vertical scrollbar
      | will appear on the right side of the view
      | port's content (this is the default),
      | otherwise it will be on the left side
      | of the content.
      | 
      | If horizontalScrollbarAtBottom is
      | set to true, then the horizontal scrollbar
      | will appear at the bottom of the view
      | port's content (this is the default),
      | otherwise it will be at the top.
      |
      */
    pub fn set_scroll_bar_position(&mut self, 
        vertical_scrollbar_on_right:    bool,
        horizontal_scrollbar_at_bottom: bool)  {
        
        todo!();
        /*
            vScrollbarRight  = verticalScrollbarOnRight;
        hScrollbarBottom = horizontalScrollbarAtBottom;

        resized();
        */
    }
}
