crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_MenuBarComponent.h]

/**
  | A menu bar component.
  | 
  | @see MenuBarModel
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MenuBarComponent<'a> {
    base:                    Component<'a>,
    timer:                   Timer,
    model:                   *mut MenuBarModel<'a>, // default = nullptr
    item_components:         Vec<Box<MenuBarComponentAccessibleItemComponent<'a>>>,
    last_mouse_pos:          Point<i32>,
    item_under_mouse:        i32, // default = -1
    current_popup_index:     i32, // default = -1
    top_level_index_clicked: i32, // default = 0
}

impl<'a> TimerInterface for MenuBarComponent<'a> {

    fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();
        updateItemUnderMouse (getMouseXYRelative());
        */
    }
}

impl<'a> MenuBarModelListener for MenuBarComponent<'a> {

    fn menu_bar_items_changed(&mut self, _0: *mut MenuBarModel)  {
        
        todo!();
        /*
            StringArray newNames;

        if (model != nullptr)
            newNames = model->getMenuBarNames();

        auto itemsHaveChanged = [this, &newNames]
        {
            if ((int) itemComponents.size() != newNames.size())
                return true;

            for (size_t i = 0; i < itemComponents.size(); ++i)
                if (itemComponents[i]->getName() != newNames[(int) i])
                    return true;

            return false;
        }();

        if (itemsHaveChanged)
        {
            updateItemComponents (newNames);

            repaint();
            resized();
        }
        */
    }

    fn menu_command_invoked(
        &mut self, 
        _0:   *mut MenuBarModel,
        info: &ApplicationCommandTargetInvocationInfo
    )  {
        
        todo!();
        /*
            if (model == nullptr || (info.commandFlags & ApplicationCommandInfo::dontTriggerVisualFeedback) != 0)
            return;

        for (size_t i = 0; i < itemComponents.size(); ++i)
        {
            const auto menu = model->getMenuForIndex ((int) i, itemComponents[i]->getName());

            if (menu.containsCommandItem (info.commandID))
            {
                setItemUnderMouse ((int) i);
                startTimer (200);
                break;
            }
        }
        */
    }
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_MenuBarComponent.cpp]
impl<'a> Drop for MenuBarComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        setModel (nullptr);
        Desktop::getInstance().removeGlobalMouseListener (this);
 */
    }
}

impl<'a> MenuBarComponent<'a> {

    /**
      | Creates a menu bar.
      | 
      | -----------
      | @param model
      | 
      | the model object to use to control this
      | bar. You can pass omit the parameter
      | or pass nullptr into this if you like,
      | and set the model later using the setModel()
      | method.
      |
      */
    pub fn new(m: *mut MenuBarModel) -> Self {

        todo!();
        /*


            setRepaintsOnMouseActivity (true);
        setWantsKeyboardFocus (false);
        setMouseClickGrabsKeyboardFocus (false);

        setModel (m);
        */
    }
    
    /**
      | Returns the current menu bar model being
      | used.
      |
      */
    pub fn get_model(&self) -> *mut MenuBarModel {
        
        todo!();
        /*
            return model;
        */
    }
    
