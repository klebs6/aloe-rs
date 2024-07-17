crate::ix!();

/**
  | Command messages that aren't handled
  | in the MenusDemoOuterCommandTarget will be passed
  | to this class to respond to.
  |
  */
pub struct MenusDemoInnerCommandTarget<'a> {
    base:            Component<'a>,
    base2:           ApplicationCommandTarget,
    command_manager: &'a mut ApplicationCommandManager<'a>,
    current_colour:  Colour, // default = { Colours::blue  }
}

impl<'a> MenusDemoInnerCommandTarget<'a> {

    pub fn new(m: &mut ApplicationCommandManager) -> Self {
    
        todo!();
        /*
        : command_manager(m),

            commandManager.registerAllCommandsForTarget (this);
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
            // this will return the next parent component that is an ApplicationCommandTarget
                    return findFirstTargetParentComponent();
        */
    }
    
    pub fn get_all_commands(&mut self, c: &mut Vec<CommandID>)  {
        
        todo!();
        /*
            Vec<CommandID> commands { MenusDemoCommandIDs::innerColourRed,
                                                MenusDemoCommandIDs::innerColourGreen,
                                                MenusDemoCommandIDs::innerColourBlue };

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
                        case MenusDemoCommandIDs::innerColourRed:
                            result.setInfo ("Red", "Sets the inner colour to red", "Inner", 0);
                            result.setTicked (currentColour == Colours::red);
                            result.addDefaultKeypress ('r', ModifierKeys::commandModifier | ModifierKeys::shiftModifier);
                            break;
                        case MenusDemoCommandIDs::innerColourGreen:
                            result.setInfo ("Green", "Sets the inner colour to green", "Inner", 0);
                            result.setTicked (currentColour == Colours::green);
                            result.addDefaultKeypress ('g', ModifierKeys::commandModifier | ModifierKeys::shiftModifier);
                            break;
                        case MenusDemoCommandIDs::innerColourBlue:
                            result.setInfo ("Blue", "Sets the inner colour to blue", "Inner", 0);
                            result.setTicked (currentColour == Colours::blue);
                            result.addDefaultKeypress ('b', ModifierKeys::commandModifier | ModifierKeys::shiftModifier);
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
                        case MenusDemoCommandIDs::innerColourRed:
                            currentColour = Colours::red;
                            break;
                        case MenusDemoCommandIDs::innerColourGreen:
                            currentColour = Colours::green;
                            break;
                        case MenusDemoCommandIDs::innerColourBlue:
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
