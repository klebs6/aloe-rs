crate::ix!();

pub struct BurgerComponentRow<'a>
{
    is_menu_header:       bool,
    top_level_menu_index: i32,
    item:                 PopupMenuItem<'a>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_BurgerMenuComponent.h]

/**
  | A component which lists all menu items
  | and groups them into categories by their
  | respective parent menus. This kind
  | of component is often used for so-called
  | "burger" menus in mobile apps.
  | 
  | @see MenuBarModel
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct BurgerMenuComponent<'a> {
    base:                             Component<'a>,
    base2:                            ListBoxModel,
    model:                            *mut MenuBarModel<'a>, // default = nullptr
    list_box:                         ListBox<'a>, //{"BurgerMenuListBox", this};
    rows:                             Vec<BurgerComponentRow<'a>>,
    last_row_clicked:                 i32, // default = -1
    input_source_index_of_last_click: i32, // default = -1
    top_level_index_clicked:          i32, // default = -1
}

impl<'a> MenuBarModelListener for BurgerMenuComponent<'a> {

    fn menu_bar_items_changed(&mut self, menu_bar_model: *mut MenuBarModel)  {
        
        todo!();
        /*
            setModel (menuBarModel);
        */
    }
    
    fn menu_command_invoked(
        &mut self, 
        _0: *mut MenuBarModel,
        _1: &ApplicationCommandTargetInvocationInfo

    )  {
        
        todo!();
        /*
        
        */
    }
}

impl<'a> Drop for BurgerMenuComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        if (model != nullptr)
            model->removeListener (this);
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_BurgerMenuComponent.cpp]
impl<'a> BurgerMenuComponent<'a> {

    /**
      | Creates a burger menu component.
      | 
      | -----------
      | @param model
      | 
      | the model object to use to control this
      | burger menu. You can set the parameter
      | or pass nullptr into this if you like,
      | and set the model later using the setModel()
      | method.
      | 
      | @see setModel
      |
      */
    pub fn new(model_to_use: *mut MenuBarModel) -> Self {

        todo!();
        /*


            lookAndFeelChanged();
        listBox.addMouseListener (this, true);

        setModel (modelToUse);
        addAndMakeVisible (listBox);
        */
    }
    
    /**
      | Changes the model object to use to control
      | the burger menu.
      | 
      | This can be a nullptr, in which case the
      | bar will be empty. This object will not
      | be owned by the BurgerMenuComponent
      | so it is up to you to manage its lifetime.
      | 
      | Don't delete the object that is passed-in
      | while it's still being used by this MenuBar.
      | 
      | Any submenus in your MenuBarModel will
      | be recursively flattened and added
      | to the top-level burger menu section.
      |
      */
    pub fn set_model(&mut self, new_model: *mut MenuBarModel)  {
        
        todo!();
        /*
            if (newModel != model)
        {
            if (model != nullptr)
                model->removeListener (this);

            model = newModel;

            if (model != nullptr)
                model->addListener (this);

            refresh();
            listBox.updateContent();
        }
        */
    }
    
    /**
      | Returns the current burger menu model
      | being used.
      |
      */
    pub fn get_model(&self) -> *mut MenuBarModel {
        
        todo!();
        /*
            return model;
        */
    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            lastRowClicked = inputSourceIndexOfLastClick = -1;

        rows.clear();

        if (model != nullptr)
        {
            auto menuBarNames = model->getMenuBarNames();

            for (auto menuIdx = 0; menuIdx < menuBarNames.size(); ++menuIdx)
            {
                PopupMenu::Item menuItem;
                menuItem.text = menuBarNames[menuIdx];

                String ignore;
                auto menu = model->getMenuForIndex (menuIdx, ignore);

                rows.add (BurgerComponentRow { true, menuIdx, menuItem });
                addMenuBarItemsForMenu (menu, menuIdx);
            }
        }
        */
    }
    