    /**
      | Changes the model object to use to control
      | the bar.
      | 
      | This can be a null pointer, in which case
      | the bar will be empty. Don't delete the
      | object that is passed-in while it's
      | still being used by this MenuBar.
      |
      */
    pub fn set_model(&mut self, new_model: *mut MenuBarModel)  {
        
        todo!();
        /*
            if (model != newModel)
        {
            if (model != nullptr)
                model->removeListener (this);

            model = newModel;

            if (model != nullptr)
                model->addListener (this);

            repaint();
            menuBarItemsChanged (nullptr);
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            const auto isMouseOverBar = (currentPopupIndex >= 0 || itemUnderMouse >= 0 || isMouseOver());

        getLookAndFeel().drawMenuBarBackground (g, getWidth(), getHeight(), isMouseOverBar, *this);

        if (model == nullptr)
            return;

        for (size_t i = 0; i < itemComponents.size(); ++i)
        {
            const auto& itemComponent = itemComponents[i];
            const auto itemBounds = itemComponent->getBounds();

            Graphics::ScopedSaveState ss (g);

            g.setOrigin (itemBounds.getX(), 0);
            g.reduceClipRegion (0, 0, itemBounds.getWidth(), itemBounds.getHeight());

            getLookAndFeel().drawMenuBarItem (g,
                                              itemBounds.getWidth(),
                                              itemBounds.getHeight(),
                                              (int) i,
                                              itemComponent->getName(),
                                              (int) i == itemUnderMouse,
                                              (int) i == currentPopupIndex,
                                              isMouseOverBar,
                                              *this);
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            int x = 0;

        for (size_t i = 0; i < itemComponents.size(); ++i)
        {
            auto& itemComponent = itemComponents[i];

            auto w = getLookAndFeel().getMenuBarItemWidth (*this, (int) i, itemComponent->getName());
            itemComponent->setBounds (x, 0, w, getHeight());
            x += w;
        }
        */
    }
    
    pub fn get_item_at(&mut self, p: Point<i32>) -> i32 {
        
        todo!();
        /*
            for (size_t i = 0; i < itemComponents.size(); ++i)
            if (itemComponents[i]->getBounds().contains (p) && reallyContains (p, true))
                return (int) i;

        return -1;
        */
    }
    
