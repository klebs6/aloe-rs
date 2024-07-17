crate::ix!();

/**
  | Allows you to iterate through the items
  | in a pop-up menu, and examine their properties.
  | 
  | To use this, just create one and repeatedly
  | call its next() method. When this returns
  | true, all the member variables of the
  | iterator are filled-out with information
  | describing the menu item. When it returns
  | false, the end of the list has been reached.
  |
  */
#[leak_detector]
pub struct MenuItemIterator<'a> {
    search_recursively: bool,
    index:              Vec<i32>,
    menus:              Vec<*const PopupMenu<'a>>,
    current_item:       *mut PopupMenuItem<'a>, // default = nullptr
}

impl<'a> MenuItemIterator<'a> {

    /**
      | Creates an iterator that will scan through
      | the items in the specified menu.
      | 
      | Be careful not to add any items to a menu
      | while it is being iterated, or things
      | could get out of step.
      | 
      | -----------
      | @param menu
      | 
      | the menu that needs to be scanned
      | ----------
      | @param searchRecursively
      | 
      | if true, all submenus will be recursed
      | into to do an exhaustive search
      |
      */
    pub fn new(
        m:       &PopupMenu,
        recurse: Option<bool>

    ) -> Self {

        let recurse: bool = recurse.unwrap_or(false);
    
        todo!();
        /*
        : search_recursively(recurse),

            index.add (0);
        menus.add (&m);
        */
    }
    
    /**
      | Returns true if there is another item,
      | and sets up all this object's member
      | variables to reflect that item's properties.
      |
      */
    pub fn next(&mut self) -> bool {
        
        todo!();
        /*
            if (index.size() == 0 || menus.getLast()->items.size() == 0)
            return false;

        currentItem = const_cast<PopupMenu::PopupMenuItem*> (&(menus.getLast()->items.getReference (index.getLast())));

        if (searchRecursively && currentItem->subMenu != nullptr)
        {
            index.add (0);
            menus.add (currentItem->subMenu.get());
        }
        else
        {
            index.setUnchecked (index.size() - 1, index.getLast() + 1);
        }

        while (index.size() > 0 && index.getLast() >= (int) menus.getLast()->items.size())
        {
            index.removeLast();
            menus.removeLast();

            if (index.size() > 0)
                index.setUnchecked (index.size() - 1, index.getLast() + 1);
        }

        return true;
        */
    }
    
    /**
      | Returns a reference to the description
      | of the current item.
      | 
      | It is only valid to call this after next()
      | has returned true!
      |
      */
    pub fn get_item(&self) -> &mut PopupMenuItem {
        
        todo!();
        /*
            jassert (currentItem != nullptr);
        return *(currentItem);
        */
    }
}

