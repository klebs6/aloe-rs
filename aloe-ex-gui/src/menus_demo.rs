crate::ix!();

/**
  | A list of the commands that this demo
  | responds to.
  |
  */
pub enum MenusDemoCommandIDs
{
    menuPositionInsideWindow = 1,
    menuPositionGlobalMenuBar,
    menuPositionBurgerMenu,
    outerColourRed,
    outerColourGreen,
    outerColourBlue,
    innerColourRed,
    innerColourGreen,
    innerColourBlue
}

/**
  | Represents the possible menu positions.
  |
  */
pub enum MenusDemoMenuBarPosition
{
    window,
    global,
    burger
}

#[no_copy]
#[leak_detector]
pub struct MenusDemo<'a> {
    base:                 Component<'a>,
    base2:                ApplicationCommandTarget,
    base3:                MenuBarModel<'a>,

    #[cfg(ALOE_DEMO_RUNNER)]
    command_manager:      &mut ApplicationCommandManager<'a>, // default = getGlobalCommandManager()

    #[cfg(not(ALOE_DEMO_RUNNER))]
    command_manager:      ApplicationCommandManager<'a>,

    menu_bar:             Box<MenuBarComponent<'a>>,
    menu_bar_position:    MenusDemoMenuBarPosition,      // default = MenusDemoMenuBarPosition::window
    side_panel:           SidePanel<'a>,            // default = { "Menu", 300, false  }
    burger_menu:          BurgerMenuComponent<'a>,
    menu_header:          BurgerMenuHeader<'a>,     // default = { sidePanel  }
    outer_command_target: MenusDemoOuterCommandTarget<'a>,   // default = { commandManager  }
}

impl<'a> Default for MenusDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            menuBar.reset (new MenuBarComponent (this));
            addAndMakeVisible (menuBar.get());
            setApplicationCommandManagerToWatch (&commandManager);
            commandManager.registerAllCommandsForTarget (this);

            // this ensures that commands invoked on the DemoRunner application are correctly
            // forwarded to this demo
            commandManager.setFirstCommandTarget (this);

            // this lets the command manager use keypresses that arrive in our window to send out commands
            addKeyListener (commandManager.getKeyMappings());

            addChildComponent (menuHeader);
            addAndMakeVisible (outerCommandTarget);
            addAndMakeVisible (sidePanel);

            setWantsKeyboardFocus (true);

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for MenusDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
           #if ALOE_MAC
            MenuBarModel::setMacMainMenu (nullptr);
           #endif

            commandManager.setFirstCommandTarget (nullptr);
         */
    }
}

impl<'a> MenusDemo<'a> {
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto b = getLocalBounds();

            if (menuBarPosition == MenusDemoMenuBarPosition::window)
            {
                menuBar->setBounds (b.removeFromTop (LookAndFeel::getDefaultLookAndFeel()
                                                                 .getDefaultMenuBarHeight()));
            }
            else if (menuBarPosition == MenusDemoMenuBarPosition::burger)
            {
                menuHeader.setBounds (b.removeFromTop (40));
            }

