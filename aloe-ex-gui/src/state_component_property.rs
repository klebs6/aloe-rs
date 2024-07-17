crate::ix!();

pub struct StateComponentProperty<'a> {
    button:       ToggleButton<'a>,
    set_state_fn: fn() -> AccessibleState,
}

impl<'a> StateComponentProperty<'a> {

    pub fn new(
        name:          &String,
        initial_state: bool,
        fn_:           fn() -> AccessibleState) -> Self {
    
        todo!();
        /*


            : button (name),
                      setStateFn (fn)

                    button.setToggleState (initialState, dontSendNotification);
        */
    }
    
    pub fn set_state(&self, s: AccessibleState) -> AccessibleState {
        
        todo!();
        /*
            return (s.*setStateFn)();
        */
    }
}

pub fn state_component_default_properties<'a>() -> [StateComponentProperty<'a>; 12] {

    todo!();

    /*
    { {
        { "Checkable",            false, &AccessibleState::withCheckable },
        { "Checked",              false, &AccessibleState::withChecked },
        { "Collapsed",            false, &AccessibleState::withCollapsed },
        { "Expandable",           false, &AccessibleState::withExpandable },
        { "Expanded",             false, &AccessibleState::withExpanded },
        { "Focusable",            true,  &AccessibleState::withFocusable },
        { "Focused",              false, &AccessibleState::withFocused },
        { "Ignored",              false, &AccessibleState::withIgnored },
        { "Selectable",           false, &AccessibleState::withSelectable },
        { "Multi-Selectable",     false, &AccessibleState::withMultiSelectable },
        { "Selected",             false, &AccessibleState::withSelected },
        { "Accessible Offscreen", false, &AccessibleState::withAccessibleOffscreen }
    } };
    */
}


