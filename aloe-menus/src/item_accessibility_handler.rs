crate::ix!();

pub struct PopupMenuItemComponentItemAccessibilityHandler<'a> {
    base:           AccessibilityHandler<'a>,
    item_component: &'a mut PopupMenuItemComponent<'a>,
}

impl<'a> PopupMenuItemComponentItemAccessibilityHandler<'a> {

    pub fn new(item_component_to_wrap: &mut PopupMenuItemComponent) -> Self {
    
        todo!();
        /*


            : AccessibilityHandler (itemComponentToWrap,
                    AccessibilityRole::menuItem,
                    getAccessibilityActions (*this, itemComponentToWrap)),
                    itemComponent (itemComponentToWrap)
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            return itemComponent.item.text;
        */
    }
    
    pub fn get_current_state(&self) -> AccessibleState {
        
        todo!();
        /*
            auto state = AccessibilityHandler::getCurrentState().withSelectable()
                    .withAccessibleOffscreen();

                if (hasActiveSubMenu (itemComponent.item))
                {
                    state = itemComponent.parentWindow.isSubMenuVisible() ? state.withExpandable().withExpanded()
                        : state.withExpandable().withCollapsed();
                }

                return state.isFocused() ? state.withSelected() : state;
        */
    }
    
    pub fn get_accessibility_actions(
        handler: &mut PopupMenuItemComponentItemAccessibilityHandler,
        item:    &mut PopupMenuItemComponent) -> AccessibilityActions {
        
        todo!();
        /*
            auto onFocus = [&item]
                {
                    item.parentWindow.disableTimerUntilMouseMoves();
                    item.parentWindow.ensureItemComponentIsVisible (item, -1);
                    item.parentWindow.setCurrentlyHighlightedChild (&item);
                };

                auto onPress = [&item]
                {
                    item.parentWindow.setCurrentlyHighlightedChild (&item);
                    item.parentWindow.triggerCurrentlyHighlightedItem();
                };

                auto onToggle = [&handler, &item, onFocus]
                {
                    if (handler.getCurrentState().isSelected())
                        item.parentWindow.setCurrentlyHighlightedChild (nullptr);
                    else
                        onFocus();
                };

                auto actions = AccessibilityActions().addAction (AccessibilityActionType::focus,  std::move (onFocus))
                    .addAction (AccessibilityActionType::press,  std::move (onPress))
                    .addAction (AccessibilityActionType::toggle, std::move (onToggle));

                if (hasActiveSubMenu (item.item))
                {
                    auto showSubMenu = [&item]
                    {
                        item.parentWindow.showSubMenuFor (&item);

                        if (auto* subMenu = item.parentWindow.activeSubMenu.get())
                            subMenu->setCurrentlyHighlightedChild (subMenu->items.getFirst());
                    };

                    actions.addAction (AccessibilityActionType::press,    showSubMenu);
                    actions.addAction (AccessibilityActionType::showMenu, showSubMenu);
                }

                return actions;
        */
    }
}
