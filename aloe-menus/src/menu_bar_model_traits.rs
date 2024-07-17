crate::ix!();

pub trait MenuBarModelInterface: 
    GetMenuBarNames 
    + GetMenuForIndex 
    + MenuItemSelected 
    + MenuBarActivated { }

/**
  | A class to receive callbacks when a MenuBarModel
  | changes.
  | 
  | @see MenuBarModel::addListener,
  | MenuBarModel::removeListener, MenuBarModel::menuItemsChanged
  |
  */
pub trait MenuBarModelListener {

    /**
      | This callback is made when items are
      | changed in the menu bar model.
      |
      */
    fn menu_bar_items_changed(&mut self, menu_bar_model: *mut MenuBarModel);

    /**
      | This callback is made when an application
      | command is invoked that is represented
      | by one of the items in the menu bar model.
      |
      */
    fn menu_command_invoked(
        &mut self, 
        menu_bar_model: *mut MenuBarModel,
        info:           &ApplicationCommandTargetInvocationInfo
    );

    /**
      | Called when the menu bar is first activated
      | or when the user finished interacting
      | with the menu bar.
      |
      */
    fn menu_bar_activated(
        &mut self, 
        menu_bar_model: *mut MenuBarModel,
        is_active:      bool
    ) {}
}

pub trait GetMenuBarNames {

    /**
      | This method must return a list of the
      | names of the menus.
      |
      */
    fn get_menu_bar_names(&mut self) -> Vec<String>;
}

pub trait GetMenuForIndex {

    /**
      | This should return the popup menu to
      | display for a given top-level menu.
      | 
      | -----------
      | @param topLevelMenuIndex
      | 
      | the index of the top-level menu to show
      | ----------
      | @param menuName
      | 
      | the name of the top-level menu item to
      | show
      |
      */
    fn get_menu_for_index(&mut self, 
            top_level_menu_index: i32,
            menu_name:            &String) -> PopupMenu;

}

pub trait MenuItemSelected {

    /**
      | This is called when a menu item has been
      | clicked on.
      | 
      | -----------
      | @param menuItemID
      | 
      | the item ID of the PopupMenu item that
      | was selected
      | ----------
      | @param topLevelMenuIndex
      | 
      | the index of the top-level menu from
      | which the item was chosen (just in case
      | you've used duplicate ID numbers on
      | more than one of the popup menus)
      |
      */
    fn menu_item_selected(&mut self, 
            menu_itemid:          i32,
            top_level_menu_index: i32);

}

pub trait MenuBarActivated {

    /**
      | This is called when the user starts/stops
      | navigating the menu bar.
      | 
      | -----------
      | @param isActive
      | 
      | true when the user starts navigating
      | the menu bar
      |
      */
    fn menu_bar_activated(&mut self, is_active: bool);
}

pub trait GetIdealSize {

    /**
      | Returns a rectangle with the size that
      | this component would like to have.
      | 
      | -----------
      | @note
      | 
      | the size which this method returns isn't
      | necessarily the one that the menu will
      | give it, as the items will be stretched
      | to have a uniform width.
      |
      */
    fn get_ideal_size(&mut self, 
            ideal_width:  &mut i32,
            ideal_height: &mut i32);


}
