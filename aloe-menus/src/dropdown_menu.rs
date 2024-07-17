crate::ix!();

pub trait ShowDropdownMenu {

    /**
      | Shows a menu attached to the OSX menu
      | bar icon.
      |
      */
    fn show_dropdown_menu(&mut self, menu: &PopupMenu);
}
