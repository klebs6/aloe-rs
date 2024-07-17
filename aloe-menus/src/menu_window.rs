crate::ix!();

pub enum MenuSelectionDirection
{
    forwards,
    backwards,
    current
}

#[no_copy]
#[leak_detector]
pub struct MenuWindow<'a> {
    base:                            Component<'a>,
    parent:                          *mut MenuWindow<'a>,
    options:                         PopupMenuOptions<'a>,
    items:                           Vec<Box<PopupMenuItemComponent<'a>>>,
    manager_of_chosen_command:       *mut *mut dyn CommandManagerInterface,
    component_attached_to:           WeakReference<Component<'a>>,
    parent_component:                *mut Component<'a>, // default = nullptr
    window_pos:                      Rectangle<i32>,
    has_been_over:                   bool, // default = false
    needs_to_scroll:                 bool, // default = false
    dismiss_on_mouse_up:             bool,
    hide_on_exit:                    bool, // default = false
    disable_mouse_moves:             bool, // default = false
    has_any_aloe_comp_had_focus:     bool, // default = false
    num_columns:                     i32, // default = 0
    content_height:                  i32, // default = 0
    child_yoffset:                   i32, // default = 0
    current_child:                   ComponentSafePointer<'a,PopupMenuItemComponent<'a>>,
    active_sub_menu:                 Box<MenuWindow<'a>>,
    column_widths:                   Vec<i32>,
    window_creation_time:            u32,
    last_focused_time:               u32,
    time_entered_current_child_comp: u32,
    mouse_source_states:             Vec<Box<MouseSourceState<'a>>>,
    scale_factor:                    f32,
}

impl<'a> Drop for MenuWindow<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                getActiveWindows().removeFirstMatchingValue (this);
                Desktop::getInstance().removeGlobalMouseListener (this);
                activeSubMenu.reset();
                items.clear();
             */
    }
}

impl<'a> MenuWindow<'a> {