    pub fn repaint_menu_item(&mut self, index: i32)  {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, (int) itemComponents.size()))
        {
            auto itemBounds = itemComponents[(size_t) index]->getBounds();

            repaint (itemBounds.getX() - 2,
                     0,
                     itemBounds.getWidth() + 4,
                     itemBounds.getHeight());
        }
        */
    }
    
    pub fn set_item_under_mouse(&mut self, index: i32)  {
        
        todo!();
        /*
            if (itemUnderMouse == index)
            return;

        repaintMenuItem (itemUnderMouse);
        itemUnderMouse = index;
        repaintMenuItem (itemUnderMouse);

        if (isPositiveAndBelow (itemUnderMouse, (int) itemComponents.size()))
            if (auto* handler = itemComponents[(size_t) itemUnderMouse]->getAccessibilityHandler())
                handler->grabFocus();
        */
    }
    
    pub fn set_open_item(&mut self, index: i32)  {
        
        todo!();
        /*
            if (currentPopupIndex != index)
        {
            if (currentPopupIndex < 0 && index >= 0)
                model->handleMenuBarActivate (true);
            else if (currentPopupIndex >= 0 && index < 0)
                model->handleMenuBarActivate (false);

            repaintMenuItem (currentPopupIndex);
            currentPopupIndex = index;
            repaintMenuItem (currentPopupIndex);

            auto& desktop = Desktop::getInstance();

            if (index >= 0)
                desktop.addGlobalMouseListener (this);
            else
                desktop.removeGlobalMouseListener (this);
        }
        */
    }
    
    pub fn update_item_under_mouse(&mut self, p: Point<i32>)  {
        
        todo!();
        /*
            setItemUnderMouse (getItemAt (p));
        */
    }
    
    /**
      | Pops up one of the menu items.
      | 
      | This lets you manually open one of the
      | menus - it could be triggered by a key
      | shortcut, for example.
      |
      */
    pub fn show_menu(&mut self, index: i32)  {
        
        todo!();
        /*
            if (index != currentPopupIndex)
        {
            PopupMenu::dismissAllActiveMenus();
            menuBarItemsChanged (nullptr);

            setOpenItem (index);
            setItemUnderMouse (index);

            if (isPositiveAndBelow (index, (int) itemComponents.size()))
            {
                const auto& itemComponent = itemComponents[(size_t) index];
                auto m = model->getMenuForIndex (itemUnderMouse, itemComponent->getName());

                if (m.lookAndFeel == nullptr)
                    m.setLookAndFeel (&getLookAndFeel());

                auto itemBounds = itemComponent->getBounds();

                m.showMenuAsync (PopupMenu::Options().withTargetComponent (this)
                                                     .withTargetScreenArea (localAreaToGlobal (itemBounds))
                                                     .withMinimumWidth (itemBounds.getWidth()),
                                 [this, index] (int result) { menuDismissed (index, result); });
            }
        }
        */
    }
    
    pub fn menu_dismissed(&mut self, 
        top_level_index: i32,
        item_id:         i32)  {
        
        todo!();
        /*
            topLevelIndexClicked = topLevelIndex;
        postCommandMessage (itemId);
        */
    }
    
    pub fn handle_command_message(&mut self, command_id: i32)  {
        
        todo!();
        /*
            updateItemUnderMouse (getMouseXYRelative());

        if (currentPopupIndex == topLevelIndexClicked)
            setOpenItem (-1);

        if (commandId != 0 && model != nullptr)
            model->menuItemSelected (commandId, topLevelIndexClicked);
        */
    }
    
    pub fn mouse_enter(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.eventComponent == this)
            updateItemUnderMouse (e.getPosition());
        */
    }
    
    pub fn mouse_exit(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.eventComponent == this)
            updateItemUnderMouse (e.getPosition());
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (currentPopupIndex < 0)
        {
            updateItemUnderMouse (e.getEventRelativeTo (this).getPosition());

            currentPopupIndex = -2;
            showMenu (itemUnderMouse);
        }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            const auto item = getItemAt (e.getEventRelativeTo (this).getPosition());

        if (item >= 0)
            showMenu (item);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            const auto e2 = e.getEventRelativeTo (this);

        updateItemUnderMouse (e2.getPosition());

        if (itemUnderMouse < 0 && getLocalBounds().contains (e2.x, e2.y))
        {
            setOpenItem (-1);
            PopupMenu::dismissAllActiveMenus();
        }
        */
    }
    
    pub fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            const auto e2 = e.getEventRelativeTo (this);

        if (lastMousePos != e2.getPosition())
        {
            if (currentPopupIndex >= 0)
            {
                const auto item = getItemAt (e2.getPosition());

                if (item >= 0)
                    showMenu (item);
            }
            else
            {
                updateItemUnderMouse (e2.getPosition());
            }

            lastMousePos = e2.getPosition();
        }
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            const auto numMenus = (int) itemComponents.size();

        if (numMenus > 0)
        {
            const auto currentIndex = jlimit (0, numMenus - 1, currentPopupIndex);

            if (key.isKeyCode (KeyPress::leftKey))
            {
                showMenu ((currentIndex + numMenus - 1) % numMenus);
                return true;
            }

            if (key.isKeyCode (KeyPress::rightKey))
            {
                showMenu ((currentIndex + 1) % numMenus);
                return true;
            }
        }

        return false;
        */
    }
    
    pub fn update_item_components(&mut self, menu_names: &Vec<String>)  {
        
        todo!();
        /*
            itemComponents.clear();

        for (const auto& name : menuNames)
        {
            itemComponents.push_back (std::make_unique<MenuBarComponentAccessibleItemComponent> (*this, name));
            addAndMakeVisible (*itemComponents.back());
        }
        */
    }
    
    pub fn index_of_item_component(&self, item_component: *mut MenuBarComponentAccessibleItemComponent) -> i32 {
        
        todo!();
        /*
            const auto iter = std::find_if (itemComponents.cbegin(), itemComponents.cend(),
                                        [itemComponent] (const std::unique_ptr<MenuBarComponentAccessibleItemComponent>& c) { return c.get() == itemComponent; });

        if (iter != itemComponents.cend())
            return (int) std::distance (itemComponents.cbegin(), iter);

        jassertfalse;
        return -1;
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            struct MenuBarComponentAccessibilityHandler  : public AccessibilityHandler
        {
            explicit MenuBarComponentAccessibilityHandler (MenuBarComponent& menuBarComponent)
                : AccessibilityHandler (menuBarComponent, AccessibilityRole::menuBar)
            {
            }

            AccessibleState getCurrentState() const override  { return AccessibleState().withIgnored(); }
        };

        return std::make_unique<MenuBarComponentAccessibilityHandler> (*this);
        */
    }
}
