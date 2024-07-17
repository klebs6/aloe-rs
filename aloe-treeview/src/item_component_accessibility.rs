crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TreeViewItemComponentItemAccessibilityHandler<'a> {
    base:           AccessibilityHandler<'a>,
    item_component: &'a mut TreeViewItemComponent<'a>,
}

impl<'a> TreeViewItemComponentItemAccessibilityHandler<'a> {

    pub fn new(comp: &mut TreeViewItemComponent) -> Self {
    
        todo!();
        /*


            : AccessibilityHandler (comp,
                                        AccessibilityRole::treeItem,
                                        getAccessibilityActions (comp),
                                        { std::make_unique<TreeViewItemComponentItemAccessibilityHandlerItemCellInterface> (comp) }),
                  itemComponent (comp)
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            return itemComponent.getRepresentedItem().getAccessibilityName();
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return itemComponent.getRepresentedItem().getTooltip();
        */
    }
    
    pub fn get_current_state(&self) -> AccessibleState {
        
        todo!();
        /*
            auto& treeItem = itemComponent.getRepresentedItem();

                auto state = AccessibilityHandler::getCurrentState().withAccessibleOffscreen();

                if (auto* tree = treeItem.getOwnerView())
                {
                    if (tree->isMultiSelectEnabled())
                        state = state.withMultiSelectable();
                    else
                        state = state.withSelectable();
                }

                if (treeItem.mightContainSubItems())
                {
                    state = state.withExpandable();

                    if (treeItem.isOpen())
                        state = state.withExpanded();
                    else
                        state = state.withCollapsed();
                }

                if (treeItem.isSelected())
                    state = state.withSelected();

                return state;
        */
    }
    
    pub fn get_accessibility_actions(item_component: &mut TreeViewItemComponent) -> AccessibilityActions {
        
        todo!();
        /*
            auto onFocus = [&itemComponent]
                {
                    auto& treeItem = itemComponent.getRepresentedItem();

                    if (auto* tree = treeItem.getOwnerView())
                        tree->scrollToKeepItemVisible (&treeItem);
                };

                auto onPress = [&itemComponent]
                {
                    itemComponent.getRepresentedItem().itemClicked (generateMouseEvent (itemComponent, { ModifierKeys::leftButtonModifier }));
                };

                auto onShowMenu = [&itemComponent]
                {
                    itemComponent.getRepresentedItem().itemClicked (generateMouseEvent (itemComponent, { ModifierKeys::popupMenuClickModifier }));
                };

                auto onToggle = [&itemComponent, onFocus]
                {
                    if (auto* handler = itemComponent.getAccessibilityHandler())
                    {
                        auto isSelected = handler->getCurrentState().isSelected();

                        if (! isSelected)
                            onFocus();

                        itemComponent.getRepresentedItem().setSelected (! isSelected, true);
                    }
                };

                auto actions = AccessibilityActions().addAction (AccessibilityActionType::focus,    std::move (onFocus))
                                                     .addAction (AccessibilityActionType::press,    std::move (onPress))
                                                     .addAction (AccessibilityActionType::showMenu, std::move (onShowMenu))
                                                     .addAction (AccessibilityActionType::toggle,   std::move (onToggle));

                return actions;
        */
    }
    
    pub fn generate_mouse_event(
        item_comp: &mut TreeViewItemComponent,
        mods:      ModifierKeys

    ) -> MouseEvent<'a> {
        
        todo!();
        /*
            auto topLeft = itemComp.getRepresentedItem().getItemPosition (false).toFloat().getTopLeft();

                return { Desktop::getInstance().getMainMouseSource(), topLeft, mods,
                         MouseInputSource::invalidPressure, MouseInputSource::invalidOrientation, MouseInputSource::invalidRotation,
                         MouseInputSource::invalidTiltX, MouseInputSource::invalidTiltY,
                         &itemComp, &itemComp, Time::getCurrentTime(), topLeft, Time::getCurrentTime(), 0, false };
        */
    }
}
