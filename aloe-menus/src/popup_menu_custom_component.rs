crate::ix!();

pub trait PopupMenuCustomComponentInterface : GetIdealSize {}

/**
  | A user-defined component that can be
  | used as an item in a popup menu. @see typename
  | PopupMenu::addCustomItem
  |
  */
#[no_copy]
#[leak_detector]
pub struct PopupMenuCustomComponent<'a> {
    base:                    Component<'a>,
    base2:                   SingleThreadedReferenceCountedObject,
    is_highlighted:          bool, // default = false
    triggered_automatically: bool,
    item:                    *const PopupMenuItem<'a>, // default = nullptr
}

impl<'a> PopupMenuCustomComponent<'a> {

    /**
      | Returns true if this item should be highlighted
      | because the mouse is over it.
      | 
      | You can call this method in your paint()
      | method to find out whether to draw a highlight.
      |
      */
    pub fn is_item_highlighted(&self) -> bool {
        
        todo!();
        /*
            return isHighlighted;
        */
    }

    /**
      | Returns a pointer to the PopupMenuItem that holds
      | this custom component, if this component
      | is currently held by an PopupMenuItem.
      | 
      | You can query the PopupMenuItem for information
      | that you might want to use in your paint()
      | method, such as the item's enabled and
      | ticked states.
      |
      */
    pub fn get_item(&self) -> *const PopupMenuItem {
        
        todo!();
        /*
            return item;
        */
    }
    
    pub fn is_triggered_automatically(&self) -> bool {
        
        todo!();
        /*
            return triggeredAutomatically;
        */
    }
    
    /**
      | Creates a custom item.
      | 
      | If isTriggeredAutomatically is true,
      | then the menu will automatically detect
      | a mouse-click on this component and
      | use that to invoke the menu item. If it's
      | false, then it's up to your class to manually
      | trigger the item when it wants to.
      |
      */
    pub fn new(auto_trigger: Option<bool>) -> Self {
    
        let auto_trigger: bool = auto_trigger.unwrap_or(true);

        todo!();
        /*
        : triggered_automatically(autoTrigger),

        
        */
    }
    
    pub fn set_highlighted(&mut self, should_be_highlighted: bool)  {
        
        todo!();
        /*
            isHighlighted = shouldBeHighlighted;
        repaint();
        */
    }
    
    /**
      | Dismisses the menu, indicating that
      | this item has been chosen.
      | 
      | This will cause the menu to exit from
      | its modal state, returning this item's
      | id as the result.
      |
      */
    pub fn trigger_menu_item(&mut self)  {
        
        todo!();
        /*
            if (auto* mic = findParentComponentOfClass<HelperClasses::ItemComponent>())
        {
            if (auto* pmw = mic->findParentComponentOfClass<HelperClasses::MenuWindow>())
            {
                pmw->dismissMenu (&mic->item);
            }
            else
            {
                // something must have gone wrong with the component hierarchy if this happens..
                jassertfalse;
            }
        }
        else
        {
            // why isn't this component inside a menu? Not much point triggering the item if
            // there's no menu.
            jassertfalse;
        }
        */
    }
}

