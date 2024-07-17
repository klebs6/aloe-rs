crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_MenuBarModel.h]

/**
  | A class for controlling MenuBar components.
  | 
  | This class is used to tell a MenuBar what
  | menus to show, and to respond to a menu
  | being selected.
  | 
  | @see MenuBarModel::Listener, MenuBarComponent,
  | PopupMenu
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MenuBarModel<'a> {
    base:      AsyncUpdater<'a>,
    manager:   *mut dyn CommandManagerInterface,
    listeners: ListenerList<Rc<RefCell<dyn MenuBarModelListener>>>,
}

impl<'a> ApplicationCommandManagerListener for MenuBarModel<'a> {

    fn application_command_invoked(&mut self, info: &ApplicationCommandTargetInvocationInfo)  {
        
        todo!();
        /*
            listeners.call ([this, &info] (Listener& l) { l.menuCommandInvoked (this, info); });
        */
    }
    
    fn application_command_list_changed(&mut self)  {
        
        todo!();
        /*
            menuItemsChanged();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_MenuBarModel.cpp]

impl<'a> Drop for MenuBarModel<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        setApplicationCommandManagerToWatch (nullptr);
 */
    }
}

impl<'a> Default for MenuBarModel<'a> {

    fn default() -> Self {
    
        todo!();
        /*
        : manager(nullptr),

        
        */
    }
}

impl<'a> MenuBarModel<'a> {
    
    /**
      | OSX ONLY - Sets the model that is currently
      | being shown as the main menu bar at the
      | top of the screen on the Mac.
      | 
      | You can pass nullptr to stop the current
      | model being displayed. Be careful not
      | to delete a model while it is being used.
      | 
      | An optional extra menu can be specified,
      | containing items to add to the top of
      | the apple menu. (Confusingly, the 'apple'
      | menu isn't the one with a picture of an
      | apple, it's the one next to it, with your
      | application's name at the top and the
      | services menu etc on it). When one of
      | these items is selected, the menu bar
      | model will be used to invoke it, and in
      | the menuItemSelected() callback the
      | topLevelMenuIndex parameter will
      | be -1. If you pass in an extraAppleMenuItems
      | object then newMenuBarModel must be
      | non-null.
      | 
      | If the recentItemsMenuName parameter
      | is non-empty, then any sub-menus with
      | this name will be replaced by OSX's special
      | recent-files menu.
      |
      */
    #[cfg(target_os="macos")]
    pub fn set_mac_main_menu(
        new_menu_bar_model:     *mut MenuBarModel,
        extra_apple_menu_items: *const PopupMenu,
        recent_items_menu_name: Option<&String>

    ) {

        let recent_items_menu_name: &String = recent_items_menu_name.unwrap_or(&String::new());

        todo!();
        /*
        
        */
    }

    /**
      | OSX ONLY - Returns the menu model that
      | is currently being shown as the main
      | menu bar.
      |
      */
    #[cfg(target_os="macos")]
    pub fn get_mac_main_menu() -> *mut MenuBarModel<'a> {
        
        todo!();
        /*
        
        */
    }

    /**
      | OSX ONLY - Returns the menu that was last
      | passed as the extraAppleMenuItems
      | argument to setMacMainMenu(), or nullptr
      | if none was specified.
      |
      */
    #[cfg(target_os="macos")]
    pub fn get_mac_extra_apple_items_menu() -> *const PopupMenu<'a> {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Call this when some of your menu items
      | have changed.
      | 
      | This method will cause a callback to
      | any MenuBarListener objects that are
      | registered with this model.
      | 
      | If this model is displaying items from
      | an ApplicationCommandManager, you
      | can use the setApplicationCommandManagerToWatch()
      | method to cause change messages to be
      | sent automatically when the ApplicationCommandManager
      | is changed.
      | 
      | @see addListener, removeListener,
      | MenuBarListener
      |
      */
    pub fn menu_items_changed(&mut self)  {
        
        todo!();
        /*
            triggerAsyncUpdate();
        */
    }
    
    /**
      | Tells the menu bar to listen to the specified
      | command manager, and to update itself
      | when the commands change.
      | 
      | This will also allow it to flash a menu
      | name when a command from that menu is
      | invoked using a keystroke.
      |
      */
    pub fn set_application_command_manager_to_watch(&mut self, new_manager: *mut dyn CommandManagerInterface)  {
        
        todo!();
        /*
            if (manager != newManager)
        {
            if (manager != nullptr)
                manager->removeListener (this);

            manager = newManager;

            if (manager != nullptr)
                manager->addListener (this);
        }
        */
    }
    
    /**
      | Registers a listener for callbacks
      | when the menu items in this model change.
      | 
      | The listener object will get callbacks
      | when this object's menuItemsChanged()
      | method is called.
      | 
      | @see removeListener
      |
      */
    pub fn add_listener(&mut self, new_listener: *mut dyn MenuBarModelListener)  {
        
        todo!();
        /*
            listeners.add (newListener);
        */
    }
    
    /**
      | Removes a listener. @see addListener
      |
      */
    pub fn remove_listener(&mut self, listener_to_remove: *mut dyn MenuBarModelListener)  {
        
        todo!();
        /*
            // Trying to remove a listener that isn't on the list!
        // If this assertion happens because this object is a dangling pointer, make sure you've not
        // deleted this menu model while it's still being used by something (e.g. by a MenuBarComponent)
        jassert (listeners.contains (listenerToRemove));

        listeners.remove (listenerToRemove);
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            listeners.call ([this] (Listener& l) { l.menuBarItemsChanged (this); });
        */
    }
    
    pub fn handle_menu_bar_activate(&mut self, is_active: bool)  {
        
        todo!();
        /*
            menuBarActivated (isActive);
        listeners.call ([this, isActive] (Listener& l) { l.menuBarActivated (this, isActive); });
        */
    }
    
    pub fn menu_bar_activated(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
}
