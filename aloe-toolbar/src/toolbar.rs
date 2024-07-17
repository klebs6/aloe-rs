crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_Toolbar.h]

/**
  | A toolbar component.
  | 
  | A toolbar contains a horizontal or vertical
  | strip of ToolbarItemComponents, and
  | looks after their order and layout.
  | 
  | Items (icon buttons or other custom
  | components) are added to a toolbar using
  | a
  | 
  | ToolbarItemFactory - each type of item
  | is given a unique ID number, and a toolbar
  | might contain more than one instance
  | of a particular item type.
  | 
  | Toolbars can be interactively customised,
  | allowing the user to drag the items around,
  | and to drag items onto or off the toolbar,
  | using the ToolbarItemPalette component
  | as a source of new items.
  | 
  | @see ToolbarItemFactory, ToolbarItemComponent,
  | ToolbarItemPalette
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Toolbar<'a> {
    base:                 Component<'a>,
    base2:                DragAndDropContainer<'a>,
    missing_items_button: Box<Button<'a>>,
    vertical:             bool, // default = false
    is_editing_active:    bool, // default = false
    toolbar_style:        ToolbarItemStyle, // default = iconsOnly
    items:                Vec<Box<dyn ToolbarItemComponent>>,
}

impl<'a> DragAndDropTarget for Toolbar<'a> {

}

impl<'a> ShouldDrawDragImageWhenOver for Toolbar<'a> { 

}

impl<'a> ItemDropped for Toolbar<'a> { 

    fn item_dropped(&mut self, drag_source_details: &DragAndDropTargetSourceDetails<'_>)  {
        
        todo!();
        /*
            if (auto* tc = dynamic_cast<ToolbarItemComponent*> (dragSourceDetails.sourceComponent.get()))
            tc->setState (Button::buttonNormal);
        */
    }
}

impl<'a> ItemDragExit for Toolbar<'a> { 

    fn item_drag_exit(&mut self, drag_source_details: &DragAndDropTargetSourceDetails<'_>)  {
        
        todo!();
        /*
            if (auto* tc = dynamic_cast<ToolbarItemComponent*> (dragSourceDetails.sourceComponent.get()))
        {
            if (isParentOf (tc))
            {
                items.removeObject (tc, false);
                removeChildComponent (tc);
                updateAllItemPositions (true);
            }
        }
        */
    }
}

impl<'a> ItemDragMove for Toolbar<'a> { 

    fn item_drag_move(&mut self, drag_source_details: &DragAndDropTargetSourceDetails<'_>)  {
        
        todo!();
        /*
            if (auto* tc = dynamic_cast<ToolbarItemComponent*> (dragSourceDetails.sourceComponent.get()))
        {
            if (! items.contains (tc))
            {
                if (tc->getEditingMode() == ToolbarItemComponent::editableOnPalette)
                {
                    if (auto* palette = tc->findParentComponentOfClass<ToolbarItemPalette>())
                        palette->replaceComponent (*tc);
                }
                else
                {
                    jassert (tc->getEditingMode() == ToolbarItemComponent::editableOnToolbar);
                }

                items.add (tc);
                addChildComponent (tc);
                updateAllItemPositions (true);
            }

            auto& animator = Desktop::getInstance().getAnimator();

            for (int i = getNumItems(); --i >= 0;)
            {
                auto currentIndex = items.indexOf (tc);
                auto newIndex = currentIndex;

                auto dragObjectLeft = vertical ? (dragSourceDetails.localPosition.getY() - tc->dragOffsetY)
                                               : (dragSourceDetails.localPosition.getX() - tc->dragOffsetX);
                auto dragObjectRight = dragObjectLeft + (vertical ? tc->getHeight() : tc->getWidth());

                auto current = animator.getComponentDestination (getChildComponent (newIndex));

                if (auto* prev = getNextActiveComponent (newIndex, -1))
                {
                    auto previousPos = animator.getComponentDestination (prev);

                    if (std::abs (dragObjectLeft - (vertical ? previousPos.getY() : previousPos.getX()))
                         < std::abs (dragObjectRight - (vertical ? current.getBottom() : current.getRight())))
                    {
                        newIndex = getIndexOfChildComponent (prev);
                    }
                }

                if (auto* next = getNextActiveComponent (newIndex, 1))
                {
                    auto nextPos = animator.getComponentDestination (next);

                    if (std::abs (dragObjectLeft - (vertical ? current.getY() : current.getX()))
                         > std::abs (dragObjectRight - (vertical ? nextPos.getBottom() : nextPos.getRight())))
                    {
                        newIndex = getIndexOfChildComponent (next) + 1;
                    }
                }

                if (newIndex == currentIndex)
                    break;

                items.removeObject (tc, false);
                removeChildComponent (tc);
                addChildComponent (tc, newIndex);
                items.insert (newIndex, tc);
                updateAllItemPositions (true);
            }
        }
        */
    }
}

