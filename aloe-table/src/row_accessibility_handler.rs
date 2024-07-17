crate::ix!();

pub struct RowAccessibilityHandler<'a> {
    base:          AccessibilityHandler<'a>,
    row_component: &'a mut TableListBoxRowComp<'a>,
}

impl<'a> RowAccessibilityHandler<'a> {

    pub fn new(row_comp: &mut TableListBoxRowComp<'a>) -> Self {
    
        todo!();
        /*
            : AccessibilityHandler (rowComp,
                                        AccessibilityRole::row,
                                        getListRowAccessibilityActions (rowComp),
                                        { std::make_unique<RowComponentCellInterface> (*this) }),
                  rowComponent (rowComp)
        */
    }
    
    pub fn get_title(&self) -> String {
        
        todo!();
        /*
            if (auto* m = rowComponent.owner.ListBox::model)
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

                auto state = AccessibilityHandler::getCurrentState();

                if (rowComponent.owner.multipleSelection)
                    state = state.withMultiSelectable();
                else
                    state = state.withSelectable();

                if (rowComponent.isSelected)
                    return state.withSelected();

                return state;
        */
    }
}
