crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstcontextmenu.h]

/**
  | Context Menu interface: Vst::IContextMenu
  | \ingroup vstIHost vst350
  | 
  | - [host imp]
  | 
  | - [create with IComponentHandler3::createContextMenu(..)]
  | 
  | - [released: 3.5.0]
  | 
  | - [optional]
  | 
  | A context menu is composed of Item (entry).
  | A Item is defined by a name, a tag, a flag
  | and a associated target (called when
  | this item will be selected/executed).
  | 
  | With IContextMenu the plug-in can retrieve
  | a Item, add a Item, remove a Item and pop-up
  | the menu.
  | 
  | \see IComponentHandler3 for more information.
  |
  */
pub trait IContextMenu: FUnknown {

    type Item = IContextMenuItem;

    /**
      | Gets the number of menu items.
      |
      */
    #[PLUGIN_API]
    fn get_item_count(&mut self) -> i32;

    /**
      | Gets a menu item and its target (target
      | could be not assigned).
      |
      */
    #[PLUGIN_API]
    fn get_item(&mut self, 
            index:  i32,
            item:   &mut Self::Item,
            target: *mut *mut dyn IContextMenuTarget) -> tresult;

    /**
      | Adds a menu item and its target.
      |
      */
    #[PLUGIN_API]
    fn add_item(&mut self, 
            item:   &Self::Item,
            target: *mut dyn IContextMenuTarget) -> tresult;

    /**
      | Removes a menu item.
      |
      */
    #[PLUGIN_API]
    fn remove_item(&mut self, 
            item:   &Self::Item,
            target: *mut dyn IContextMenuTarget) -> tresult;

    /**
      | Pop-ups the menu. Coordinates are relative
      | to the top-left position of the plug-ins
      | view.
      |
      */
    #[PLUGIN_API]
    fn popup(&mut self, 
            x: UCoord,
            y: UCoord) -> tresult;
}

lazy_static!{
    /*
    static const FUID icontext_menu_iid;
    */
}

declare_class_iid!{
    IContextMenu, 
    0x2E93C863, 
    0x0C9C4588, 
    0x97DBECF5, 
    0xAD17817D
}