impl<'a> ItemDragEnter for Toolbar<'a> { }

impl<'a> IsInterestedInDragSource for Toolbar<'a> { 

    fn is_interested_in_drag_source(&mut self, drag_source_details: &DragAndDropTargetSourceDetails<'_>) -> bool {
        
        todo!();
        /*
            return dragSourceDetails.description == toolbarDragDescriptor && isEditingActive;
        */
    }
}

impl<'a> Drop for Toolbar<'a> {

    /**
      | Destructor.
      | 
      | Any items on the bar will be deleted when
      | the toolbar is deleted.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            items.clear();
        */
    }
}

lazy_static!{
    /*
    static const char* const toolbarDragDescriptor = "_toolbarItem_";
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_Toolbar.cpp]
impl<'a> Default for Toolbar<'a> {

    /**
      | Creates an empty toolbar component.
      | 
      | To add some icons or other components
      | to your toolbar, you'll need to create
      | a ToolbarItemFactory class that can
      | create a suitable set of
      | 
      | ToolbarItemComponents.
      | 
      | @see ToolbarItemFactory, ToolbarItemComponents
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            lookAndFeelChanged();
        addChildComponent (missingItemsButton.get());

        missingItemsButton->setAlwaysOnTop (true);
        missingItemsButton->onClick = [this] { showMissingItems(); };
        */
    }
}

impl<'a> Toolbar<'a> {
    
    /**
      | Returns true if the bar is set to be vertical,
      | or false if it's horizontal.
      | 
      | You can change the bar's orientation
      | with setVertical().
      |
      */
    pub fn is_vertical(&self) -> bool {
        
        todo!();
        /*
            return vertical;
        */
    }

    /**
      | Returns the toolbar's current style.
      | @see ToolbarItemStyle, setStyle
      |
      */
    pub fn get_style(&self) -> ToolbarItemStyle {
        
        todo!();
        /*
            return toolbarStyle;
        */
    }

    /**
      | Changes the bar's orientation. @see
      | isVertical
      |
      */
    pub fn set_vertical(&mut self, should_be_vertical: bool)  {
        
        todo!();
        /*
            if (vertical != shouldBeVertical)
        {
            vertical = shouldBeVertical;
            resized();
        }
        */
    }
    
