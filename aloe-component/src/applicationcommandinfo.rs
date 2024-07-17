crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_ApplicationCommandInfo.h]

/**
  | Holds information describing an application
  | command.
  |
  | This object is used to pass information about
  | a particular command, such as its name,
  | description and other usage flags.
  |
  | When an ApplicationCommandTarget is asked to
  | provide information about the commands it can
  | perform, this is the structure gets filled-in
  | to describe each one.
  |
  | @see ApplicationCommandTarget,
  |      ApplicationCommandTarget::getCommandInfo(),
  |      ApplicationCommandManager
  |
  | @tags{GUI}
  */
pub struct ApplicationCommandInfo {

    /**
      | The command's unique ID number.
      |
      */
    commandid: CommandID,

    /** 
      | A short name to describe the command.
      |
      | This should be suitable for use in menus,
      | on buttons that trigger the command, etc.
      |
      | You can use the setInfo() method to
      | quickly set this and some of the command's
      | other properties.
      */
    short_name: String,

    /** 
      | A longer description of the command.
      |
      | This should be suitable for use in contexts
      | such as a KeyMappingEditorComponent or
      | pop-up tooltip describing what the command
      | does.
      |
      | You can use the setInfo() method to quickly
      | set this and some of the command's other
      | properties.
      */
    description: String,

    /** 
      | A named category that the command fits into.
      |
      | You can give your commands any category you
      | like, and these will be displayed in
      | contexts such as the
      | KeyMappingEditorComponent, where the
      | category is used to group commands together.
      |
      | You can use the setInfo() method to quickly
      | set this and some of the command's other
      | properties.
      */
    category_name: String,

    /** 
      | A list of zero or more keypresses that should
      | be used as the default keys for this command.
      |
      | Methods such as
      | KeyPressMappingSet::resetToDefaultMappings()
      | will use the keypresses in this list to
      | initialise the default set of
      | key-to-command mappings.
      |
      | @see addDefaultKeypress
      */
    default_keypresses: Vec<KeyPress>,

    /** 
      | A bitwise-OR of the values specified in the
      | ApplicationCommandInfoCommandFlags enum.
      |
      | You can use the setInfo() method to quickly
      | set this and some of the command's other
      | properties.
      */
    flags: i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/commands/aloe_ApplicationCommandInfo.cpp]
impl ApplicationCommandInfo {

    pub fn new(cid: CommandID) -> Self {
    
        todo!();
        /*

            : commandID (cid), flags (0)
        */
    }
    
    /** 
      | Sets a number of the structures values at
      | once.
      |
      | The meanings of each of the parameters is
      | described below, in the appropriate member
      | variable's description.
      */
    pub fn set_info(&mut self, 
        short_name:    &String,
        description:   &String,
        category_name: &String,
        flags:         i32)  {
        
        todo!();
        /*
            shortName = shortName_;
        description = description_;
        categoryName = categoryName_;
        flags = flags_;
        */
    }
    
    /** 
      | An easy way to set or remove the isDisabled
      | bit in the structure's flags field.
      |
      | If isActive is true, the flags member has
      | the isDisabled bit cleared; if isActive is
      | false, the bit is set.
      */
    pub fn set_active(&mut self, b: bool)  {
        
        todo!();
        /*
            if (b)
            flags &= ~isDisabled;
        else
            flags |= isDisabled;
        */
    }
    
    /**
      | An easy way to set or remove the isTicked
      | bit in the structure's flags field.
      |
      */
    pub fn set_ticked(&mut self, b: bool)  {
        
        todo!();
        /*
            if (b)
            flags |= isTicked;
        else
            flags &= ~isTicked;
        */
    }
    
    /** 
      | Handy method for adding a keypress to the
      | defaultKeypresses array.
      |
      | This is just so you can write things like:
      | @code
      | myinfo.addDefaultKeypress ('s', ModifierKeys::commandModifier);
      | @endcode
      | instead of
      | @code
      | myinfo.defaultKeypresses.add (KeyPress ('s', ModifierKeys::commandModifier));
      | @endcode
      */
    pub fn add_default_keypress(&mut self, 
        key_code:  i32,
        modifiers: ModifierKeys)  {
        
        todo!();
        /*
            defaultKeypresses.add (KeyPress (keyCode, modifiers, 0));
        */
    }
}
