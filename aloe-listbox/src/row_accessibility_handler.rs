crate::ix!();

pub struct RowAccessibilityHandler<'a> {
    base:          AccessibilityHandler<'a>,
    row_component: &'a mut ListBoxRowComponent<'a>,
}

impl<'a> RowAccessibilityHandler<'a> {

    pub fn new(row_component_to_wrap: &mut ListBoxRowComponent) -> Self {
    
        todo!();
        /*


            : AccessibilityHandler (rowComponentToWrap,
                                        AccessibilityRole::listItem,
                                        getListRowAccessibilityActions (rowComponentToWrap),
                                        { std::make_unique<RowCellInterface> (*this) }),
                  rowComponent (rowComponentToWrap)
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            if (auto* m = rowComponent.owner.getModel())
                    return m->getNameForRow (rowComponent.row);

                return {};
        */
    }
    
    pub fn get_help(&self) -> String {
        
        todo!();
        /*
            return rowComponent.getTooltip();
        */
    }
    
    pub fn get_current_state(&self) -> AccessibleState {
        
        todo!();
        /*
            if (auto* m = rowComponent.owner.getModel())
                    if (rowComponent.row >= m->getNumRows())
                        return AccessibleState().withIgnored();

                auto state = AccessibilityHandler::getCurrentState().withAccessibleOffscreen();

                if (rowComponent.owner.multipleSelection)
                    state = state.withMultiSelectable();
                else
                    state = state.withSelectable();

                if (rowComponent.isSelected)
                    state = state.withSelected();

                return state;
        */
    }
}

pub fn get_list_row_accessibility_actions<RowComponentType>(row_component: &mut RowComponentType) -> AccessibilityActions {

    todo!();
        /*
            auto onFocus = [&rowComponent]
        {
            rowComponent.owner.scrollToEnsureRowIsOnscreen (rowComponent.row);
            rowComponent.owner.selectRow (rowComponent.row);
        };

        auto onPress = [&rowComponent, onFocus]
        {
            onFocus();
            rowComponent.owner.keyPressed (KeyPress (KeyPress::returnKey));
        };

        auto onToggle = [&rowComponent]
        {
            rowComponent.owner.flipRowSelection (rowComponent.row);
        };

        return AccessibilityActions().addAction (AccessibilityActionType::focus,  std::move (onFocus))
                                     .addAction (AccessibilityActionType::press,  std::move (onPress))
                                     .addAction (AccessibilityActionType::toggle, std::move (onToggle));
        */
}

