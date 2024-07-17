crate::ix!();

/**
  | Command messages that aren't handled
  | in the main component will be passed
  | to this class to respond to.
  |
  */
pub struct MenusDemoOuterCommandTarget<'a> {
    base:                 Component<'a>,
    base2:                ApplicationCommandTarget,
    command_manager:      &'a mut ApplicationCommandManager<'a>,
    inner_command_target: MenusDemoInnerCommandTarget<'a>,
    current_colour:       Colour, // default = { Colours::red  }
}

impl<'a> MenusDemoOuterCommandTarget<'a> {

    pub fn new(m: &mut ApplicationCommandManager) -> Self {
    
        todo!();
        /*


            : commandManager (m),
                  innerCommandTarget (commandManager)

                commandManager.registerAllCommandsForTarget (this);

                addAndMakeVisible (innerCommandTarget);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            innerCommandTarget.setBounds (getLocalBounds().reduced (50));
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (currentColour);
        */
    }
    
    pub fn get_next_command_target(&mut self) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            return &innerCommandTarget;
        */
    }
    
    pub fn get_all_commands(&mut self, c: &mut Vec<CommandID>)  {
        
        todo!();
        /*
            Vec<CommandID> commands { MenusDemoCommandIDs::outerColourRed,
                                            MenusDemoCommandIDs::outerColourGreen,
                                            MenusDemoCommandIDs::outerColourBlue };

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
                    case MenusDemoCommandIDs::outerColourRed:
                        result.setInfo ("Red", "Sets the outer colour to red", "Outer", 0);
                        result.setTicked (currentColour == Colours::red);
                        result.addDefaultKeypress ('r', ModifierKeys::commandModifier);
                        break;
                    case MenusDemoCommandIDs::outerColourGreen:
                        result.setInfo ("Green", "Sets the outer colour to green", "Outer", 0);
                        result.setTicked (currentColour == Colours::green);
                        result.addDefaultKeypress ('g', ModifierKeys::commandModifier);
                        break;
                    case MenusDemoCommandIDs::outerColourBlue:
                        result.setInfo ("Blue", "Sets the outer colour to blue", "Outer", 0);
                        result.setTicked (currentColour == Colours::blue);
                        result.addDefaultKeypress ('b', ModifierKeys::commandModifier);
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
                    case MenusDemoCommandIDs::outerColourRed:
                        currentColour = Colours::red;
                        break;
                    case MenusDemoCommandIDs::outerColourGreen:
                        currentColour = Colours::green;
                        break;
                    case MenusDemoCommandIDs::outerColourBlue:
                        currentColour = Colours::blue;
                        break;
                    default:
                        return false;
                }

                repaint();
                return true;
        */
    }
}