    /**
      | Deletes all items from the bar.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            items.clear();
        resized();
        */
    }
    
    pub fn create_item(
        &mut self, 
        factory: &mut dyn ToolbarItemFactory,
        item_id: i32

    ) -> *mut dyn ToolbarItemComponent {
        
        todo!();
        /*
            if (itemId == ToolbarItemFactory::separatorBarId)    return new ToolbarSpacer (itemId, 0.1f, true);
        if (itemId == ToolbarItemFactory::spacerId)          return new ToolbarSpacer (itemId, 0.5f, false);
        if (itemId == ToolbarItemFactory::flexibleSpacerId)  return new ToolbarSpacer (itemId, 0.0f, false);

        return factory.createItem (itemId);
        */
    }
    
    pub fn add_item_internal(
        &mut self, 
        factory:      &mut dyn ToolbarItemFactory,
        item_id:      i32,
        insert_index: i32

    ) {
        
        todo!();
        /*
            // An ID can't be zero - this might indicate a mistake somewhere?
        jassert (itemId != 0);

        if (auto* tc = createItem (factory, itemId))
        {
           #if ALOE_DEBUG
            Vec<int> allowedIds;
            factory.getAllToolbarItemIds (allowedIds);

            // If your factory can create an item for a given ID, it must also return
            // that ID from its getAllToolbarItemIds() method!
            jassert (allowedIds.contains (itemId));
           #endif

            items.insert (insertIndex, tc);
            addAndMakeVisible (tc, insertIndex);
        }
        */
    }
    
    /**
      | Adds an item to the toolbar.
      | 
      | The factory's ToolbarItemFactory::createItem()
      | will be called by this method to create
      | the component that will actually be
      | added to the bar.
      | 
      | The new item will be inserted at the specified
      | index (if the index is -1, it will be added
      | to the right-hand or bottom end of the
      | bar).
      | 
      | Once added, the component will be automatically
      | deleted by this object when it is no longer
      | needed.
      | 
      | @see ToolbarItemFactory
      |
      */
    pub fn add_item(
        &mut self, 
        factory:      &mut dyn ToolbarItemFactory,
        item_id:      i32,
        insert_index: Option<i32>

    ) {

        let insert_index: i32 = insert_index.unwrap_or(-1);
        
        todo!();
        /*
            addItemInternal (factory, itemId, insertIndex);
        resized();
        */
    }
    
    /**
      | Clears this toolbar and adds to it the
      | default set of items that the specified
      | factory creates.
      | 
      | @see ToolbarItemFactory::getDefaultItemSet
      |
      */
    pub fn add_default_items(
        &mut self, 
        factory_to_use: &mut dyn ToolbarItemFactory

    ) {
        
        todo!();
        /*
            Vec<int> ids;
        factoryToUse.getDefaultItemSet (ids);

        clear();

        for (auto i : ids)
            addItemInternal (factoryToUse, i, -1);

        resized();
        */
    }
    
    /**
      | Deletes one of the items from the bar.
      |
      */
    pub fn remove_toolbar_item(&mut self, item_index: i32)  {
        
        todo!();
        /*
            items.remove (itemIndex);
        resized();
        */
    }
    
    /**
      | Removes an item from the bar and returns
      | it.
      |
      */
    pub fn remove_and_return_item(&mut self, item_index: i32) 
    -> *mut dyn ToolbarItemComponent 
    {
        todo!();
        /*
            if (auto* tc = items.removeAndReturn (itemIndex))
        {
            removeChildComponent (tc);
            resized();
            return tc;
        }

        return nullptr;
        */
    }
    
    /**
      | Returns the number of items currently
      | on the toolbar.
      | 
      | @see getItemId, getItemComponent
      |
      */
    pub fn get_num_items(&self) -> i32 {
        
        todo!();
        /*
            return items.size();
        */
    }
    
    /**
      | Returns the ID of the item with the given
      | index.
      | 
      | If the index is less than zero or greater
      | than the number of items, this will return
      | nullptr.
      | 
      | @see getNumItems
      |
      */
    pub fn get_item_id(&self, item_index: i32) -> i32 {
        
        todo!();
        /*
            if (auto* tc = getItemComponent (itemIndex))
            return tc->getItemId();

        return 0;
        */
    }
    
    /**
      | Returns the component being used for
      | the item with the given index.
      | 
      | If the index is less than zero or greater
      | than the number of items, this will return
      | nullptr.
      | 
      | @see getNumItems
      |
      */
    pub fn get_item_component(&self, item_index: i32) 
    -> *mut dyn ToolbarItemComponent 
    {
        todo!();
        /*
            return items[itemIndex];
        */
    }
    
    pub fn get_next_active_component(
        &self, 
        index: i32,
        delta: i32

    ) -> *mut dyn ToolbarItemComponent {
        
        todo!();
        /*
            for (;;)
        {
            index += delta;

            if (auto* tc = getItemComponent (index))
            {
                if (tc->isActive)
                    return tc;
            }
            else
            {
                return nullptr;
            }
        }
        */
    }
    
    /**
      | Changes the toolbar's current style.
      | @see ToolbarItemStyle, getStyle,
      | ToolbarItemComponent::setStyle
      |
      */
    pub fn set_style(&mut self, new_style: &ToolbarItemStyle)  {
        
        todo!();
        /*
            if (toolbarStyle != newStyle)
        {
            toolbarStyle = newStyle;
            updateAllItemPositions (false);
        }
        */
    }
    
    /**
      | Returns a string that represents the
      | toolbar's current set of items.
      | 
      | This lets you later restore the same
      | item layout using restoreFromString().
      | 
      | @see restoreFromString
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            String s ("TB:");

        for (int i = 0; i < getNumItems(); ++i)
            s << getItemId(i) << ' ';

        return s.trimEnd();
        */
    }
    
    /**
      | Restores a set of items that was previously
      | stored in a string by the toString()
      | method.
      | 
      | The factory object is used to create
      | any item components that are needed.
      | 
      | @see toString
      |
      */
    pub fn restore_from_string(
        &mut self, 
        factory_to_use: &mut dyn ToolbarItemFactory,
        saved_version:  &String

    ) -> bool {
        
        todo!();
        /*
            if (! savedVersion.startsWith ("TB:"))
            return false;

        StringArray tokens;
        tokens.addTokens (savedVersion.substring (3), false);

        clear();

        for (auto& t : tokens)
            addItemInternal (factoryToUse, t.getIntValue(), -1);

        resized();
        return true;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().paintToolbarBackground (g, getWidth(), getHeight(), *this);
        */
    }
    
    /**
      | Returns the depth of the bar.
      | 
      | If the bar is horizontal, this will return
      | its height; if it's vertical, it will
      | return its width.
      | 
      | @see getLength
      |
      */
    pub fn get_thickness(&self) -> i32 {
        
        todo!();
        /*
            return vertical ? getWidth() : getHeight();
        */
    }
    
    /**
      | Returns the length of the bar.
      | 
      | If the bar is horizontal, this will return
      | its width; if it's vertical, it will
      | return its height.
      | 
      | @see getThickness
      |
      */
    pub fn get_length(&self) -> i32 {
        
        todo!();
        /*
            return vertical ? getHeight() : getWidth();
        */
    }
    
    /**
      | Turns on or off the toolbar's editing
      | mode, in which its items can be rearranged
      | by the user.
      | 
      | (In most cases it's easier just to use
      | showCustomisationDialog() instead
      | of trying to enable editing directly).
      | 
      | @see ToolbarItemPalette
      |
      */
    pub fn set_editing_active(&mut self, active: bool)  {
        
        todo!();
        /*
            if (isEditingActive != active)
        {
            isEditingActive = active;
            updateAllItemPositions (false);
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            updateAllItemPositions (false);
        */
    }
    
    pub fn update_all_item_positions(&mut self, animate: bool)  {
        
        todo!();
        /*
            if (getWidth() > 0 && getHeight() > 0)
        {
            StretchableObjectResizer resizer;

            for (auto* tc : items)
            {
                tc->setEditingMode (isEditingActive ? ToolbarItemComponent::editableOnToolbar
                                                    : ToolbarItemComponent::normalMode);

                tc->setStyle (toolbarStyle);

                auto* spacer = dynamic_cast<ToolbarSpacer*> (tc);

                int preferredSize = 1, minSize = 1, maxSize = 1;

                if (tc->getToolbarItemSizes (getThickness(), isVertical(),
                                             preferredSize, minSize, maxSize))
                {
                    tc->isActive = true;
                    resizer.addItem (preferredSize, minSize, maxSize,
                                     spacer != nullptr ? spacer->getResizeOrder() : 2);
                }
                else
                {
                    tc->isActive = false;
                    tc->setVisible (false);
                }
            }

            resizer.resizeToFit (getLength());

            int totalLength = 0;

            for (int i = 0; i < resizer.getNumItems(); ++i)
                totalLength += (int) resizer.getItemSize (i);

            const bool itemsOffTheEnd = totalLength > getLength();

            auto extrasButtonSize = getThickness() / 2;
            missingItemsButton->setSize (extrasButtonSize, extrasButtonSize);
            missingItemsButton->setVisible (itemsOffTheEnd);
            missingItemsButton->setEnabled (! isEditingActive);

            if (vertical)
                missingItemsButton->setCentrePosition (getWidth() / 2,
                                                       getHeight() - 4 - extrasButtonSize / 2);
            else
                missingItemsButton->setCentrePosition (getWidth() - 4 - extrasButtonSize / 2,
                                                       getHeight() / 2);

            auto maxLength = itemsOffTheEnd ? (vertical ? missingItemsButton->getY()
                                                        : missingItemsButton->getX()) - 4
                                            : getLength();

            int pos = 0, activeIndex = 0;

            for (auto* tc : items)
            {
                if (tc->isActive)
                {
                    auto size = (int) resizer.getItemSize (activeIndex++);

                    Rectangle<int> newBounds;

                    if (vertical)
                        newBounds.setBounds (0, pos, getWidth(), size);
                    else
                        newBounds.setBounds (pos, 0, size, getHeight());

                    auto& animator = Desktop::getInstance().getAnimator();

                    if (animate)
                    {
                        animator.animateComponent (tc, newBounds, 1.0f, 200, false, 3.0, 0.0);
                    }
                    else
                    {
                        animator.cancelAnimation (tc, false);
                        tc->setBounds (newBounds);
                    }

                    pos += size;
                    tc->setVisible (pos <= maxLength
                                     && ((! tc->isBeingDragged)
                                          || tc->getEditingMode() == ToolbarItemComponent::editableOnPalette));
                }
            }
        }
        */
    }
    
    pub fn show_missing_items(&mut self)  {
        
        todo!();
        /*
            jassert (missingItemsButton->isShowing());

        if (missingItemsButton->isShowing())
        {
            PopupMenu m;
            auto comp = std::make_unique<ToolbarMissingItemsComponent> (*this, getThickness());
            m.addCustomItem (1, std::move (comp));
            m.showMenuAsync (typename PopupMenu::Options().withTargetComponent (missingItemsButton.get()));
        }
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            missingItemsButton.reset (getLookAndFeel().createToolbarMissingItemsButton (*this));
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Pops up a modal dialog box that allows
      | this toolbar to be customised by the
      | user.
      | 
      | The dialog contains a ToolbarItemPalette
      | and various controls for editing other
      | aspects of the toolbar. The dialog box
      | will be opened modally, but the method
      | will return immediately.
      | 
      | The factory is used to determine the
      | set of items that will be shown on the
      | palette.
      | 
      | The optionFlags parameter is a bitwise-or
      | of values from the ToolbarCustomisationFlags
      | enum.
      | 
      | @see ToolbarItemPalette
      |
      */
    pub fn show_customisation_dialog(
        &mut self, 
        factory:      &mut dyn ToolbarItemFactory,
        option_flags: Option<ToolbarCustomisationFlags>

    ) {

        let option_flags 
        = option_flags.unwrap_or(ToolbarCustomisationFlags::AllCustomisationOptionsEnabled);
        
        todo!();
        /*
            setEditingActive (true);

        (new ToolbarCustomisationDialog (factory, *this, optionFlags))
            ->enterModalState (true, nullptr, true);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::group);
        */
    }
}
