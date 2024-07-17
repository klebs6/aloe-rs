crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MouseSourceState<'a> {
    base:                 Timer,
    window:               &'a mut MenuWindow<'a>,
    source:               MouseInputSource<'a>,
    last_mouse_pos:       Point<i32>,
    scroll_acceleration:  f64, // default = 0
    last_scroll_time:     u32,
    last_mouse_move_time: u32, // default = 0
    is_down:              bool, // default = false
}

impl<'a> MouseSourceState<'a> {

    pub fn new(
        w: &mut MenuWindow,
        s: MouseInputSource) -> Self {
    
        todo!();
        /*


            : window (w), source (s), lastScrollTime (Time::getMillisecondCounter())
                startTimerHz (20);
        */
    }
    
    pub fn handle_mouse_event(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (! window.windowIsStillValid())
                    return;

                startTimerHz (20);
                handleMousePosition (e.getScreenPosition());
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            #if ALOE_WINDOWS
                // touch and pen devices on Windows send an offscreen mouse move after mouse up events
                // but we don't want to forward these on as they will dismiss the menu
                if ((source.isTouch() || source.isPen()) && ! isValidMousePosition())
                    return;
               #endif

                if (window.windowIsStillValid())
                    handleMousePosition (source.getScreenPosition().roundToInt());
        */
    }
    
    pub fn is_over(&self) -> bool {
        
        todo!();
        /*
            return window.reallyContains (window.getLocalPoint (nullptr, source.getScreenPosition()).roundToInt(), true);
        */
    }
    
    pub fn handle_mouse_position(&mut self, global_mouse_pos: Point<i32>)  {
        
        todo!();
        /*
            auto localMousePos = window.getLocalPoint (nullptr, globalMousePos);
                auto timeNow = Time::getMillisecondCounter();

                if (timeNow > window.timeEnteredCurrentChildComp + 100
                     && window.reallyContains (localMousePos, true)
                     && window.currentChild != nullptr
                     && ! (window.disableMouseMoves || window.isSubMenuVisible()))
                {
                    window.showSubMenuFor (window.currentChild);
                }

                highlightItemUnderMouse (globalMousePos, localMousePos, timeNow);

                const bool overScrollArea = scrollIfNecessary (localMousePos, timeNow);
                const bool isOverAny = window.isOverAnyMenu();

                if (window.hideOnExit && window.hasBeenOver && ! isOverAny)
                    window.hide (nullptr, true);
                else
                    checkButtonState (localMousePos, timeNow, isDown, overScrollArea, isOverAny);
        */
    }
    
    pub fn check_button_state(&mut self, 
        local_mouse_pos:  Point<i32>,
        time_now:         u32,
        was_down:         bool,
        over_scroll_area: bool,
        is_over_any:      bool)  {
        
        todo!();
        /*
            isDown = window.hasBeenOver
                            && (ModifierKeys::currentModifiers.isAnyMouseButtonDown()
                                 || ComponentPeer::getCurrentModifiersRealtime().isAnyMouseButtonDown());

                if (! window.doesAnyAloeCompHaveFocus())
                {
                    if (timeNow > window.lastFocusedTime + 10)
                    {
                        PopupMenuSettings::menuWasHiddenBecauseOfAppChange = true;
                        window.dismissMenu (nullptr);
                        // Note: This object may have been deleted by the previous call.
                    }
                }
                else if (wasDown && timeNow > window.windowCreationTime + 250
                           && ! (isDown || overScrollArea))
                {
                    if (window.reallyContains (localMousePos, true))
                        window.triggerCurrentlyHighlightedItem();
                    else if ((window.hasBeenOver || ! window.dismissOnMouseUp) && ! isOverAny)
                        window.dismissMenu (nullptr);

                    // Note: This object may have been deleted by the previous call.
                }
                else
                {
                    window.lastFocusedTime = timeNow;
                }
        */
    }
    
