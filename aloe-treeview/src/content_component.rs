crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TreeViewContentComponent<'a> {
    base:                       Component<'a>,
    base3:                      AsyncUpdater<'a>,
    owner:                      &'a mut TreeView<'a>,
    item_components:            Vec<Box<TreeViewItemComponent<'a>>>,
    item_under_mouse:           *mut TreeViewItemComponent<'a>, // default = nullptr
    is_dragging:                bool, // default = false
    need_selection_on_mouse_up: bool, // default = false
}

impl<'a> TooltipClient for TreeViewContentComponent<'a> {

    fn get_tooltip(&mut self) -> String {
        
        todo!();
        /*
            if (auto* itemComponent = getItemComponentAt (getMouseXYRelative()))
                return itemComponent->getRepresentedItem().getTooltip();

            return owner.getTooltip();
        */
    }
}

impl<'a> TreeViewContentComponent<'a> {

    pub fn new(tree: &mut TreeView) -> Self {
    
        todo!();
        /*
        : owner(tree),

        
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDownInternal        (e.getEventRelativeTo (this));
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseUpInternal          (e.getEventRelativeTo (this));
        */
    }
    
    pub fn mouse_double_click(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDoubleClickInternal (e.getEventRelativeTo (this));
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseDragInternal        (e.getEventRelativeTo (this));
        */
    }
    