    pub fn new(
        menu:                       &PopupMenu,
        parent_window:              *mut MenuWindow,
        opts:                       PopupMenuOptions,
        align_to_rectangle:         bool,
        should_dismiss_on_mouse_up: bool,
        manager:                    *mut *mut dyn CommandManagerInterface,
        parent_scale_factor:        Option<f32>

    ) -> Self {

        let parent_scale_factor: f32 =
                 parent_scale_factor.unwrap_or(1.0);

        todo!();
        /*


            : Component ("menu"),
                 parent (parentWindow),
                 options (std::move (opts)),
                 managerOfChosenCommand (manager),
                 componentAttachedTo (options.getTargetComponent()),
                 dismissOnMouseUp (shouldDismissOnMouseUp),
                 windowCreationTime (Time::getMillisecondCounter()),
                 lastFocusedTime (windowCreationTime),
                 timeEnteredCurrentChildComp (windowCreationTime),
                 scaleFactor (parentWindow != nullptr ? parentScaleFactor : 1.0f)

                setWantsKeyboardFocus (false);
                setMouseClickGrabsKeyboardFocus (false);
                setAlwaysOnTop (true);
                setFocusContainerType (FocusContainerType::focusContainer);

                setLookAndFeel (parent != nullptr ? &(parent->getLookAndFeel())
                                                  : menu.lookAndFeel.get());

                auto& lf = getLookAndFeel();

                parentComponent = lf.getParentComponentForMenuOptions (options);
                const_cast<PopupMenuOptions&>(options) = options.withParentComponent (parentComponent);

                if (parentComponent != nullptr)
                {
                    parentComponent->addChildComponent (this);
                }
                else
                {
                    const auto shouldDisableAccessibility = [this]
                    {
                        const auto* compToCheck = parent != nullptr ? parent
                                                                    : options.getTargetComponent();

                        return compToCheck != nullptr && ! compToCheck->isAccessible();
                    }();

                    if (shouldDisableAccessibility)
                        setAccessible (false);

                    addToDesktop (ComponentPeer::windowIsTemporary
                                  | ComponentPeer::windowIgnoresKeyPresses
                                  | lf.getMenuWindowFlags());

                    Desktop::getInstance().addGlobalMouseListener (this);
                }

                if (parentComponent == nullptr && parentWindow == nullptr && lf.shouldPopupMenuScaleWithTargetComponent (options))
                    if (auto* targetComponent = options.getTargetComponent())
                        scaleFactor = Component::getApproximateScaleFactorForComponent (targetComponent);

                setOpaque (lf.findColour (typename PopupMenu::backgroundColourId).isOpaque()
                             || ! Desktop::canUseSemiTransparentWindows());

                const auto initialSelectedId = options.getInitiallySelectedItemId();

                for (int i = 0; i < menu.items.size(); ++i)
                {
                    auto& item = menu.items.getReference (i);

                    if (i + 1 < menu.items.size() || ! item.isSeparator)
                    {
                        auto* child = items.add (new PopupMenuItemComponent (item, options, *this));

                        if (initialSelectedId != 0 && item.itemID == initialSelectedId)
                            setCurrentlyHighlightedChild (child);
                    }
                }

                auto targetArea = options.getTargetScreenArea() / scaleFactor;

                calculateWindowPos (targetArea, alignToRectangle);
                setTopLeftPosition (windowPos.getPosition());
                updateYPositions();

                if (auto visibleID = options.getItemThatMustBeVisible())
                {
                    for (auto* item : items)
                    {
                        if (item->item.itemID == visibleID)
                        {
                            auto targetPosition = parentComponent != nullptr ? parentComponent->getLocalPoint (nullptr, targetArea.getTopLeft())
                                                                             : targetArea.getTopLeft();

                            auto y = targetPosition.getY() - windowPos.getY();
                            ensureItemComponentIsVisible (*item, isPositiveAndBelow (y, windowPos.getHeight()) ? y : -1);

                            break;
                        }
                    }
                }

                resizeToBestWindowPos();

                getActiveWindows().add (this);
                lf.preparePopupMenuWindow (*this);

                getMouseState (Desktop::getInstance().getMainMouseSource()); // forces creation of a mouse source watcher for the main mouse
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (isOpaque())
                    g.fillAll (Colours::white);

                auto& theme = getLookAndFeel();
                theme.drawPopupMenuBackgroundWithOptions (g, getWidth(), getHeight(), options);

                if (columnWidths.isEmpty())
                    return;

                const auto separatorWidth = theme.getPopupMenuColumnSeparatorWidthWithOptions (options);
                const auto border = theme.getPopupMenuBorderSizeWithOptions (options);

                auto currentX = 0;

                std::for_each (columnWidths.begin(), std::prev (columnWidths.end()), [&] (int width)
                {
                    const Rectangle<int> separator (currentX + width,
                                                    border,
                                                    separatorWidth,
                                                    getHeight() - border * 2);
                    theme.drawPopupMenuColumnSeparatorWithOptions (g, separator, options);
                    currentX += width + separatorWidth;
                });
        */
    }
    
    pub fn paint_over_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

                if (parentComponent != nullptr)
                    lf.drawResizableFrame (g, getWidth(), getHeight(),
                                           BorderSize<int> (getLookAndFeel().getPopupMenuBorderSizeWithOptions (options)));