    pub fn highlight_item_under_mouse(&mut self, 
        global_mouse_pos: Point<i32>,
        local_mouse_pos:  Point<i32>,
        time_now:         u32)  {
        
        todo!();
        /*
            if (globalMousePos != lastMousePos || timeNow > lastMouseMoveTime + 350)
                {
                    const auto isMouseOver = window.reallyContains (localMousePos, true);

                    if (isMouseOver)
                        window.hasBeenOver = true;

                    if (lastMousePos.getDistanceFrom (globalMousePos) > 2)
                    {
                        lastMouseMoveTime = timeNow;

                        if (window.disableMouseMoves && isMouseOver)
                            window.disableMouseMoves = false;
                    }

                    if (window.disableMouseMoves || (window.activeSubMenu != nullptr && window.activeSubMenu->isOverChildren()))
                        return;

                    const bool isMovingTowardsMenu = isMouseOver && globalMousePos != lastMousePos
                                                        && isMovingTowardsSubmenu (globalMousePos);

                    lastMousePos = globalMousePos;

                    if (! isMovingTowardsMenu)
                    {
                        auto* c = window.getComponentAt (localMousePos);

                        if (c == &window)
                            c = nullptr;

                        auto* itemUnderMouse = dynamic_cast<PopupMenuItemComponent*> (c);

                        if (itemUnderMouse == nullptr && c != nullptr)
                            itemUnderMouse = c->findParentComponentOfClass<PopupMenuItemComponent>();

                        if (itemUnderMouse != window.currentChild
                              && (isMouseOver || (window.activeSubMenu == nullptr) || ! window.activeSubMenu->isVisible()))
                        {
                            if (isMouseOver && (c != nullptr) && (window.activeSubMenu != nullptr))
                                window.activeSubMenu->hide (nullptr, true);

                            if (! isMouseOver)
                            {
                                if (! window.hasBeenOver)
                                    return;

                                itemUnderMouse = nullptr;
                            }

                            window.setCurrentlyHighlightedChild (itemUnderMouse);
                        }
                    }
                }
        */
    }
    
    pub fn is_moving_towards_submenu(&self, new_global_pos: Point<i32>) -> bool {
        
        todo!();
        /*
            if (window.activeSubMenu == nullptr)
                    return false;

                // try to intelligently guess whether the user is moving the mouse towards a currently-open
                // submenu. To do this, look at whether the mouse stays inside a triangular region that
                // extends from the last mouse pos to the submenu's rectangle..

                auto itemScreenBounds = window.activeSubMenu->getScreenBounds();
                auto subX = (float) itemScreenBounds.getX();

                auto oldGlobalPos = lastMousePos;

                if (itemScreenBounds.getX() > window.getX())
                {
                    oldGlobalPos -= Point<int> (2, 0);  // to enlarge the triangle a bit, in case the mouse only moves a couple of pixels
                }
                else
                {
                    oldGlobalPos += Point<int> (2, 0);
                    subX += (float) itemScreenBounds.getWidth();
                }

                Path areaTowardsSubMenu;
                areaTowardsSubMenu.addTriangle ((float) oldGlobalPos.x, (float) oldGlobalPos.y,
                                                subX, (float) itemScreenBounds.getY(),
                                                subX, (float) itemScreenBounds.getBottom());

                return areaTowardsSubMenu.contains (newGlobalPos.toFloat());
        */
    }
    
    pub fn scroll_if_necessary(&mut self, 
        local_mouse_pos: Point<i32>,
        time_now:        u32) -> bool {
        
        todo!();
        /*
            if (window.canScroll()
                     && isPositiveAndBelow (localMousePos.x, window.getWidth())
                     && (isPositiveAndBelow (localMousePos.y, window.getHeight()) || source.isDragging()))
                {
                    if (window.isTopScrollZoneActive() && localMousePos.y < PopupMenuSettings::scrollZone)
                        return scroll (timeNow, -1);

                    if (window.isBottomScrollZoneActive() && localMousePos.y > window.getHeight() - PopupMenuSettings::scrollZone)
                        return scroll (timeNow, 1);
                }

                scrollAcceleration = 1.0;
                return false;
        */
    }
    
    pub fn scroll(&mut self, 
        time_now:  u32,
        direction: i32) -> bool {
        
        todo!();
        /*
            if (timeNow > lastScrollTime + 20)
                {
                    scrollAcceleration = jmin (4.0, scrollAcceleration * 1.04);
                    int amount = 0;

                    for (int i = 0; i < window.items.size() && amount == 0; ++i)
                        amount = ((int) scrollAcceleration) * window.items.getUnchecked (i)->getHeight();

                    window.alterChildYPos (amount * direction);
                    lastScrollTime = timeNow;
                }

                return true;
        */
    }

    #[cfg(target_os="windows")]
    pub fn is_valid_mouse_position(&mut self) -> bool {
        
        todo!();
        /*
            auto screenPos = source.getScreenPosition();
                auto localPos = (window.activeSubMenu == nullptr) ? window.getLocalPoint (nullptr, screenPos)
                                                                  : window.activeSubMenu->getLocalPoint (nullptr, screenPos);

                if (localPos.x < 0 && localPos.y < 0)
                    return false;

                return true;
        */
    }
}
