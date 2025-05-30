crate::ix!();

/**
  | A factory object which can create ToolbarItemComponent
  | objects.
  | 
  | A subclass of ToolbarItemFactory publishes
  | a set of types of toolbar item that it
  | can create.
  | 
  | Each type of item is identified by a unique
  | ID, and multiple instances of an item
  | type can exist at once (even on the same
  | toolbar, e.g. spacers or separator
  | bars).
  | 
  | @see Toolbar, ToolbarItemComponent,
  | ToolbarButton
  | 
  | @tags{GUI}
  |
  */
pub trait ToolbarItemFactory:
GetAllToolbarItemIds 
+ GetDefaultItemSet 
+ ToolbarItemComponent {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ToolbarItemFactory.h]

/**
  | A set of reserved item ID values, used
  | for the built-in item types.
  |
  */
pub enum ToolbarItemFactorySpecialItemIds
{
    /**
      | The item ID for a vertical (or horizontal)
      | separator bar that can be placed between
      | sets of items to break them into groups.
      |
      */
    separatorBarId      = -1,   

    /**
      | The item ID for a fixed-width space that
      | can be placed between items.
      |
      */
    spacerId            = -2,   

    /**
      | The item ID for a gap that pushes outwards
      | against the things on either side of
      | it, filling any available space.
      |
      */
    flexibleSpacerId    = -3,    
}

pub trait GetAllToolbarItemIds {

    /**
      | Must return a list of the IDs for all the
      | item types that this factory can create.
      | 
      | The ids should be added to the array that
      | is passed-in.
      | 
      | An item ID can be any integer you choose,
      | except for 0, which is considered a null
      | ID, and the predefined IDs in the ToolbarItemFactorySpecialItemIds
      | enum.
      | 
      | You should also add the built-in types
      | (separatorBarId, spacerId and flexibleSpacerId)
      | to this list if you want your toolbar
      | to be able to contain those items.
      | 
      | The list returned here is used by the
      | ToolbarItemPalette class to obtain
      | its list of available items, and their
      | order on the palette will reflect the
      | order in which they appear on this list.
      | 
      | @see ToolbarItemPalette
      |
      */
    fn get_all_toolbar_item_ids(&mut self, ids: &mut Vec<i32>);
}

pub trait GetDefaultItemSet {

    /**
      | Must return the set of items that should
      | be added to a toolbar as its default set.
      | 
      | This method is used by Toolbar::addDefaultItems()
      | to determine which items to create.
      | 
      | The items that your method adds to the
      | array that is passed-in will be added
      | to the toolbar in the same order. Items
      | can appear in the list more than once.
      |
      */
    fn get_default_item_set(&mut self, ids: &mut Vec<i32>);
}

pub trait ToolbarItemComponent {

    /**
      | Must create an instance of one of the
      | items that the factory lists in its getAllToolbarItemIds()
      | method.
      | 
      | The itemId parameter can be any of the
      | values listed by your getAllToolbarItemIds()
      | method, except for the built-in item
      | types from the ToolbarItemFactorySpecialItemIds enum,
      | which are created internally by the
      | toolbar code.
      | 
      | Try not to keep a pointer to the object
      | that is returned, as it will be deleted
      | automatically by the toolbar, and remember
      | that multiple instances of the same
      | item type are likely to exist at the same
      | time.
      |
      */
    fn create_item(&mut self, item_id: i32) -> *mut dyn ToolbarItemComponent;
}
