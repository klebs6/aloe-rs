crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TabbedComponentButtonBar<'a> {
    base:  TabbedButtonBar<'a>,
    owner: &'a mut TabbedComponent<'a>,
}

impl<'a> TabbedComponentButtonBar<'a> {

    pub fn new(
        tab_comp: &mut TabbedComponent,
        o:        TabbedButtonBarOrientation) -> Self {
    
        todo!();
        /*
        : tabbed_button_bar(o),
        : owner(tabComp),

        
        */
    }
    
    pub fn current_tab_changed(&mut self, 
        new_current_tab_index: i32,
        new_tab_name:          &String)  {
        
        todo!();
        /*
            owner.changeCallback (newCurrentTabIndex, newTabName);
        */
    }
    
    pub fn popup_menu_click_on_tab(&mut self, 
        tab_index: i32,
        tab_name:  &String)  {
        
        todo!();
        /*
            owner.popupMenuClickOnTab (tabIndex, tabName);
        */
    }
    
    pub fn get_tab_background_colour(&mut self, tab_index: i32) -> Colour {
        
        todo!();
        /*
            return owner.tabs->getTabBackgroundColour (tabIndex);
        */
    }
    
    pub fn create_tab_button(&mut self, 
        tab_name:  &String,
        tab_index: i32) -> *mut TabBarButton {
        
        todo!();
        /*
            return owner.createTabButton (tabName, tabIndex);
        */
    }
}