    pub fn add_menu_bar_items_for_menu(&mut self, 
        menu:     &mut PopupMenu,
        menu_idx: i32)  {
        
        todo!();
        /*
            for (PopupMenu::MenuItemIterator it (menu); it.next();)
        {
            auto& item = it.getItem();

            if (item.isSeparator)
                continue;

            if (hasSubMenu (item))
                addMenuBarItemsForMenu (*item.subMenu, menuIdx);
            else
                rows.add (BurgerComponentRow {false, menuIdx, it.getItem()});
        }
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return rows.size();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawPopupMenuBackground (g, getWidth(), getHeight());
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row_index: i32,
        g:         &mut Graphics,
        w:         i32,
        h:         i32,
        highlight: bool)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();
        Rectangle<int> r (w, h);

        auto row = (rowIndex < rows.size() ? rows.getReference (rowIndex)
                                           : BurgerComponentRow { true, 0, {} });

        g.fillAll (findColour (PopupMenu::backgroundColourId));

        if (row.isMenuHeader)
        {
            lf.drawPopupMenuSectionHeader (g, r.reduced (20, 0), row.item.text);
            g.setColour (Colours::grey);
            g.fillRect (r.withHeight (1));
        }
        else
        {
            auto& item = row.item;
            auto* colour = item.colour != Colour() ? &item.colour : nullptr;

            if (item.customComponent == nullptr)
                lf.drawPopupMenuItem (g, r.reduced (20, 0),
                                      item.isSeparator,
                                      item.isEnabled,
                                      highlight,
                                      item.isTicked,
                                      hasSubMenu (item),
                                      item.text,
                                      item.shortcutKeyDescription,
                                      item.image.get(),
                                      colour);
        }
        */
    }
    
    pub fn has_sub_menu(&mut self, item: &PopupMenuItem) -> bool {
        
        todo!();
        /*
            return item.subMenu != nullptr && (item.itemID == 0 || item.subMenu->getNumItems() > 0);
        */
    }
    
    pub fn list_box_item_clicked(&mut self, 
        row_index: i32,
        e:         &MouseEvent)  {
        
        todo!();
        /*
            auto row = rowIndex < rows.size() ? rows.getReference (rowIndex)
                                          : BurgerComponentRow { true, 0, {} };

        if (! row.isMenuHeader)
        {
            lastRowClicked = rowIndex;
            inputSourceIndexOfLastClick = e.source.getIndex();
        }
        */
    }
    
    pub fn refresh_component_for_row(&mut self, 
        row_index:       i32,
        is_row_selected: bool,
        existing:        *mut Component) -> *mut Component {
        
        todo!();
        /*
            auto row = rowIndex < rows.size() ? rows.getReference (rowIndex)
                                          : BurgerComponentRow { true, 0, {} };

        auto hasCustomComponent = (row.item.customComponent != nullptr);

        if (existing == nullptr && hasCustomComponent)
            return new CustomMenuBarItemHolder (row.item.customComponent);

        if (existing != nullptr)
        {
            auto* componentToUpdate = dynamic_cast<CustomMenuBarItemHolder*> (existing);
            jassert (componentToUpdate != nullptr);

            if (hasCustomComponent && componentToUpdate != nullptr)
            {
                row.item.customComponent->setHighlighted (isRowSelected);
                componentToUpdate->update (row.item.customComponent);
            }
            else
            {
                delete existing;
                existing = nullptr;
            }
        }

        return existing;
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            listBox.setBounds (getLocalBounds());
        */
    }
    
    pub fn mouse_up(&mut self, event: &MouseEvent)  {
        
        todo!();
        /*
            auto rowIndex = listBox.getSelectedRow();

        if (rowIndex == lastRowClicked && rowIndex < rows.size()
             && event.source.getIndex() == inputSourceIndexOfLastClick)
        {
            auto& row = rows.getReference (rowIndex);

            if (! row.isMenuHeader)
            {
                listBox.selectRow (-1);

                lastRowClicked = -1;
                inputSourceIndexOfLastClick = -1;

                topLevelIndexClicked = row.topLevelMenuIndex;
                auto& item = row.item;

                if (auto* managerOfChosenCommand = item.commandManager)
                {
                    ApplicationCommandTarget::InvocationInfo info (item.itemID);
                    info.invocationMethod = ApplicationCommandTarget::InvocationInfo::fromMenu;

                    managerOfChosenCommand->invoke (info, true);
                }

                postCommandMessage (item.itemID);
            }
        }
        */
    }
    
    pub fn handle_command_message(&mut self, commandid: i32)  {
        
        todo!();
        /*
            if (model != nullptr)
        {
            model->menuItemSelected (commandID, topLevelIndexClicked);
            topLevelIndexClicked = -1;

            refresh();
            listBox.updateContent();
        }
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            listBox.setRowHeight (roundToInt (getLookAndFeel().getPopupMenuFont().getHeight() * 2.0f));
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::menuBar);
        */
    }
}
