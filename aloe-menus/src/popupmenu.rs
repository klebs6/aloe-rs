crate::ix!();

pub const POPUP_MENU_SCROLL_ZONE:        i32 = 24;
pub const POPUP_MENU_DISMISS_COMMAND_ID: i32 = 0x6287345f;

pub fn create_drawable_from_image(im: &Image) -> Box<Drawable> {
    
    todo!();
    /*
        if (im.isValid())
        {
            auto d = new DrawableImage();
            d->setImage (im);
            return std::unique_ptr<Drawable> (d);
        }

        return {};
    */
}

lazy_static!{
    /*
    static bool popup_menu_menu_was_hidden_because_of_app_change = false;
    */
}

pub fn popup_menu_can_be_triggered(item: &PopupMenuItem) -> bool {
    
    todo!();
    /*
        return item.isEnabled && item.itemID != 0 && ! item.isSectionHeader;
    */
}

pub fn popup_menu_has_active_sub_menu(item: &PopupMenuItem) -> bool {
    
    todo!();
    /*
        return item.isEnabled && item.subMenu != nullptr && item.subMenu->items.size() > 0;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_PopupMenu.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/menus/aloe_PopupMenu.cpp]

/** 
 | Creates and displays a popup-menu.
 |
 |  To show a popup-menu, you create one of these, add some items to it, then
 |  call its show() method, which returns the id of the item the user selects.
 |
 |  E.g. @code
 |  void MyWidget::mouseDown (const MouseEvent& e)
 |  {
 |      PopupMenu m;
 |      m.addItem (1, "item 1");
 |      m.addItem (2, "item 2");
 |
 |      const int result = m.show();
 |
 |      if (result == 0)
 |      {
 |          // user dismissed the menu without picking anything
 |      }
 |      else if (result == 1)
 |      {
 |          // user picked item 1
 |      }
 |      else if (result == 2)
 |      {
 |          // user picked item 2
 |      }
 |  }
 |  @endcode
 |
 |  Submenus are easy too: @code
 |
 |  void MyWidget::mouseDown (const MouseEvent& e)
 |  {
 |      PopupMenu subMenu;
 |      subMenu.addItem (1, "item 1");
 |      subMenu.addItem (2, "item 2");
 |
 |      PopupMenu mainMenu;
 |      mainMenu.addItem (3, "item 3");
 |      mainMenu.addSubMenu ("other choices", subMenu);
 |
 |      const int result = m.show();
 |
 |      ...etc
 |  }
 |  @endcode
 |
 |  @tags{GUI}
*/
#[leak_detector]
pub struct PopupMenu<'a> {
    items:         Vec<PopupMenuItem<'a>>,
    look_and_feel: WeakReference<Rc<RefCell<dyn LookAndFeelPopupMenuInterface>>>,
}

//TODO
pub trait LookAndFeelPopupMenuInterface {}

impl<'a> Default for PopupMenu<'a> {
    
    /**
      | Creates an empty popup menu.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl<'a> PopupMenu<'a> {

    /**
      | Creates a copy of another menu.
      |
      */
    pub fn new_from_ref(other: &PopupMenu<'a>) -> Self {
    
        todo!();
        /*
        : items(other.items),
        : look_and_feel(other.lookAndFeel),

        
        */
    }
    