    pub fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseMoveInternal        (e.getEventRelativeTo (this));
        */
    }
    
    pub fn mouse_exit(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            mouseExitInternal        (e.getEventRelativeTo (this));
        */
    }
    
    pub fn get_item_component_at(&mut self, p: Point<i32>) -> *mut TreeViewItemComponent {
        
        todo!();
        /*
            auto iter = std::find_if (itemComponents.cbegin(), itemComponents.cend(),
                                      [p] (const std::unique_ptr<TreeViewItemComponent>& c)
                                      {
                                          return c->getBounds().contains (p);
                                      });

            if (iter != itemComponents.cend())
                return iter->get();

            return nullptr;
        */
    }
    
    pub fn get_component_for_item(&self, item: *const TreeViewItem) -> *mut TreeViewItemComponent {
        
        todo!();
        /*
            const auto iter = std::find_if (itemComponents.begin(), itemComponents.end(),
                                            [item] (const std::unique_ptr<TreeViewItemComponent>& c)
                                            {
                                                return &c->getRepresentedItem() == item;
                                            });

            if (iter != itemComponents.end())
                return iter->get();

            return nullptr;
        */
    }
    
    pub fn item_being_deleted(&mut self, item: *const TreeViewItem)  {
        
        todo!();
        /*
            const auto iter = std::find_if (itemComponents.begin(), itemComponents.end(),
                                            [item] (const std::unique_ptr<TreeViewItemComponent>& c)
                                            {
                                                return &c->getRepresentedItem() == item;
                                            });

            if (iter != itemComponents.end())
            {
                if (isMouseDraggingInChildComp (*(iter->get())))
                    owner.hideDragHighlight();

                itemComponents.erase (iter);
            }
        */
    }
    
    pub fn update_components(&mut self)  {
        
        todo!();
        /*
            std::vector<TreeViewItemComponent*> componentsToKeep;

            for (auto* treeItem : getAllVisibleItems())
            {
                if (auto* itemComp = getComponentForItem (treeItem))
                {
                    componentsToKeep.push_back (itemComp);
                }
                else
                {
                    auto newComp = std::make_unique<TreeViewItemComponent> (*treeItem);

                    addAndMakeVisible (*newComp);
                    newComp->addMouseListener (this, true);
                    componentsToKeep.push_back (newComp.get());

                    itemComponents.push_back (std::move (newComp));
                }
            }

            for (int i = (int) itemComponents.size(); --i >= 0;)
            {
                auto& comp = itemComponents[(size_t) i];

                if (std::find (componentsToKeep.cbegin(), componentsToKeep.cend(), comp.get())
                      != componentsToKeep.cend())
                {
                    auto& treeItem = comp->getRepresentedItem();
                    comp->setBounds ({ 0, treeItem.y, getWidth(), treeItem.itemHeight });
                }
                else
                {
                    itemComponents.erase (itemComponents.begin() + i);
                }
            }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return createIgnoredAccessibilityHandler (*this);
        */
    }
    
    pub fn mouse_down_internal(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateItemUnderMouse (e);

            isDragging = false;
            needSelectionOnMouseUp = false;

            if (! isEnabled())
                return;

            if (auto* itemComponent = getItemComponentAt (e.getPosition()))
            {
                auto& item = itemComponent->getRepresentedItem();
                auto pos = item.getItemPosition (false);

                // (if the open/close buttons are hidden, we'll treat clicks to the left of the item
                // as selection clicks)
                if (e.x < pos.getX() && owner.openCloseButtonsVisible)
                {
                    // (clicks to the left of an open/close button are ignored)
                    if (e.x >= pos.getX() - owner.getIndentSize())
                        item.setOpen (! item.isOpen());
                }
                else
                {
                    // mouse-down inside the body of the item..
                    if (! owner.isMultiSelectEnabled())
                        item.setSelected (true, true);
                    else if (item.isSelected())
                        needSelectionOnMouseUp = ! e.mods.isPopupMenu();
                    else
                        selectBasedOnModifiers (item, e.mods);

                    if (e.x >= pos.getX())
                        item.itemClicked (e.withNewPosition (e.position - pos.getPosition().toFloat()));
                }
            }
        */
    }
    
    pub fn mouse_up_internal(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateItemUnderMouse (e);

            if (isEnabled() && needSelectionOnMouseUp && e.mouseWasClicked())
                if (auto* itemComponent = getItemComponentAt (e.getPosition()))
                    selectBasedOnModifiers (itemComponent->getRepresentedItem(), e.mods);
        */
    }
    
    pub fn mouse_double_click_internal(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (isEnabled() && e.getNumberOfClicks() != 3)  // ignore triple clicks
            {
                if (auto* itemComponent = getItemComponentAt (e.getPosition()))
                {
                    auto& item = itemComponent->getRepresentedItem();
                    auto pos = item.getItemPosition (false);

                    if (e.x >= pos.getX() || ! owner.openCloseButtonsVisible)
                        item.itemDoubleClicked (e.withNewPosition (e.position - pos.getPosition().toFloat()));
                }
            }
        */
    }
    
    pub fn mouse_drag_internal(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (isEnabled()
                 && ! (isDragging || e.mouseWasClicked()
                        || e.getDistanceFromDragStart() < 5
                        || e.mods.isPopupMenu()))
            {
                isDragging = true;

                if (auto* itemComponent = getItemComponentAt (e.getMouseDownPosition()))
                {
                    auto& item = itemComponent->getRepresentedItem();
                    auto pos = item.getItemPosition (false);

                    if (e.getMouseDownX() >= pos.getX())
                    {
                        auto dragDescription = item.getDragSourceDescription();

                        if (! (dragDescription.isVoid() || (dragDescription.isString() && dragDescription.toString().isEmpty())))
                        {
                            if (auto* dragContainer = DragAndDropContainer::findParentDragContainerFor (this))
                            {
                                pos.setSize (pos.getWidth(), item.itemHeight);

                                auto dragImage = Component::createComponentSnapshot (pos,
                                                                                     true,
                                                                                     Component::getApproximateScaleFactorForComponent (itemComponent));

                                dragImage.multiplyAllAlphas (0.6f);

                                auto imageOffset = pos.getPosition() - e.getPosition();
                                dragContainer->startDragging (dragDescription, &owner, dragImage, true, &imageOffset, &e.source);
                            }
                            else
                            {
                                // to be able to do a drag-and-drop operation, the treeview needs to
                                // be inside a component which is also a DragAndDropContainer.
                                jassertfalse;
                            }
                        }
                    }
                }
            }
        */
    }
    
    pub fn mouse_move_internal(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateItemUnderMouse (e);
        */
    }
    
    pub fn mouse_exit_internal(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateItemUnderMouse (e);
        */
    }
    
    pub fn is_mouse_dragging_in_child_comp(comp: &Component) -> bool {
        
        todo!();
        /*
            for (auto& ms : Desktop::getInstance().getMouseSources())
                if (ms.isDragging())
                    if (auto* underMouse = ms.getComponentUnderMouse())
                        return (&comp == underMouse || comp.isParentOf (underMouse));

            return false;
        */
    }
    
    pub fn update_item_under_mouse(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            TreeViewItemComponent* newItem = nullptr;

            if (owner.openCloseButtonsVisible)
            {
                if (auto* itemComponent = getItemComponentAt (e.getPosition()))
                {
                    auto& item = itemComponent->getRepresentedItem();
                    auto pos = item.getItemPosition (false);

                    if (e.x < pos.getX()
                        && e.x >= pos.getX() - owner.getIndentSize()
                        && item.mightContainSubItems())
                    {
                        newItem = itemComponent;
                    }
                }
            }

            if (itemUnderMouse != newItem)
            {
                auto updateItem = [] (TreeViewItemComponent* itemComp, bool isMouseOverButton)
                {
                    if (itemComp != nullptr)
                    {
                        itemComp->setMouseIsOverButton (isMouseOverButton);
                        itemComp->repaint();
                    }
                };

                updateItem (itemUnderMouse, false);
                updateItem (newItem, true);

                itemUnderMouse = newItem;
            }
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            owner.updateVisibleItems();
        */
    }
    
    pub fn select_based_on_modifiers(&mut self, 
        item:      &mut TreeViewItem,
        modifiers: ModifierKeys)  {
        
        todo!();
        /*
            TreeViewItem* firstSelected = nullptr;

            if (modifiers.isShiftDown() && ((firstSelected = owner.getSelectedItem (0)) != nullptr))
            {
                auto* lastSelected = owner.getSelectedItem (owner.getNumSelectedItems() - 1);

                if (lastSelected == nullptr)
                {
                    jassertfalse;
                    return;
                }

                auto rowStart = firstSelected->getRowNumberInTree();
                auto rowEnd = lastSelected->getRowNumberInTree();

                if (rowStart > rowEnd)
                    std::swap (rowStart, rowEnd);

                auto ourRow = item.getRowNumberInTree();
                auto otherEnd = ourRow < rowEnd ? rowStart : rowEnd;

                if (ourRow > otherEnd)
                    std::swap (ourRow, otherEnd);

                for (int i = ourRow; i <= otherEnd; ++i)
                    owner.getItemOnRow (i)->setSelected (true, false);
            }
            else
            {
                const auto cmd = modifiers.isCommandDown();
                item.setSelected ((! cmd) || ! item.isSelected(), ! cmd);
            }
        */
    }
    
    pub fn get_next_visible_item(
        item:     *mut TreeViewItem,
        forwards: bool) -> *mut TreeViewItem {
        
        todo!();
        /*
            if (item == nullptr || item->ownerView == nullptr)
                return nullptr;

            auto* nextItem = item->ownerView->getItemOnRow (item->getRowNumberInTree() + (forwards ? 1 : -1));

            return nextItem == item->ownerView->rootItem && ! item->ownerView->rootItemVisible ? nullptr
                                                                                               : nextItem;
        */
    }
    
    pub fn get_all_visible_items(&self) -> Vec<*mut TreeViewItem> {
        
        todo!();
        /*
            if (owner.rootItem == nullptr)
                return {};

            const auto visibleTop = -getY();
            const auto visibleBottom = visibleTop + getParentHeight();

            std::vector<TreeViewItem*> visibleItems;

            auto* item = [&]
            {
                auto* i = owner.rootItemVisible ? owner.rootItem
                                                : owner.rootItem->subItems.getFirst();

                while (i != nullptr && i->y < visibleTop)
                    i = getNextVisibleItem (i, true);

                return i;
            }();

            auto addOffscreenItemBuffer = [&visibleItems] (TreeViewItem* i, int num, bool forwards)
            {
                while (--num >= 0)
                {
                    i = getNextVisibleItem (i, forwards);

                    if (i == nullptr)
                        return;

                    visibleItems.push_back (i);
                }
            };

            addOffscreenItemBuffer (item, 2, false);

            while (item != nullptr && item->y < visibleBottom)
            {
                visibleItems.push_back (item);
                item = getNextVisibleItem (item, true);
            }

            if (item != nullptr)
                visibleItems.push_back (item);

            addOffscreenItemBuffer (item, 2, true);

            return visibleItems;
        */
    }
}
