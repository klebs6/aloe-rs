crate::ix!();

pub trait PopupMenuCustomCallbackInterface {

    /**
      | Callback to indicate this item has been
      | triggered.
      | 
      | 
      | -----------
      | @return
      | 
      | true if the itemID should be sent to the
      | exitModalState method, or false if
      | it should send 0, indicating no further
      | action should be taken
      |
      */
    fn menu_item_triggered(&mut self) -> bool;
}

/**
  | A user-defined callback that can be
  | used for specific items in a popup menu.
  | @see typename PopupMenu::PopupMenuItem::customCallback
  |
  */
#[no_copy]
#[leak_detector]
pub struct PopupMenuCustomCallback<'a> {
    base:     SingleThreadedReferenceCountedObject,
    callback: &'a dyn PopupMenuCustomCallbackInterface,
}
