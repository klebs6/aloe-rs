crate::ix!();

pub trait TextEditorInterface: 
    AddPopupMenuItems 
    + PerformPopupMenuAction 
    + ReturnPressed 
    + EscapePressed {}

pub trait ReturnPressed {

    /**
      | Can be overridden to intercept return
      | key presses directly
      |
      */
    fn return_pressed(&mut self);
}

pub trait EscapePressed {

    /**
      | Can be overridden to intercept escape
      | key presses directly
      |
      */
    fn escape_pressed(&mut self);
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
      | The default menu items will be added
      | using item IDs from the
      | 
      | StandardApplicationCommandIDs namespace.
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

/**
  | Base class for input filters that can
  | be applied to a TextEditor to restrict
  | the text that can be entered.
  |
  */
pub trait TextEditorInputFilter {

    /**
      | This method is called whenever text
      | is entered into the editor.
      | 
      | An implementation of this class should
      | should check the input string, and return
      | an edited version of it that should be
      | used.
      |
      */
    fn filter_new_text(&mut self, 
            _0:        &mut TextEditor,
            new_input: &String) -> String;
}
