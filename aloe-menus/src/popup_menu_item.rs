crate::ix!();

/**
  | Describes a popup menu item.
  |
  */
pub struct PopupMenuItem<'a> {

    /**
      | The menu item's name.
      |
      */
    text:                     String,

    /**
      | The menu item's ID. This must not be 0 if you want
      | the item to be triggerable, but if you're attaching
      | an action callback to the item, you can set the
      | itemID to -1 to indicate that it isn't actively
      | needed.
      */
    itemid:                   i32, // default = 0

    /**
      | An optional function which should be
      | invoked when this menu item is triggered.
      |
      */
    action:                   fn() -> (),

    /**
      | A sub-menu, or nullptr if there isn't
      | one.
      |
      */
    sub_menu:                 Box<PopupMenu<'a>>,

    /**
      | A drawable to use as an icon, or nullptr
      | if there isn't one.
      |
      */
    image:                    Box<Drawable<'a>>,

    /**
      | A custom component for the item to display,
      | or nullptr if there isn't one.
      |
      */
    custom_component:         ReferenceCountedObjectPtr<PopupMenuCustomComponent<'a>>,

    /**
      | A custom callback for the item to use,
      | or nullptr if there isn't one.
      |
      */
    custom_callback:          ReferenceCountedObjectPtr<PopupMenuCustomCallback<'a>>,

    /**
      | A command manager to use to automatically
      | invoke the command, or nullptr if none
      | is specified.
      |
      */
    command_manager:          *mut dyn CommandManagerInterface, // default = nullptr

    /**
      | An optional string describing the shortcut key
      | for this item. This is only used for displaying
      | at the right-hand edge of a menu item - the menu
      | won't attempt to actually catch or process the
      | key. If you supply a commandManager parameter then
      | the menu will attempt to fill-in this field automatically.
      */
    shortcut_key_description: String,

    /**
      | A colour to use to draw the menu text. By default
      | this is transparent black, which means that the
      | LookAndFeel should choose the colour.
      */
    colour:                   Colour,

    /**
      | True if this menu item is enabled.
      |
      */
    is_enabled:               bool, // default = true

    /**
      | True if this menu item should have a tick
      | mark next to it.
      |
      */
    is_ticked:                bool, // default = false

    /**
      | True if this menu item is a separator
      | line.
      |
      */
    is_separator:             bool, // default = false

    /**
      | True if this menu item is a section header.
      |
      */
    is_section_header:        bool, // default = false

    /**
      | True if this is the final item in the current
      | column.
      |
      */
    should_break_after:       bool, // default = false
}

impl<'a> Default for PopupMenuItem<'a> {
    
    /**
      | Creates a null item.
      | 
      | You'll need to set some fields after
      | creating an PopupMenuItem before you can add it
      | to a PopupMenu
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl<'a> PopupMenuItem<'a> {

    /**
      | Creates an item with the given text.
      | 
      | This constructor also initialises
      | the itemID to -1, which makes it suitable
      | for creating lambda-based item actions.
      |
      */
    pub fn new_from_text(t: String) -> Self {
    
        todo!();
        /*
        : text(std::move (t)),
        : itemid(-1),

        
        */
    }
    
    pub fn new(other: &PopupMenuItem) -> Self {
    
        todo!();
        /*


            : text (other.text),
        itemID (other.itemID),
        action (other.action),
        subMenu (createCopyIfNotNull (other.subMenu.get())),
        image (other.image != nullptr ? other.image->createCopy() : nullptr),
        customComponent (other.customComponent),
        customCallback (other.customCallback),
        commandManager (other.commandManager),
        shortcutKeyDescription (other.shortcutKeyDescription),
        colour (other.colour),
        isEnabled (other.isEnabled),
        isTicked (other.isTicked),
        isSeparator (other.isSeparator),
        isSectionHeader (other.isSectionHeader),
        shouldBreakAfter (other.shouldBreakAfter)
        */
    }
    
    pub fn assign_from(&mut self, other: &PopupMenuItem) -> &mut PopupMenuItem {
        
        todo!();
        /*
            text = other.text;
        itemID = other.itemID;
        action = other.action;
        subMenu.reset (createCopyIfNotNull (other.subMenu.get()));
        image = other.image != nullptr ? other.image->createCopy() : std::unique_ptr<Drawable>();
        customComponent = other.customComponent;
        customCallback = other.customCallback;
        commandManager = other.commandManager;
        shortcutKeyDescription = other.shortcutKeyDescription;
        colour = other.colour;
        isEnabled = other.isEnabled;
        isTicked = other.isTicked;
        isSeparator = other.isSeparator;
        isSectionHeader = other.isSectionHeader;
        shouldBreakAfter = other.shouldBreakAfter;
        return *this;
        */
    }
    
    /**
      | Sets the isTicked flag (and returns
      | a reference to this item to allow chaining).
      |
      */
    pub fn set_ticked(&mut self, should_be_ticked: Option<bool>) -> &mut PopupMenuItem {

        let should_be_ticked: bool = should_be_ticked.unwrap_or(true);
        
        todo!();
        /*
            isTicked = shouldBeTicked;
        return *this;
        */
    }
    
    /**
      | Sets the isEnabled flag (and returns
      | a reference to this item to allow chaining).
      |
      */
    pub fn set_enabled(&mut self, should_be_enabled: bool) -> &mut PopupMenuItem {
        
        todo!();
        /*
            isEnabled = shouldBeEnabled;
        return *this;
        */
    }
    
    /**
      | Sets the action property (and returns
      | a reference to this item to allow chaining).
      |
      */
    pub fn set_action(&mut self, new_action: fn() -> ()) -> &mut PopupMenuItem {
        
        todo!();
        /*
            action = std::move (newAction);
        return *this;
        */
    }
    
    /**
      | Sets the itemID property (and returns
      | a reference to this item to allow chaining).
      |
      */
    pub fn setid(&mut self, newid: i32) -> &mut PopupMenuItem {
        
        todo!();
        /*
            itemID = newID;
        return *this;
        */
    }
    
    /**
      | Sets the colour property (and returns
      | a reference to this item to allow chaining).
      |
      */
    pub fn set_colour(&mut self, new_colour: Colour) -> &mut PopupMenuItem {
        
        todo!();
        /*
            colour = newColour;
        return *this;
        */
    }
    
    /**
      | Sets the customComponent property
      | (and returns a reference to this item
      | to allow chaining).
      |
      */
    pub fn set_custom_component(&mut self, comp: ReferenceCountedObjectPtr<PopupMenuCustomComponent>) -> &mut PopupMenuItem {
        
        todo!();
        /*
            customComponent = comp;
        return *this;
        */
    }
    
    /**
      | Sets the image property (and returns
      | a reference to this item to allow chaining).
      |
      */
    pub fn set_image(&mut self, new_image: Box<Drawable>) -> &mut PopupMenuItem {
        
        todo!();
        /*
            image = std::move (newImage);
        return *this;
        */
    }
}
