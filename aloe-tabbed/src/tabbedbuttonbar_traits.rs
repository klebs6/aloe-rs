crate::ix!();

pub trait PopupMenuClickOnTab {

    /**
      | Callback method to indicate that the
      | user has right-clicked on a tab.
      |
      */
    fn popup_menu_click_on_tab(&mut self, 
            tab_index: i32,
            tab_name:  &String);
}

pub trait CreateTabButton {

    /**
      | This creates one of the tabs.
      | 
      | If you need to use custom tab components,
      | you can override this method and return
      | your own class instead of the default.
      |
      */
    fn create_tab_button(&mut self, 
            tab_name:  &String,
            tab_index: i32) -> *mut TabBarButton;

}

pub trait GetBestTabLength {

    /**
      | Chooses the best length for the tab,
      | given the specified depth.
      | 
      | If the tab is horizontal, this should
      | return its width, and the depth specifies
      | its height. If it's vertical, it should
      | return the height, and the depth is actually
      | its width.
      |
      */
    fn get_best_tab_length(&mut self, depth: i32) -> i32;
}

pub trait CurrentTabChanged {

    /**
      | Callback method to indicate the selected
      | tab has been changed. @see setCurrentTabIndex
      |
      */
    fn current_tab_changed(&mut self, 
            new_current_tab_index: i32,
            new_current_tab_name:  &String);
}