    /**
      | Copies this menu from another one.
      |
      */
    pub fn assign_from_ref(&mut self, other: &PopupMenu<'a>) -> &mut PopupMenu<'a> {
        
        todo!();
        /*
            if (this != &other)
        {
            items = other.items;
            lookAndFeel = other.lookAndFeel;
        }

        return *this;
        */
    }
    
    pub fn new(other: PopupMenu<'a>) -> Self {
    
        todo!();
        /*
        : items(std::move (other.items)),
        : look_and_feel(std::move (other.lookAndFeel)),

        
        */
    }
    
    pub fn assign_from(&mut self, other: PopupMenu<'a>) -> &mut PopupMenu<'a> {
        
        todo!();
        /*
            items = std::move (other.items);
        lookAndFeel = other.lookAndFeel;
        return *this;
        */
    }
    
    /**
      | Resets the menu, removing all its items.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            items.clear();
        */
    }
    
    /**
      | Adds an item to the menu.
      | 
      | You can call this method for full control
      | over the item that is added, or use the
      | other addItem helper methods if you
      | want to pass arguments rather than creating
      | an PopupMenuItem object.
      |
      */
    pub fn add_popup_menu_item(&mut self, new_item: PopupMenuItem)  {
        
        todo!();
        /*
            // An ID of 0 is used as a return value to indicate that the user
        // didn't pick anything, so you shouldn't use it as the ID for an item..
        jassert (newItem.itemID != 0
                  || newItem.isSeparator || newItem.isSectionHeader
                  || newItem.subMenu != nullptr);

        items.add (std::move (newItem));
        */
    }
    
    /**
      | Adds an item to the menu with an action
      | callback.
      |
      */
    pub fn add_item_with_text_and_action(
        &mut self, 
        item_text: String,
        action:    fn() -> ()
    )  {
        
        todo!();
        /*
            addItem (std::move (itemText), true, false, std::move (action));
        */
    }
    
    /**
      | Adds an item to the menu with an action
      | callback.
      |
      */
    pub fn add_item_with_text_active_is_ticked_and_action(
        &mut self, 
        item_text: String,
        is_active: bool,
        is_ticked: bool,
        action:    fn() -> ()

    )  {
        
        todo!();
        /*
            PopupMenuItem i (std::move (itemText));
        i.action = std::move (action);
        i.isEnabled = isActive;
        i.isTicked = isTicked;
        addItem (std::move (i));
        */
    }
    
    /**
      | Appends a new text item for this menu
      | to show.
      | 
      | -----------
      | @param itemResultID
      | 
      | the number that will be returned from
      | the show() method if the user picks this
      | item. The value should never be zero,
      | because that's used to indicate that
      | the user didn't select anything.
      | ----------
      | @param itemText
      | 
      | the text to show.
      | ----------
      | @param isEnabled
      | 
      | if false, the item will be shown 'greyed-out'
      | and can't be picked
      | ----------
      | @param isTicked
      | 
      | if true, the item will be shown with a
      | tick next to it
      | 
      | @see addSeparator, addColouredItem,
      | addCustomItem, addSubMenu
      |
      */
    pub fn add_item_with_resultid_text_is_active_and_is_ticked(
        &mut self, 
        item_resultid: i32,
        item_text:     String,
        is_active:     Option<bool>,
        is_ticked:     Option<bool>

    )  {

        let is_active: bool = is_active.unwrap_or(true);
        let is_ticked: bool = is_ticked.unwrap_or(false);
        
        todo!();
        /*
            PopupMenuItem i (std::move (itemText));
        i.itemID = itemResultID;
        i.isEnabled = isActive;
        i.isTicked = isTicked;
        addItem (std::move (i));
        */
    }
    
    /**
      | Appends a new item with an icon.
      | 
      | -----------
      | @param itemResultID
      | 
      | the number that will be returned from
      | the show() method if the user picks this
      | item. The value should never be zero,
      | because that's used to indicate that
      | the user didn't select anything.
      | ----------
      | @param itemText
      | 
      | the text to show.
      | ----------
      | @param isEnabled
      | 
      | if false, the item will be shown 'greyed-out'
      | and can't be picked
      | ----------
      | @param isTicked
      | 
      | if true, the item will be shown with a
      | tick next to it
      | ----------
      | @param iconToUse
      | 
      | if this is a valid image, it will be displayed
      | to the left of the item.
      | 
      | @see addSeparator, addColouredItem,
      | addCustomItem, addSubMenu
      |
      */
    pub fn add_item_with_resultid_text_is_active_is_ticked_and_icon_with_image(
        &mut self, 
        item_resultid: i32,
        item_text:     String,
        is_active:     bool,
        is_ticked:     bool,
        icon_to_use:   &Image

    ) {
        
        todo!();
        /*
            addItem (itemResultID, std::move (itemText), isActive, isTicked, createDrawableFromImage (iconToUse));
        */
    }
    
    /**
      | Appends a new item with an icon.
      | 
      | -----------
      | @param itemResultID
      | 
      | the number that will be returned from
      | the show() method if the user picks this
      | item. The value should never be zero,
      | because that's used to indicate that
      | the user didn't select anything.
      | ----------
      | @param itemText
      | 
      | the text to show.
      | ----------
      | @param isEnabled
      | 
      | if false, the item will be shown 'greyed-out'
      | and can't be picked
      | ----------
      | @param isTicked
      | 
      | if true, the item will be shown with a
      | tick next to it
      | ----------
      | @param iconToUse
      | 
      | a Drawable object to use as the icon to
      | the left of the item.
      | 
      | The menu will take ownership of this
      | drawable object and will delete it later
      | when no longer needed @see addSeparator,
      | addColouredItem, addCustomItem,
      | addSubMenu
      |
      */
    pub fn add_item_with_resultid_text_is_active_is_ticked_and_icon_with_drawable(
        &mut self, 
        item_resultid: i32,
        item_text:     String,
        is_active:     bool,
        is_ticked:     bool,
        icon_to_use:   Box<Drawable>

    ) {
        
        todo!();
        /*
            PopupMenuItem i (std::move (itemText));
        i.itemID = itemResultID;
        i.isEnabled = isActive;
        i.isTicked = isTicked;
        i.image = std::move (iconToUse);
        addItem (std::move (i));
        */
    }
    
    /**
      | Adds an item that represents one of the
      | commands in a command manager object.
      | 
      | -----------
      | @param commandManager
      | 
      | the manager to use to trigger the command
      | and get information about it
      | ----------
      | @param commandID
      | 
      | the ID of the command
      | ----------
      | @param displayName
      | 
      | if this is non-empty, then this string
      | will be used instead of the command's
      | registered name
      | ----------
      | @param iconToUse
      | 
      | an optional Drawable object to use as
      | the icon to the left of the item.
      | 
      | The menu will take ownership of this
      | drawable object and will delete it later
      | when no longer needed
      |
      */
    pub fn add_command_item(&mut self, 
        command_manager: *mut dyn CommandManagerInterface,
        commandid:       CommandID,
        display_name:    String,
        icon_to_use:     Box<Drawable>)  {
        
        todo!();
        /*
            jassert (commandManager != nullptr && commandID != 0);

        if (auto* registeredInfo = commandManager->getCommandForID (commandID))
        {
            ApplicationCommandInfo info (*registeredInfo);
            auto* target = commandManager->getTargetForCommand (commandID, info);

            PopupMenuItem i;
            i.text = displayName.isNotEmpty() ? std::move (displayName) : info.shortName;
            i.itemID = (int) commandID;
            i.commandManager = commandManager;
            i.isEnabled = target != nullptr && (info.flags & ApplicationCommandInfo::isDisabled) == 0;
            i.isTicked = (info.flags & ApplicationCommandInfo::isTicked) != 0;
            i.image = std::move (iconToUse);
            addItem (std::move (i));
        }
        */
    }
    
    /**
      | Appends a text item with a special colour.
      | 
      | This is the same as addItem(), but specifies
      | a colour to use for the text, which will
      | override the default colours that are
      | used by the current look-and-feel.
      | See addItem() for a description of the
      | parameters.
      |
      */
    pub fn add_coloured_item_with_drawable(
        &mut self, 
        item_resultid:    i32,
        item_text:        String,
        item_text_colour: Colour,
        is_active:        Option<bool>,
        is_ticked:        Option<bool>,
        icon_to_use:      Box<Drawable>

    ) {

        let is_active: bool = is_active.unwrap_or(true);
        let is_ticked: bool = is_ticked.unwrap_or(false);
        
        todo!();
        /*
            PopupMenuItem i (std::move (itemText));
        i.itemID = itemResultID;
        i.colour = itemTextColour;
        i.isEnabled = isActive;
        i.isTicked = isTicked;
        i.image = std::move (iconToUse);
        addItem (std::move (i));
        */
    }
    
    /**
      | Appends a text item with a special colour.
      | 
      | This is the same as addItem(), but specifies
      | a colour to use for the text, which will
      | override the default colours that are
      | used by the current look-and-feel.
      | See addItem() for a description of the
      | parameters.
      |
      */
    pub fn add_coloured_item(
        &mut self, 
        item_resultid:    i32,
        item_text:        String,
        item_text_colour: Colour,
        is_active:        bool,
        is_ticked:        bool,
        icon_to_use:      &Image

    ) {
        
        todo!();
        /*
            PopupMenuItem i (std::move (itemText));
        i.itemID = itemResultID;
        i.colour = itemTextColour;
        i.isEnabled = isActive;
        i.isTicked = isTicked;
        i.image = createDrawableFromImage (iconToUse);
        addItem (std::move (i));
        */
    }
    
    /**
      | Appends a custom menu item.
      | 
      | This will add a user-defined component
      | to use as a menu item.
      | 
      | -----------
      | @note
      | 
      | native macOS menus do not support custom
      | components.
      | 
      | @see PopupMenuCustomComponent
      |
      */
    pub fn add_custom_item(&mut self, 
        item_resultid: i32,
        cc:            Box<PopupMenuCustomComponent>,
        sub_menu:      Box<PopupMenu<'a>>)  {
        
        todo!();
        /*
            PopupMenuItem i;
        i.itemID = itemResultID;
        i.customComponent = cc.release();
        i.subMenu.reset (createCopyIfNotNull (subMenu.get()));
        addItem (std::move (i));
        */
    }
    
    /**
      | Appends a custom menu item that can't
      | be used to trigger a result.
      | 
      | This will add a user-defined component
      | to use as a menu item.
      | 
      | The caller must ensure that the passed-in
      | component stays alive until after the
      | menu has been hidden.
      | 
      | If triggerMenuItemAutomaticallyWhenClicked
      | is true, the menu itself will handle
      | detection of a mouse-click on your component,
      | and use that to trigger the menu ID specified
      | in itemResultID. If this is false, the
      | menu item can't be triggered, so itemResultID
      | is not used.
      | 
      | -----------
      | @note
      | 
      | native macOS menus do support custom
      | components.
      |
      */
    pub fn add_custom_item_with_ideal_wh(
        &mut self, 
        item_resultid:                                i32,
        custom_component:                             &mut Component,
        ideal_width:                                  i32,
        ideal_height:                                 i32,
        trigger_menu_item_automatically_when_clicked: bool,
        sub_menu:                                     Box<PopupMenu>
    ) {
        
        todo!();
        /*
            auto comp = std::make_unique<HelperClasses::NormalComponentWrapper> (customComponent, idealWidth, idealHeight,
                                                                             triggerMenuItemAutomaticallyWhenClicked);
        addCustomItem (itemResultID, std::move (comp), std::move (subMenu));
        */
    }
    
    /**
      | Appends a sub-menu.
      | 
      | If the menu that's passed in is empty,
      | it will appear as an inactive item.
      | 
      | If the itemResultID argument is non-zero,
      | then the sub-menu item itself can be
      | clicked to trigger it as a command.
      |
      */
    pub fn add_sub_menu(
        &mut self, 
        sub_menu_name: String,
        sub_menu:      PopupMenu,
        is_active:     Option<bool>

    ) {

        let is_active: bool = is_active.unwrap_or(true);
        
        todo!();
        /*
            addSubMenu (std::move (subMenuName), std::move (subMenu), isActive, nullptr, false, 0);
        */
    }
    
    /**
      | Appends a sub-menu with an icon.
      | 
      | If the menu that's passed in is empty,
      | it will appear as an inactive item.
      | 
      | If the itemResultID argument is non-zero,
      | then the sub-menu item itself can be
      | clicked to trigger it as a command.
      |
      */
    pub fn add_sub_menu_with_image(
        &mut self, 
        sub_menu_name: String,
        sub_menu:      PopupMenu,
        is_active:     bool,
        icon_to_use:   &Image,
        is_ticked:     Option<bool>,
        item_resultid: Option<i32>

    ) {

        let is_ticked: bool = is_ticked.unwrap_or(false);
        let item_resultid: i32 = item_resultid.unwrap_or(0);
        
        todo!();
        /*
            addSubMenu (std::move (subMenuName), std::move (subMenu), isActive,
                    createDrawableFromImage (iconToUse), isTicked, itemResultID);
        */
    }
    
    /**
      | Appends a sub-menu with an icon.
      | 
      | If the menu that's passed in is empty,
      | it will appear as an inactive item.
      | 
      | If the itemResultID argument is non-zero,
      | then the sub-menu item itself can be
      | clicked to trigger it as a command.
      | 
      | The iconToUse parameter is a Drawable
      | object to use as the icon to the left of
      | the item. The menu will take ownership
      | of this drawable object and will delete
      | it later when no longer needed
      |
      */
    pub fn add_sub_menu_with_drawable(
        &mut self, 
        sub_menu_name: String,
        sub_menu:      PopupMenu,
        is_active:     bool,
        icon_to_use:   Box<Drawable>,
        is_ticked:     Option<bool>,
        item_resultid: Option<i32>

    ) {

        let is_ticked: bool = is_ticked.unwrap_or(false);
        let item_resultid: i32 = item_resultid.unwrap_or(0);
        
        todo!();
        /*
            PopupMenuItem i (std::move (subMenuName));
        i.itemID = itemResultID;
        i.isEnabled = isActive && (itemResultID != 0 || subMenu.getNumItems() > 0);
        i.subMenu.reset (new PopupMenu (std::move (subMenu)));
        i.isTicked = isTicked;
        i.image = std::move (iconToUse);
        addItem (std::move (i));
        */
    }
    
    /**
      | Appends a separator to the menu, to help
      | break it up into sections.
      | 
      | The menu class is smart enough not to
      | display separators at the top or bottom
      | of the menu, and it will replace multiple
      | adjacent separators with a single one,
      | so your code can be quite free and easy
      | about adding these, and it'll always
      | look ok.
      |
      */
    pub fn add_separator(&mut self)  {
        
        todo!();
        /*
            if (items.size() > 0 && ! items.getLast().isSeparator)
        {
            PopupMenuItem i;
            i.isSeparator = true;
            addItem (std::move (i));
        }
        */
    }
    
    /**
      | Adds a non-clickable text item to the
      | menu.
      | 
      | This is a bold-font items which can be
      | used as a header to separate the items
      | into named groups.
      |
      */
    pub fn add_section_header(&mut self, title: String)  {
        
        todo!();
        /*
            PopupMenuItem i (std::move (title));
        i.itemID = 0;
        i.isSectionHeader = true;
        addItem (std::move (i));
        */
    }
    
    /**
      | Adds a column break to the menu, to help
      | break it up into sections.
      | 
      | Subsequent items will be placed in a
      | new column, rather than being appended
      | to the current column.
      | 
      | If a menu contains explicit column breaks,
      | the menu will never add additional breaks.
      |
      */
    pub fn add_column_break(&mut self)  {
        
        todo!();
        /*
            if (! items.isEmpty())
            std::prev (items.end())->shouldBreakAfter = true;
        */
    }
    
    pub fn create_window(&self, 
        options:                   &PopupMenuOptions,
        manager_of_chosen_command: *mut *mut dyn CommandManagerInterface) -> *mut Component {
        
        todo!();
        /*
            return items.isEmpty() ? nullptr
                               : new HelperClasses::MenuWindow (*this, nullptr, options,
                                                                ! options.getTargetScreenArea().isEmpty(),
                                                                ModifierKeys::currentModifiers.isAnyMouseButtonDown(),
                                                                managerOfChosenCommand);
        */
    }

    pub fn show_with_optional_callback(
        &mut self, 
        options:       &PopupMenuOptions,
        user_callback: *mut dyn ModalComponentManagerCallback,
        can_be_modal:  bool

    ) -> i32 {
        
        todo!();
        /*
            std::unique_ptr<typename ModalComponentManager::Callback> userCallbackDeleter (userCallback);
        std::unique_ptr<PopupMenuCompletionCallback> callback (new PopupMenuCompletionCallback());

        if (auto* window = createWindow (options, &(callback->managerOfChosenCommand)))
        {
            callback->component.reset (window);

            PopupMenuSettings::menuWasHiddenBecauseOfAppChange = false;

            window->setVisible (true); // (must be called before enterModalState on Windows to avoid DropShadower confusion)
            window->enterModalState (false, userCallbackDeleter.release());
            typename ModalComponentManager::getInstance()->attachCallback (window, callback.release());

            window->toFront (false);  // need to do this after making it modal, or it could
                                      // be stuck behind other comps that are already modal..

           #if ALOE_MODAL_LOOPS_PERMITTED
            if (userCallback == nullptr && canBeModal)
                return window->runModalLoop();
           #else
            ignoreUnused (canBeModal);
            jassert (! (userCallback == nullptr && canBeModal));
           #endif
        }

        return 0;
        */
    }

    /**
      | Displays and runs the menu modally,
      | with a set of options.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_menu(&mut self, options: &PopupMenuOptions) -> i32 {
        
        todo!();
        /*
            return showWithOptionalCallback (options, nullptr, true);
        */
    }
    
