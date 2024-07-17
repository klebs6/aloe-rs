crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ListViewport<'a> {
    base:              Viewport<'a>,
    base2:             Timer,
    owner:             &'a mut ListBox<'a>,
    rows:              Vec<Box<ListBoxRowComponent<'a>>>,
    first_index:       i32, // default = 0
    first_whole_index: i32, // default = 0
    last_whole_index:  i32, // default = 0
    has_updated:       bool, // default = false
}

impl<'a> ListViewport<'a> {

    pub fn new(lb: &mut ListBox) -> Self {
    
        todo!();
        /*
        : owner(lb),

            setWantsKeyboardFocus (false);

                auto content = std::make_unique<Component>();
                content->setWantsKeyboardFocus (false);

                setViewedComponent (content.release());
        */
    }
    
    pub fn get_component_for_row(&self, row: i32) -> *mut ListBoxRowComponent {
        
        todo!();
        /*
            return rows [row % jmax (1, rows.size())];
        */
    }
    
    pub fn get_component_for_row_if_onscreen(&self, row: i32) -> *mut ListBoxRowComponent {
        
        todo!();
        /*
            return (row >= firstIndex && row < firstIndex + rows.size())
                         ? getComponentForRow (row) : nullptr;
        */
    }
    
    pub fn get_row_number_of_component(&self, row_component: *mut Component) -> i32 {
        
        todo!();
        /*
            const int index = getViewedComponent()->getIndexOfChildComponent (rowComponent);
                const int num = rows.size();

                for (int i = num; --i >= 0;)
                    if (((firstIndex + i) % jmax (1, num)) == index)
                        return firstIndex + i;

                return -1;
        */
    }
    
    pub fn visible_area_changed(&mut self, _0: &Rectangle<i32>)  {
        
        todo!();
        /*
            updateVisibleArea (true);

                if (auto* m = owner.getModel())
                    m->listWasScrolled();

                startTimer (50);
        */
    }
    
    pub fn update_visible_area(&mut self, make_sure_it_updates_content: bool)  {
        
        todo!();
        /*
            hasUpdated = false;

                auto& content = *getViewedComponent();
                auto newX = content.getX();
                auto newY = content.getY();
                auto newW = jmax (owner.minimumRowWidth, getMaximumVisibleWidth());
                auto newH = owner.totalItems * owner.getRowHeight();

                if (newY + newH < getMaximumVisibleHeight() && newH > getMaximumVisibleHeight())
                    newY = getMaximumVisibleHeight() - newH;

                content.setBounds (newX, newY, newW, newH);

                if (makeSureItUpdatesContent && ! hasUpdated)
                    updateContents();
        */
    }
    
    pub fn update_contents(&mut self)  {
        
        todo!();
        /*
            hasUpdated = true;
                auto rowH = owner.getRowHeight();
                auto& content = *getViewedComponent();

                if (rowH > 0)
                {
                    auto y = getViewPositionY();
                    auto w = content.getWidth();

                    const int numNeeded = 4 + getMaximumVisibleHeight() / rowH;
                    rows.removeRange (numNeeded, rows.size());

                    while (numNeeded > rows.size())
                    {
                        auto* newRow = rows.add (new ListBoxRowComponent (owner));
                        content.addAndMakeVisible (newRow);
                    }

                    firstIndex = y / rowH;
                    firstWholeIndex = (y + rowH - 1) / rowH;
                    lastWholeIndex = (y + getMaximumVisibleHeight() - 1) / rowH;

                    auto startIndex = jmax (0, firstIndex - 1);

                    for (int i = 0; i < numNeeded; ++i)
                    {
                        const int row = i + startIndex;

                        if (auto* rowComp = getComponentForRow (row))
                        {
                            rowComp->setBounds (0, row * rowH, w, rowH);
                            rowComp->update (row, owner.isRowSelected (row));
                        }
                    }
                }

                if (owner.headerComponent != nullptr)
                    owner.headerComponent->setBounds (owner.outlineThickness + content.getX(),
                                                      owner.outlineThickness,
                                                      jmax (owner.getWidth() - owner.outlineThickness * 2,
                                                            content.getWidth()),
                                                      owner.headerComponent->getHeight());
        */
    }
    
    pub fn select_row(&mut self, 
        row:               i32,
        rowh:              i32,
        dont_scroll:       bool,
        last_selected_row: i32,
        total_rows:        i32,
        is_mouse_click:    bool)  {
        
        todo!();
        /*
            hasUpdated = false;

                if (row < firstWholeIndex && ! dontScroll)
                {
                    setViewPosition (getViewPositionX(), row * rowH);
                }
                else if (row >= lastWholeIndex && ! dontScroll)
                {
                    const int rowsOnScreen = lastWholeIndex - firstWholeIndex;

                    if (row >= lastSelectedRow + rowsOnScreen
                         && rowsOnScreen < totalRows - 1
                         && ! isMouseClick)
                    {
                        setViewPosition (getViewPositionX(),
                                         jlimit (0, jmax (0, totalRows - rowsOnScreen), row) * rowH);
                    }
                    else
                    {
                        setViewPosition (getViewPositionX(),
                                         jmax (0, (row  + 1) * rowH - getMaximumVisibleHeight()));
                    }
                }

                if (! hasUpdated)
                    updateContents();
        */
    }
    
    pub fn scroll_to_ensure_row_is_onscreen(&mut self, 
        row:  i32,
        rowh: i32)  {
        
        todo!();
        /*
            if (row < firstWholeIndex)
                {
                    setViewPosition (getViewPositionX(), row * rowH);
                }
                else if (row >= lastWholeIndex)
                {
                    setViewPosition (getViewPositionX(),
                                     jmax (0, (row  + 1) * rowH - getMaximumVisibleHeight()));
                }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (isOpaque())
                    g.fillAll (owner.findColour (ListBox::backgroundColourId));
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (Viewport::respondsToKey (key))
                {
                    const int allowableMods = owner.multipleSelection ? ModifierKeys::shiftModifier : 0;

                    if ((key.getModifiers().getRawFlags() & ~allowableMods) == 0)
                    {
                        // we want to avoid these keypresses going to the viewport, and instead allow
                        // them to pass up to our listbox..
                        return false;
                    }
                }

                return Viewport::keyPressed (key);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return createIgnoredAccessibilityHandler (*this);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

                if (auto* handler = owner.getAccessibilityHandler())
                    handler->notifyAccessibilityEvent (AccessibilityEvent::structureChanged);
        */
    }
}