            outerCommandTarget.setBounds (b);
        */
    }
    
    pub fn get_menu_bar_names(&mut self) -> StringArray {
        
        todo!();
        /*
            return { "Menu Position", "Outer Colour", "Inner Colour" };
        */
    }
    
    pub fn get_menu_for_index(&mut self, 
        menu_index: i32,
        menu_name:  &String) -> PopupMenu {
        
        todo!();
        /*
            PopupMenu menu;

            if (menuIndex == 0)
            {
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::menuPositionInsideWindow);
               #if ALOE_MAC
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::menuPositionGlobalMenuBar);
               #endif
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::menuPositionBurgerMenu);
            }
            else if (menuIndex == 1)
            {
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::outerColourRed);
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::outerColourGreen);
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::outerColourBlue);
            }
            else if (menuIndex == 2)
            {
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::innerColourRed);
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::innerColourGreen);
                menu.addCommandItem (&commandManager, MenusDemoCommandIDs::innerColourBlue);
            }

            return menu;
        */
    }
    
    pub fn menu_item_selected(&mut self, 
        menu_itemid:          i32,
        top_level_menu_index: i32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | The following methods implement the
      | ApplicationCommandTarget interface,
      | allowing this window to publish a set of
      | actions it can perform, and which can be
      | mapped onto menus, keypresses, etc.
      */
    pub fn get_next_command_target(&mut self) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            return &outerCommandTarget;
        */
    }
    
    pub fn get_all_commands(&mut self, c: &mut Vec<CommandID>)  {
        
        todo!();
        /*
            Vec<CommandID> commands { MenusDemoCommandIDs::menuPositionInsideWindow,
                                        MenusDemoCommandIDs::menuPositionGlobalMenuBar,
                                        MenusDemoCommandIDs::menuPositionBurgerMenu };
            c.addArray (commands);
        */
    }
    
    pub fn get_command_info(&mut self, 
        commandid: CommandID,
        result:    &mut ApplicationCommandInfo)  {
        
        todo!();
        /*
            switch (commandID)
            {
                case MenusDemoCommandIDs::menuPositionInsideWindow:
                    result.setInfo ("Inside Window", "Places the menu bar inside the application window", "Menu", 0);
                    result.setTicked (menuBarPosition == MenusDemoMenuBarPosition::window);
                    result.addDefaultKeypress ('w', ModifierKeys::shiftModifier);
                    break;
                case MenusDemoCommandIDs::menuPositionGlobalMenuBar:
                    result.setInfo ("Global", "Uses a global menu bar", "Menu", 0);
                    result.setTicked (menuBarPosition == MenusDemoMenuBarPosition::global);
                    result.addDefaultKeypress ('g', ModifierKeys::shiftModifier);
                    break;
                case MenusDemoCommandIDs::menuPositionBurgerMenu:
                    result.setInfo ("Burger Menu", "Uses a burger menu", "Menu", 0);
                    result.setTicked (menuBarPosition == MenusDemoMenuBarPosition::burger);
                    result.addDefaultKeypress ('b', ModifierKeys::shiftModifier);
                    break;
                default:
                    break;
            }
        */
    }
    
    pub fn perform(&mut self, info: &ApplicationCommandTargetInvocationInfo) -> bool {
        
        todo!();
        /*
            switch (info.commandID)
            {
                case MenusDemoCommandIDs::menuPositionInsideWindow:
                    setMenuBarPosition (MenusDemoMenuBarPosition::window);
                    break;
                case MenusDemoCommandIDs::menuPositionGlobalMenuBar:
                    setMenuBarPosition (MenusDemoMenuBarPosition::global);
                    break;
                case MenusDemoCommandIDs::menuPositionBurgerMenu:
                    setMenuBarPosition (MenusDemoMenuBarPosition::burger);
                    break;
                default:
                    return false;
            }

            return true;
        */
    }
    
    pub fn set_menu_bar_position(&mut self, new_position: MenusDemoMenuBarPosition)  {
        
        todo!();
        /*
            if (menuBarPosition != newPosition)
            {
                menuBarPosition = newPosition;

                if (menuBarPosition != MenusDemoMenuBarPosition::burger)
                    sidePanel.showOrHide (false);

               #if ALOE_MAC
                MenuBarModel::setMacMainMenu (menuBarPosition == MenusDemoMenuBarPosition::global ? this : nullptr);
               #endif

                menuBar->setVisible   (menuBarPosition == MenusDemoMenuBarPosition::window);
                burgerMenu.setModel   (menuBarPosition == MenusDemoMenuBarPosition::burger ? this : nullptr);
                menuHeader.setVisible (menuBarPosition == MenusDemoMenuBarPosition::burger);

                sidePanel.setContent  (menuBarPosition == MenusDemoMenuBarPosition::burger ? &burgerMenu : nullptr, false);
                menuItemsChanged();

                resized();
            }
        */
    }
}