    /**
      | Runs the menu asynchronously.
      |
      */
    pub fn show_menu_async_with_popup_menu(&mut self, options: &PopupMenuOptions)  {
        
        todo!();
        /*
            showWithOptionalCallback (options, nullptr, false);
        */
    }
    
    /**
      | Runs the menu asynchronously, with
      | a user-provided callback that will
      | receive the result.
      |
      */
    pub fn show_menu_async_with_user_callback(
        &mut self, 
        options:       &PopupMenuOptions,
        user_callback: *mut dyn ModalComponentManagerCallback

    ) {
        
        todo!();
        /*
            #if ! ALOE_MODAL_LOOPS_PERMITTED
        jassert (userCallback != nullptr);
       #endif

        showWithOptionalCallback (options, userCallback, false);
        */
    }
    
    /**
      | Runs the menu asynchronously, with
      | a user-provided callback that will
      | receive the result.
      |
      */
    pub fn show_menu_async_with_basic_callback(
        &mut self, 
        options:       &PopupMenuOptions,
        user_callback: fn(_0: i32) -> ())  {
        
        todo!();
        /*
            showWithOptionalCallback (options, ModalCallbackFunction::create (userCallback), false);
        */
    }

    /**
      | Displays the menu and waits for the user
      | to pick something.
      | 
      | This will display the menu modally,
      | and return the ID of the item that the
      | user picks. If they click somewhere
      | off the menu to get rid of it without choosing
      | anything, this will return 0.
      | 
      | The current location of the mouse will
      | be used as the position to show the menu
      | - to explicitly set the menu's position,
      | use showAt() instead. Depending on
      | where this point is on the screen, the
      | menu will appear above, below or to the
      | side of the point.
      | 
      | -----------
      | @param itemIDThatMustBeVisible
      | 
      | if you set this to the ID of one of the menu
      | items, then when the menu first appears,
      | it will make sure that this item is visible.
      | So if the menu has too many items to fit
      | on the screen, it will be scrolled to
      | a position where this item is visible.
      | ----------
      | @param minimumWidth
      | 
      | a minimum width for the menu, in pixels.
      | It may be wider than this if some items
      | are too long to fit.
      | ----------
      | @param maximumNumColumns
      | 
      | if there are too many items to fit on-screen
      | in a single vertical column, the menu
      | may be laid out as a series of columns
      | - this is the maximum number allowed.
      | To use the default value for this (probably
      | about 7), you can pass in zero.
      | ----------
      | @param standardItemHeight
      | 
      | if this is non-zero, it will be used as
      | the standard height for menu items (apart
      | from custom items)
      | ----------
      | @param callback
      | 
      | if this is not a nullptr, the menu will
      | be launched asynchronously, returning
      | immediately, and the callback will
      | receive a call when the menu is either
      | dismissed or has an item selected. This
      | object will be owned and deleted by the
      | system, so make sure that it works safely
      | and that any pointers that it uses are
      | safely within scope. @see showAt
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show(
        &mut self, 
        item_id_that_must_be_visible: i32,
        minimum_width:                i32,
        maximum_num_columns:          i32,
        standard_item_height:         i32,
        callback:                     *mut ModalComponentManagerCallback

    ) -> i32 {

        let item_id_that_must_be_visible: i32 = item_id_that_must_be_visible.unwrap_or(0);
        let minimum_width: i32 = minimum_width.unwrap_or(0);
        let maximum_num_columns: i32 = maximum_num_columns.unwrap_or(0);
        let standard_item_height: i32 = standard_item_height.unwrap_or(0);
        let callback: *mut ModalComponentManager::Callback = callback.unwrap_or(nullptr);
        
        todo!();
        /*
            return showWithOptionalCallback (PopupMenuOptions().withItemThatMustBeVisible (itemIDThatMustBeVisible)
                                                  .withMinimumWidth (minimumWidth)
                                                  .withMaximumNumColumns (maximumNumColumns)
                                                  .withStandardItemHeight (standardItemHeight),
                                         callback, true);
        */
    }
    
    /**
      | Displays the menu at a specific location.
      | 
      | This is the same as show(), but uses a
      | specific location (in global screen
      | coordinates) rather than the current
      | mouse position.
      | 
      | The screenAreaToAttachTo parameter
      | indicates a screen area to which the
      | menu will be adjacent. Depending on
      | where this is, the menu will decide which
      | edge to attach itself to, in order to
      | fit itself fully on-screen. If you just
      | want to trigger a menu at a specific point,
      | you can pass in a rectangle of size (0,
      | 0) with the position that you want.
      | 
      | @see show()
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_at(
        &mut self, 
        screen_area_to_attach_to:     Rectangle<i32>,
        item_id_that_must_be_visible: i32,
        minimum_width:                i32,
        maximum_num_columns:          i32,
        standard_item_height:         i32,
        callback:                     *mut ModalComponentManagerCallback

    ) -> i32 {

        let item_id_that_must_be_visible: i32 = item_id_that_must_be_visible.unwrap_or(0);
        let minimum_width: i32 = minimum_width.unwrap_or(0);
        let maximum_num_columns: i32 = maximum_num_columns.unwrap_or(0);
        let standard_item_height: i32 = standard_item_height.unwrap_or(0);
        let callback: *mut ModalComponentManager::Callback = callback.unwrap_or(nullptr);
        
        todo!();
        /*
            return showWithOptionalCallback (PopupMenuOptions().withTargetScreenArea (screenAreaToAttachTo)
                                                  .withItemThatMustBeVisible (itemIDThatMustBeVisible)
                                                  .withMinimumWidth (minimumWidth)
                                                  .withMaximumNumColumns (maximumNumColumns)
                                                  .withStandardItemHeight (standardItemHeight),
                                         callback, true);
        */
    }
    
    /**
      | Displays the menu as if it's attached
      | to a component such as a button.
      | 
      | This is similar to showAt(), but will
      | position it next to the given component,
      | e.g. so that the menu's edge is aligned
      | with that of the component. This is intended
      | for things like buttons that trigger
      | a pop-up menu.
      |
      */
    #[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
    pub fn show_at(
        &mut self, 
        component_to_attach_to:       *mut Component,
        item_id_that_must_be_visible: i32,
        minimum_width:                i32,
        maximum_num_columns:          i32,
        standard_item_height:         i32,
        callback:                     *mut ModalComponentManagerCallback

    ) -> i32 {

        let item_id_that_must_be_visible: i32 = item_id_that_must_be_visible.unwrap_or(0);
        let minimum_width: i32 = minimum_width.unwrap_or(0);
        let maximum_num_columns: i32 = maximum_num_columns.unwrap_or(0);
        let standard_item_height: i32 = standard_item_height.unwrap_or(0);
        let callback: *mut ModalComponentManager::Callback = callback.unwrap_or(nullptr);
        
        todo!();
        /*
            auto options = PopupMenuOptions().withItemThatMustBeVisible (itemIDThatMustBeVisible)
                                .withMinimumWidth (minimumWidth)
                                .withMaximumNumColumns (maximumNumColumns)
                                .withStandardItemHeight (standardItemHeight);

        if (componentToAttachTo != nullptr)
            options = options.withTargetComponent (componentToAttachTo);

        return showWithOptionalCallback (options, callback, true);
        */
    }
    
    /**
      | Closes any menus that are currently
      | open.
      | 
      | This might be useful if you have a situation
      | where your window is being closed by
      | some means other than a user action,
      | and you'd like to make sure that menus
      | aren't left hanging around.
      |
      */
    pub fn dismiss_all_active_menus(&mut self) -> bool {
        
        todo!();
        /*
            auto& windows = HelperClasses::MenuWindow::getActiveWindows();
        auto numWindows = windows.size();

        for (int i = numWindows; --i >= 0;)
        {
            if (auto* pmw = windows[i])
            {
                pmw->setLookAndFeel (nullptr);
                pmw->dismissMenu (nullptr);
            }
        }

        return numWindows > 0;
        */
    }
    
    /**
      | Returns the number of items that the
      | menu currently contains. (This doesn't
      | count separators).
      |
      */
    pub fn get_num_items(&self) -> i32 {
        
        todo!();
        /*
            int num = 0;

        for (auto& mi : items)
            if (! mi.isSeparator)
                ++num;

        return num;
        */
    }
    
    /**
      | Returns true if the menu contains a command
      | item that triggers the given command.
      |
      */
    pub fn contains_command_item(&self, commandid: i32) -> bool {
        
        todo!();
        /*
            for (auto& mi : items)
            if ((mi.itemID == commandID && mi.commandManager != nullptr)
                  || (mi.subMenu != nullptr && mi.subMenu->containsCommandItem (commandID)))
                return true;

        return false;
        */
    }
    
    /**
      | Returns true if the menu contains any
      | items that can be used.
      |
      */
    pub fn contains_any_active_items(&self) -> bool {
        
        todo!();
        /*
            for (auto& mi : items)
        {
            if (mi.subMenu != nullptr)
            {
                if (mi.subMenu->containsAnyActiveItems())
                    return true;
            }
            else if (mi.isEnabled)
            {
                return true;
            }
        }

        return false;
        */
    }
    
    /**
      | Specifies a look-and-feel for the menu
      | and any sub-menus that it has.
      | 
      | This can be called before show() if you
      | need a customised menu. Be careful not
      | to delete the LookAndFeel object before
      | the menu has been deleted.
      |
      */
    pub fn set_look_and_feel(&mut self, new_look_and_feel: *mut dyn LookAndFeelPopupMenuInterface)  {
        
        todo!();
        /*
            lookAndFeel = newLookAndFeel;
        */
    }
    
    pub fn set_item(
        &mut self, 
        c:           &mut PopupMenuCustomComponent,
        item_to_use: *const PopupMenuItem

    ) {
        
        todo!();
        /*
            c.item = itemToUse;
        c.repaint();
        */
    }
}