                if (canScroll())
                {
                    if (isTopScrollZoneActive())
                    {
                        lf.drawPopupMenuUpDownArrowWithOptions (g,
                                                                getWidth(),
                                                                PopupMenuSettings::scrollZone,
                                                                true,
                                                                options);
                    }

                    if (isBottomScrollZoneActive())
                    {
                        g.setOrigin (0, getHeight() - PopupMenuSettings::scrollZone);
                        lf.drawPopupMenuUpDownArrowWithOptions (g,
                                                                getWidth(),
                                                                PopupMenuSettings::scrollZone,
                                                                false,
                                                                options);
                    }
                }
        */
    }

    /**
       hide this and all sub-comps
      */
    pub fn hide(&mut self, 
        item:           *const PopupMenuItem,
        make_invisible: bool)  {
        
        todo!();
        /*
            if (isVisible())
                {
                    WeakReference<Component> deletionChecker (this);

                    activeSubMenu.reset();
                    currentChild = nullptr;

                    if (item != nullptr
                         && item->commandManager != nullptr
                         && item->itemID != 0)
                    {
                        *managerOfChosenCommand = item->commandManager;
                    }

                    auto resultID = options.hasWatchedComponentBeenDeleted() ? 0 : getResultItemID (item);

                    exitModalState (resultID);

                    if (makeInvisible && deletionChecker != nullptr)
                        setVisible (false);

                    if (resultID != 0
                         && item != nullptr
                         && item->action != nullptr)
                        MessageManager::callAsync (item->action);
                }
        */
    }
    
    pub fn get_result_itemid(item: *const PopupMenuItem) -> i32 {
        
        todo!();
        /*
            if (item == nullptr)
                    return 0;

                if (auto* cc = item->customCallback.get())
                    if (! cc->menuItemTriggered())
                        return 0;

                return item->itemID;
        */
    }
    
    pub fn dismiss_menu(&mut self, item: *const PopupMenuItem)  {
        
        todo!();
        /*
            if (parent != nullptr)
                {
                    parent->dismissMenu (item);
                }
                else
                {
                    if (item != nullptr)
                    {
                        // need a copy of this on the stack as the one passed in will get deleted during this call
                        auto mi (*item);
                        hide (&mi, false);
                    }
                    else
                    {
                        hide (nullptr, true);
                    }
                }
        */
    }
    
    pub fn get_desktop_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            return scaleFactor * Desktop::getInstance().getGlobalScaleFactor();
        */
    }
    
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            if (! isShowing())
                    return;

                auto* accessibleFocus = [this]
                {
                  if (currentChild != nullptr)
                      if (auto* childHandler = currentChild->getAccessibilityHandler())
                          return childHandler;

                    return getAccessibilityHandler();
                }();

                if (accessibleFocus != nullptr)
                    accessibleFocus->grabFocus();
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (key.isKeyCode (KeyPress::downKey))
                {
                    selectNextItem (MenuSelectionDirection::forwards);
                }
                else if (key.isKeyCode (KeyPress::upKey))
                {
                    selectNextItem (MenuSelectionDirection::backwards);
                }
                else if (key.isKeyCode (KeyPress::leftKey))
                {
                    if (parent != nullptr)
                    {
                        Component::SafePointer<MenuWindow> parentWindow (parent);
                        PopupMenuItemComponent* currentChildOfParent = parentWindow->currentChild;

                        hide (nullptr, true);

                        if (parentWindow != nullptr)
                            parentWindow->setCurrentlyHighlightedChild (currentChildOfParent);

                        disableTimerUntilMouseMoves();
                    }
                    else if (componentAttachedTo != nullptr)
                    {
                        componentAttachedTo->keyPressed (key);
                    }
                }
                else if (key.isKeyCode (KeyPress::rightKey))
                {
                    disableTimerUntilMouseMoves();

                    if (showSubMenuFor (currentChild))
                    {
                        if (isSubMenuVisible())
                            activeSubMenu->selectNextItem (MenuSelectionDirection::current);
                    }
                    else if (componentAttachedTo != nullptr)
                    {
                        componentAttachedTo->keyPressed (key);
                    }
                }
                else if (key.isKeyCode (KeyPress::returnKey) || key.isKeyCode (KeyPress::spaceKey))
                {
                    triggerCurrentlyHighlightedItem();
                }
                else if (key.isKeyCode (KeyPress::escapeKey))
                {
                    dismissMenu (nullptr);
                }
                else
                {
                    return false;
                }

                return true;
        */
    }
    
    pub fn input_attempt_when_modal(&mut self)  {
        
        todo!();
        /*
            WeakReference<Component> deletionChecker (this);

                for (auto* ms : mouseSourceStates)
                {
                    ms->timerCallback();

                    if (deletionChecker == nullptr)
                        return;
                }

                if (! isOverAnyMenu())
                {
                    if (componentAttachedTo != nullptr)
                    {
                        // we want to dismiss the menu, but if we do it synchronously, then
                        // the mouse-click will be allowed to pass through. That's good, except
                        // when the user clicks on the button that originally popped the menu up,
                        // as they'll expect the menu to go away, and in fact it'll just
                        // come back. So only dismiss synchronously if they're not on the original
                        // comp that we're attached to.
                        auto mousePos = componentAttachedTo->getMouseXYRelative();

                        if (componentAttachedTo->reallyContains (mousePos, true))
                        {
                            postCommandMessage (PopupMenuSettings::dismissCommandId); // dismiss asynchronously
                            return;
                        }
                    }

                    dismissMenu (nullptr);
                }
        */
    }
    
    pub fn handle_command_message(&mut self, command_id: i32)  {
        
        todo!();
        /*
            Component::handleCommandMessage (commandId);

                if (commandId == PopupMenuSettings::dismissCommandId)
                    dismissMenu (nullptr);
        */
    }
    
    pub fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            handleMouseEvent (e);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            handleMouseEvent (e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            handleMouseEvent (e);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            handleMouseEvent (e);
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        _0:    &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            alterChildYPos (roundToInt (-10.0f * wheel.deltaY * PopupMenuSettings::scrollZone));
        */
    }
    
    pub fn handle_mouse_event(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            getMouseState (e.source).handleMouseEvent (e);
        */
    }
    
    pub fn window_is_still_valid(&mut self) -> bool {
        
        todo!();
        /*
            if (! isVisible())
                    return false;

                if (componentAttachedTo != options.getTargetComponent())
                {
                    dismissMenu (nullptr);
                    return false;
                }

                if (auto* currentlyModalWindow = dynamic_cast<MenuWindow*> (Component::getCurrentlyModalComponent()))
                    if (! treeContains (currentlyModalWindow))
                        return false;

                return true;
        */
    }
    
    pub fn get_active_windows() -> &'a mut Vec<*mut MenuWindow<'a>> {
        
        todo!();
        /*
            static Vec<MenuWindow*> activeMenuWindows;
                return activeMenuWindows;
        */
    }
    
    pub fn get_mouse_state(&mut self, source: MouseInputSource) -> &mut MouseSourceState {
        
        todo!();
        /*
            MouseSourceState* mouseState = nullptr;

                for (auto* ms : mouseSourceStates)
                {
                    if      (ms->source == source)                        mouseState = ms;
                    else if (ms->source.getType() != source.getType())    ms->stopTimer();
                }

                if (mouseState == nullptr)
                {
                    mouseState = new MouseSourceState (*this, source);
                    mouseSourceStates.add (mouseState);
                }

                return *mouseState;
        */
    }
    
    pub fn is_over_any_menu(&self) -> bool {
        
        todo!();
        /*
            return parent != nullptr ? parent->isOverAnyMenu()
                                         : isOverChildren();
        */
    }
    
    pub fn is_over_children(&self) -> bool {
        
        todo!();
        /*
            return isVisible()
                        && (isAnyMouseOver() || (activeSubMenu != nullptr && activeSubMenu->isOverChildren()));
        */
    }
    
    pub fn is_any_mouse_over(&self) -> bool {
        
        todo!();
        /*
            for (auto* ms : mouseSourceStates)
                    if (ms->isOver())
                        return true;

                return false;
        */
    }
    
    pub fn tree_contains(&self, window: *const MenuWindow) -> bool {
        
        todo!();
        /*
            auto* mw = this;

                while (mw->parent != nullptr)
                    mw = mw->parent;

                while (mw != nullptr)
                {
                    if (mw == window)
                        return true;

                    mw = mw->activeSubMenu.get();
                }

                return false;
        */
    }
    
    pub fn does_any_aloe_comp_have_focus(&mut self) -> bool {
        
        todo!();
        /*
            if (! isForegroundOrEmbeddedProcess (componentAttachedTo))
                    return false;

                if (Component::getCurrentlyFocusedComponent() != nullptr)
                    return true;

                for (int i = ComponentPeer::getNumPeers(); --i >= 0;)
                {
                    if (ComponentPeer::getPeer (i)->isFocused())
                    {
                        hasAnyAloeCompHadFocus = true;
                        return true;
                    }
                }

                return ! hasAnyAloeCompHadFocus;
        */
    }
    
    pub fn get_parent_area(
        &mut self, 
        target_point: Point<i32>,
        relative_to:  *mut Component

    ) -> Rectangle<i32> {

        todo!();
        /*
            if (relativeTo != nullptr)
                    targetPoint = relativeTo->localPointToGlobal (targetPoint);

                auto parentArea = Desktop::getInstance().getDisplays().getDisplayForPoint (targetPoint * scaleFactor)
                                      #if ALOE_MAC || ALOE_ANDROID
                                       ->userArea;
                                      #else
                                       ->totalArea; // on windows, don't stop the menu overlapping the taskbar
                                      #endif

                if (parentComponent == nullptr)
                    return parentArea;

                return parentComponent->getLocalArea (nullptr,
                                                      parentComponent->getScreenBounds()
                                                            .reduced (getLookAndFeel().getPopupMenuBorderSizeWithOptions (options))
                                                            .getIntersection (parentArea));
        */
    }
    
    pub fn calculate_window_pos(&mut self, 
        target:             Rectangle<i32>,
        align_to_rectangle: bool)  {
        
        todo!();
        /*
            auto parentArea = getParentArea (target.getCentre()) / scaleFactor;

                if (parentComponent != nullptr)
                    target = parentComponent->getLocalArea (nullptr, target).getIntersection (parentArea);

                auto maxMenuHeight = parentArea.getHeight() - 24;

                int x, y, widthToUse, heightToUse;
                layoutMenuItems (parentArea.getWidth() - 24, maxMenuHeight, widthToUse, heightToUse);

                if (alignToRectangle)
                {
                    x = target.getX();

                    auto spaceUnder = parentArea.getBottom() - target.getBottom();
                    auto spaceOver = target.getY() - parentArea.getY();
                    auto bufferHeight = 30;

                    if (options.getPreferredPopupDirection() == PopupMenuOptions::PopupDirection::upwards)
                        y = (heightToUse < spaceOver - bufferHeight  || spaceOver >= spaceUnder) ? target.getY() - heightToUse
                                                                                                 : target.getBottom();
                    else
                        y = (heightToUse < spaceUnder - bufferHeight || spaceUnder >= spaceOver) ? target.getBottom()
                                                                                                 : target.getY() - heightToUse;
                }
                else
                {
                    bool tendTowardsRight = target.getCentreX() < parentArea.getCentreX();

                    if (parent != nullptr)
                    {
                        if (parent->parent != nullptr)
                        {
                            const bool parentGoingRight = (parent->getX() + parent->getWidth() / 2
                                                            > parent->parent->getX() + parent->parent->getWidth() / 2);

                            if (parentGoingRight && target.getRight() + widthToUse < parentArea.getRight() - 4)
                                tendTowardsRight = true;
                            else if ((! parentGoingRight) && target.getX() > widthToUse + 4)
                                tendTowardsRight = false;
                        }
                        else if (target.getRight() + widthToUse < parentArea.getRight() - 32)
                        {
                            tendTowardsRight = true;
                        }
                    }

                    auto biggestSpace = jmax (parentArea.getRight() - target.getRight(),
                                              target.getX() - parentArea.getX()) - 32;

                    if (biggestSpace < widthToUse)
                    {
                        layoutMenuItems (biggestSpace + target.getWidth() / 3, maxMenuHeight, widthToUse, heightToUse);

                        if (numColumns > 1)
                            layoutMenuItems (biggestSpace - 4, maxMenuHeight, widthToUse, heightToUse);

                        tendTowardsRight = (parentArea.getRight() - target.getRight()) >= (target.getX() - parentArea.getX());
                    }

                    x = tendTowardsRight ? jmin (parentArea.getRight() - widthToUse - 4, target.getRight())
                                         : jmax (parentArea.getX() + 4, target.getX() - widthToUse);

                    if (getLookAndFeel().getPopupMenuBorderSizeWithOptions (options) == 0) // workaround for dismissing the window on mouse up when border size is 0
                        x += tendTowardsRight ? 1 : -1;

                    const auto border = getLookAndFeel().getPopupMenuBorderSizeWithOptions (options);
                    y = target.getCentreY() > parentArea.getCentreY() ? jmax (parentArea.getY(), target.getBottom() - heightToUse) + border
                                                                      : target.getY() - border;
                }

                x = jmax (parentArea.getX() + 1, jmin (parentArea.getRight()  - (widthToUse  + 6), x));
                y = jmax (parentArea.getY() + 1, jmin (parentArea.getBottom() - (heightToUse + 6), y));

                windowPos.setBounds (x, y, widthToUse, heightToUse);

                // sets this flag if it's big enough to obscure any of its parent menus
                hideOnExit = parent != nullptr
                              && parent->windowPos.intersects (windowPos.expanded (-4, -4));
        */
    }
    
    pub fn layout_menu_items(&mut self, 
        max_menuw: i32,
        max_menuh: i32,
        width:     &mut i32,
        height:    &mut i32)  {
        
        todo!();
        /*
            // Ensure we don't try to add an empty column after the final item
                if (auto* last = items.getLast())
                    last->item.shouldBreakAfter = false;

                const auto isBreak = [] (const PopupMenuItemComponent* item) { return item->item.shouldBreakAfter; };
                const auto numBreaks = static_cast<int> (std::count_if (items.begin(), items.end(), isBreak));
                numColumns = numBreaks + 1;

                if (numBreaks == 0)
                    insertColumnBreaks (maxMenuW, maxMenuH);

                workOutManualSize (maxMenuW);
                height = jmin (contentHeight, maxMenuH);

                needsToScroll = contentHeight > height;

                width = updateYPositions();
        */
    }
    
    pub fn insert_column_breaks(&mut self, 
        max_menuw: i32,
        max_menuh: i32)  {
        
        todo!();
        /*
            numColumns = options.getMinimumNumColumns();
                contentHeight = 0;

                auto maximumNumColumns = options.getMaximumNumColumns() > 0 ? options.getMaximumNumColumns() : 7;

                for (;;)
                {
                    auto totalW = workOutBestSize (maxMenuW);

                    if (totalW > maxMenuW)
                    {
                        numColumns = jmax (1, numColumns - 1);
                        workOutBestSize (maxMenuW); // to update col widths
                        break;
                    }

                    if (totalW > maxMenuW / 2
                        || contentHeight < maxMenuH
                        || numColumns >= maximumNumColumns)
                        break;

                    ++numColumns;
                }

                const auto itemsPerColumn = (items.size() + numColumns - 1) / numColumns;

                for (auto i = 0;; i += itemsPerColumn)
                {
                    const auto breakIndex = i + itemsPerColumn - 1;

                    if (breakIndex >= items.size())
                        break;

                    items[breakIndex]->item.shouldBreakAfter = true;
                }

                if (! items.isEmpty())
                    (*std::prev (items.end()))->item.shouldBreakAfter = false;
        */
    }
    
    pub fn correct_column_widths(&mut self, max_menuw: i32) -> i32 {
        
        todo!();
        /*
            auto totalW = std::accumulate (columnWidths.begin(), columnWidths.end(), 0);
                const auto minWidth = jmin (maxMenuW, options.getMinimumWidth());

                if (totalW < minWidth)
                {
                    totalW = minWidth;

                    for (auto& column : columnWidths)
                        column = totalW / numColumns;
                }

                return totalW;
        */
    }
    
    pub fn work_out_manual_size(&mut self, max_menuw: i32)  {
        
        todo!();
        /*
            contentHeight = 0;
                columnWidths.clear();

                for (auto it = items.begin(), end = items.end(); it != end;)
                {
                    const auto isBreak = [] (const PopupMenuItemComponent* item) { return item->item.shouldBreakAfter; };
                    const auto nextBreak = std::find_if (it, end, isBreak);
                    const auto columnEnd = nextBreak == end ? end : std::next (nextBreak);

                    const auto getMaxWidth = [] (int acc, const PopupMenuItemComponent* item) { return jmax (acc, item->getWidth()); };
                    const auto colW = std::accumulate (it, columnEnd, options.getStandardItemHeight(), getMaxWidth);
                    const auto adjustedColW = jmin (maxMenuW / jmax (1, numColumns - 2),
                                                    colW + getLookAndFeel().getPopupMenuBorderSizeWithOptions (options) * 2);

                    const auto sumHeight = [] (int acc, const PopupMenuItemComponent* item) { return acc + item->getHeight(); };
                    const auto colH = std::accumulate (it, columnEnd, 0, sumHeight);

                    contentHeight = jmax (contentHeight, colH);
                    columnWidths.add (adjustedColW);
                    it = columnEnd;
                }

                contentHeight += getLookAndFeel().getPopupMenuBorderSizeWithOptions (options) * 2;

                correctColumnWidths (maxMenuW);
        */
    }
    
    pub fn work_out_best_size(&mut self, max_menuw: i32) -> i32 {
        
        todo!();
        /*
            contentHeight = 0;
                int childNum = 0;

                for (int col = 0; col < numColumns; ++col)
                {
                    int colW = options.getStandardItemHeight(), colH = 0;

                    auto numChildren = jmin (items.size() - childNum,
                                             (items.size() + numColumns - 1) / numColumns);

                    for (int i = numChildren; --i >= 0;)
                    {
                        colW = jmax (colW, items.getUnchecked (childNum + i)->getWidth());
                        colH += items.getUnchecked (childNum + i)->getHeight();
                    }

                    colW = jmin (maxMenuW / jmax (1, numColumns - 2),
                                 colW + getLookAndFeel().getPopupMenuBorderSizeWithOptions (options) * 2);

                    columnWidths.set (col, colW);
                    contentHeight = jmax (contentHeight, colH);

                    childNum += numChildren;
                }

                return correctColumnWidths (maxMenuW);
        */
    }
    
    pub fn ensure_item_component_is_visible(&mut self, 
        item_comp: &PopupMenuItemComponent,
        wantedy:   i32)  {
        
        todo!();
        /*
            if (windowPos.getHeight() > PopupMenuSettings::scrollZone * 4)
                {
                    auto currentY = itemComp.getY();

                    if (wantedY > 0 || currentY < 0 || itemComp.getBottom() > windowPos.getHeight())
                    {
                        if (wantedY < 0)
                            wantedY = jlimit (PopupMenuSettings::scrollZone,
                                              jmax (PopupMenuSettings::scrollZone,
                                                    windowPos.getHeight() - (PopupMenuSettings::scrollZone + itemComp.getHeight())),
                                              currentY);

                        auto parentArea = getParentArea (windowPos.getPosition(), parentComponent) / scaleFactor;
                        auto deltaY = wantedY - currentY;

                        windowPos.setSize (jmin (windowPos.getWidth(), parentArea.getWidth()),
                                           jmin (windowPos.getHeight(), parentArea.getHeight()));

                        auto newY = jlimit (parentArea.getY(),
                                            parentArea.getBottom() - windowPos.getHeight(),
                                            windowPos.getY() + deltaY);

                        deltaY -= newY - windowPos.getY();

                        childYOffset -= deltaY;
                        windowPos.setPosition (windowPos.getX(), newY);

                        updateYPositions();
                    }
                }
        */
    }
    
    pub fn resize_to_best_window_pos(&mut self)  {
        
        todo!();
        /*
            auto r = windowPos;

                if (childYOffset < 0)
                {
                    r = r.withTop (r.getY() - childYOffset);
                }
                else if (childYOffset > 0)
                {
                    auto spaceAtBottom = r.getHeight() - (contentHeight - childYOffset);

                    if (spaceAtBottom > 0)
                        r.setSize (r.getWidth(), r.getHeight() - spaceAtBottom);
                }

                setBounds (r);
                updateYPositions();
        */
    }
    
    pub fn alter_child_ypos(&mut self, delta: i32)  {
        
        todo!();
        /*
            if (canScroll())
                {
                    childYOffset += delta;

                    childYOffset = [&]
                    {
                        if (delta < 0)
                            return jmax (childYOffset, 0);

                        if (delta > 0)
                        {
                            const auto limit = contentHeight
                                                - windowPos.getHeight()
                                                + getLookAndFeel().getPopupMenuBorderSizeWithOptions (options);
                            return jmin (childYOffset, limit);
                        }

                        return childYOffset;
                    }();

                    updateYPositions();
                }
                else
                {
                    childYOffset = 0;
                }

                resizeToBestWindowPos();
                repaint();
        */
    }
    
    pub fn update_ypositions(&mut self) -> i32 {
        
        todo!();
        /*
            const auto separatorWidth = getLookAndFeel().getPopupMenuColumnSeparatorWidthWithOptions (options);
                const auto initialY = getLookAndFeel().getPopupMenuBorderSizeWithOptions (options)
                                      - (childYOffset + (getY() - windowPos.getY()));

                auto col = 0;
                auto x = 0;
                auto y = initialY;

                for (const auto& item : items)
                {
                    jassert (col < columnWidths.size());
                    const auto columnWidth = columnWidths[col];
                    item->setBounds (x, y, columnWidth, item->getHeight());
                    y += item->getHeight();

                    if (item->item.shouldBreakAfter)
                    {
                        col += 1;
                        x += columnWidth + separatorWidth;
                        y = initialY;
                    }
                }

                return std::accumulate (columnWidths.begin(), columnWidths.end(), 0)
                       + (separatorWidth * (columnWidths.size() - 1));
        */
    }
    
    pub fn set_currently_highlighted_child(&mut self, child: *mut PopupMenuItemComponent)  {
        
        todo!();
        /*
            if (currentChild != nullptr)
                    currentChild->setHighlighted (false);

                currentChild = child;

                if (currentChild != nullptr)
                {
                    currentChild->setHighlighted (true);
                    timeEnteredCurrentChildComp = Time::getApproximateMillisecondCounter();
                }

                if (auto* handler = getAccessibilityHandler())
                    handler->notifyAccessibilityEvent (AccessibilityEvent::rowSelectionChanged);
        */
    }
    
    pub fn is_sub_menu_visible(&self) -> bool {
        
        todo!();
        /*
            return activeSubMenu != nullptr && activeSubMenu->isVisible();
        */
    }
    
    pub fn show_sub_menu_for(&mut self, child_comp: *mut PopupMenuItemComponent) -> bool {
        
        todo!();
        /*
            activeSubMenu.reset();

                if (childComp != nullptr
                     && hasActiveSubMenu (childComp->item))
                {
                    activeSubMenu.reset (new HelperClasses::MenuWindow (*(childComp->item.subMenu), this,
                                                                        options.withTargetScreenArea (childComp->getScreenBounds())
                                                                               .withMinimumWidth (0)
                                                                               .withTargetComponent (nullptr)
                                                                               .withParentComponent (parentComponent),
                                                                        false, dismissOnMouseUp, managerOfChosenCommand, scaleFactor));

                    activeSubMenu->setVisible (true); // (must be called before enterModalState on Windows to avoid DropShadower confusion)
                    activeSubMenu->enterModalState (false);
                    activeSubMenu->toFront (false);
                    return true;
                }

                return false;
        */
    }
    
    pub fn trigger_currently_highlighted_item(&mut self)  {
        
        todo!();
        /*
            if (currentChild != nullptr
                     && canBeTriggered (currentChild->item)
                     && (currentChild->item.customComponent == nullptr
                          || currentChild->item.customComponent->isTriggeredAutomatically()))
                {
                    dismissMenu (&currentChild->item);
                }
        */
    }
    
    pub fn select_next_item(&mut self, direction: MenuSelectionDirection)  {
        
        todo!();
        /*
            disableTimerUntilMouseMoves();

                auto start = [&]
                {
                    auto index = items.indexOf (currentChild);

                    if (index >= 0)
                        return index;

                    return direction == MenuSelectionDirection::backwards ? items.size() - 1
                                                                          : 0;
                }();

                auto preIncrement = (direction != MenuSelectionDirection::current && currentChild != nullptr);

                for (int i = items.size(); --i >= 0;)
                {
                    if (preIncrement)
                        start += (direction == MenuSelectionDirection::backwards ? -1 : 1);

                    if (auto* mic = items.getUnchecked ((start + items.size()) % items.size()))
                    {
                        if (canBeTriggered (mic->item) || hasActiveSubMenu (mic->item))
                        {
                            setCurrentlyHighlightedChild (mic);
                            return;
                        }
                    }

                    if (! preIncrement)
                        preIncrement = true;
                }
        */
    }
    
    pub fn disable_timer_until_mouse_moves(&mut self)  {
        
        todo!();
        /*
            disableMouseMoves = true;

                if (parent != nullptr)
                    parent->disableTimerUntilMouseMoves();
        */
    }
    
    pub fn can_scroll(&self) -> bool {
        
        todo!();
        /*
            return childYOffset != 0 || needsToScroll;
        */
    }
    
    pub fn is_top_scroll_zone_active(&self) -> bool {
        
        todo!();
        /*
            return canScroll() && childYOffset > 0;
        */
    }
    
    pub fn is_bottom_scroll_zone_active(&self) -> bool {
        
        todo!();
        /*
            return canScroll() && childYOffset < contentHeight - windowPos.getHeight();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this,
                                                               AccessibilityRole::popupMenu,
                                                               AccessibilityActions().addAction (AccessibilityActionType::focus, [this]
                                                               {
                                                                   if (currentChild != nullptr)
                                                                   {
                                                                       if (auto* handler = currentChild->getAccessibilityHandler())
                                                                           handler->grabFocus();
                                                                   }
                                                                   else
                                                                   {
                                                                       selectNextItem (MenuSelectionDirection::forwards);
                                                                   }
                                                               }));
        */
    }
}
