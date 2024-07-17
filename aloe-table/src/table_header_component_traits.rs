crate::ix!();

pub trait ColumnClicked {

    /**
      | This can be overridden to handle a mouse-click
      | on one of the column headers.
      | 
      | The default implementation will use
      | this click to call getSortColumnId()
      | and change the sort order.
      |
      */
    fn column_clicked(&mut self, 
            column_id: i32,
            mods:      &ModifierKeys);
}

pub trait AddMenuItems {

    /**
      | This can be overridden to add custom
      | items to the pop-up menu.
      | 
      | If you override this, you should call
      | the superclass's method to add its column
      | show/hide items, if you want them on
      | the menu as well.
      | 
      | Then to handle the result, override
      | reactToMenuItem().
      | 
      | @see reactToMenuItem
      |
      */
    fn add_menu_items(&mut self, 
            menu:              &mut PopupMenu,
            column_id_clicked: i32);
}
    
pub trait ReactToMenuItem {

    /**
      | Override this to handle any custom items
      | that you have added to the pop-up menu
      | with an addMenuItems() override.
      | 
      | If the menuReturnId isn't one of your
      | own custom menu items, you'll need to
      | call TableHeaderComponent::reactToMenuItem()
      | to allow the base class to handle the
      | items that it had added.
      | 
      | @see addMenuItems
      |
      */
    fn react_to_menu_item(&mut self, 
            menu_return_id:    i32,
            column_id_clicked: i32);
}

pub trait ShowColumnChooserMenu {

    /**
      | Can be overridden for more control over
      | the pop-up menu behaviour.
      |
      */
    fn show_column_chooser_menu(&mut self, column_id_clicked: i32);
}
