crate::ix!();

pub trait CodeEditorComponentInterface: 
    CopyToClipboard 
    + CutToClipboard 
    + HandleReturnKey
    + HandleTabKey
    + HandleEscapeKey
    + EditorViewportPositionChanged
    + CaretPositionMoved
    + AddPopupMenuItems
    + PerformPopupMenuAction
    + UpdateCaretPosition
    + PasteFromClipboard {}

pub trait HandleReturnKey {

    /**
      | Called when the return key is pressed
      | - this can be overridden for custom behaviour.
      |
      */
    fn handle_return_key(&mut self);
}

pub trait HandleTabKey {

    /**
      | Called when the tab key is pressed - this
      | can be overridden for custom behaviour.
      |
      */
    fn handle_tab_key(&mut self);
}

pub trait HandleEscapeKey {

    /**
      | Called when the escape key is pressed
      | - this can be overridden for custom behaviour.
      |
      */
    fn handle_escape_key(&mut self);
}

pub trait AddPopupMenuItems {

    /**
      | This adds the items to the popup menu.
      | 
      | By default it adds the cut/copy/paste
      | items, but you can override this if you
      | need to replace these with your own items.
      | 
      | If you want to add your own items to the
      | existing ones, you can override this,
      | call the base class's addPopupMenuItems()
      | method, then append your own items.
      | 
      | When the menu has been shown, performPopupMenuAction()
      | will be called to perform the item that
      | the user has chosen.
      | 
      | If this was triggered by a mouse-click,
      | the mouseClickEvent parameter will
      | be a pointer to the info about it, or may
      | be null if the menu is being triggered
      | by some other means.
      | 
      | @see performPopupMenuAction, setPopupMenuEnabled,
      | isPopupMenuEnabled
      |
      */
    fn add_popup_menu_items(&mut self, 
            menu_to_add_to:    &mut PopupMenu,
            mouse_click_event: *const MouseEvent);
}

pub trait PerformPopupMenuAction {

    /**
      | This is called to perform one of the items
      | that was shown on the popup menu.
      | 
      | If you've overridden addPopupMenuItems(),
      | you should also override this to perform
      | the actions that you've added.
      | 
      | If you've overridden addPopupMenuItems()
      | but have still left the default items
      | on the menu, remember to call the superclass's
      | performPopupMenuAction() so that
      | it can perform the default actions if
      | that's what the user clicked on.
      | 
      | @see addPopupMenuItems, setPopupMenuEnabled,
      | isPopupMenuEnabled
      |
      */
    fn perform_popup_menu_action(&mut self, menu_itemid: i32);
}

pub trait CopyToClipboard {

    fn copy_to_clipboard(&mut self) -> bool;
}

pub trait CutToClipboard {

    fn cut_to_clipboard(&mut self) -> bool;
}

pub trait PasteFromClipboard {

    fn paste_from_clipboard(&mut self) -> bool;
}

pub trait CaretPositionMoved {

    /**
      | Called when the caret position moves.
      |
      */
    fn caret_position_moved(&mut self);
}

pub trait UpdateCaretPosition {

    fn update_caret_position(&mut self);
}
