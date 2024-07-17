crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/KeyMappingsDemo.h]

/**
  | A list of the command IDs that this demo
  | can perform.
  |
  */
pub enum KeyPressCommandIDs
{
    buttonMoveUp = 1,
    buttonMoveRight,
    buttonMoveDown,
    buttonMoveLeft,
    nextButtonColour,
    previousButtonColour,
    nextBackgroundColour,
    previousBackgroundColour
}

/**
  | This is a simple target for the key-presses
  | which will live inside the demo component
  | and contains a button that can be moved
  | around with the arrow keys.
  |
  */
pub struct KeyPressTarget<'a> {
    base:                    Component<'a>,
    base2:                   ApplicationCommandTarget,
    button:                  TextButton<'a>,
    buttonx:                 i32, // default = -200
    buttony:                 i32, // default = -200
    colours:                 Vec<Colour>,
    button_colour_index:     i32, // default = 0
    background_colour_index: i32, // default = 1
}

impl<'a> Default for KeyPressTarget<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            Vec<Colour> coloursToUse { Colours::darkblue, Colours::darkgrey, Colours::red,
                                         Colours::green, Colours::blue, Colours::hotpink };
            colours.addArray (coloursToUse);

            addAndMakeVisible (button)
        */
    }
}

impl<'a> Resized for KeyPressTarget<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

            // keep the button on-screen
            if (buttonX < -150 || buttonX > bounds.getWidth()
                || buttonY < -30 || buttonY > bounds.getHeight())
            {
                buttonX = bounds.getCentreX() - 75;
                buttonY = bounds.getCentreY() - 15;
            }

            button.setBounds (buttonX, buttonY, 150, 30);
        */
    }
}

impl<'a> Paint for KeyPressTarget<'a> {
    
    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (colours.getUnchecked (backgroundColourIndex));
        */
    }

}

impl<'a> KeyPressTarget<'a> {

    /**
      | No other command targets in this simple
      | example so just return nullptr.
      |
      */
    pub fn get_next_command_target(&mut self) -> *mut ApplicationCommandTarget {
        
        todo!();
        /*
            return nullptr;
        */
    }
    
    pub fn get_all_commands(&mut self, commands: &mut Vec<CommandID>)  {
        
        todo!();
        /*
            Vec<CommandID> ids { KeyPressCommandIDs::buttonMoveUp, KeyPressCommandIDs::buttonMoveRight,
                                   KeyPressCommandIDs::buttonMoveDown, KeyPressCommandIDs::buttonMoveLeft,
                                   KeyPressCommandIDs::nextButtonColour, KeyPressCommandIDs::previousButtonColour,
                                   KeyPressCommandIDs::nextBackgroundColour, KeyPressCommandIDs::previousBackgroundColour };

            commands.addArray (ids);
        */
    }
    
    pub fn get_command_info(&mut self, 
        commandid: CommandID,
        result:    &mut ApplicationCommandInfo)  {
        
        todo!();
        /*
            switch (commandID)
            {
                case KeyPressCommandIDs::buttonMoveUp:
                    result.setInfo ("Move up", "Move the button up", "Button", 0);
                    result.addDefaultKeypress (KeyPress::upKey, 0);
                    break;
                case KeyPressCommandIDs::buttonMoveRight:
                    result.setInfo ("Move right", "Move the button right", "Button", 0);
                    result.addDefaultKeypress (KeyPress::rightKey, 0);
                    break;
                case KeyPressCommandIDs::buttonMoveDown:
                    result.setInfo ("Move down", "Move the button down", "Button", 0);
                    result.addDefaultKeypress (KeyPress::downKey, 0);
                    break;
                case KeyPressCommandIDs::buttonMoveLeft:
                    result.setInfo ("Move left", "Move the button left", "Button", 0);
                    result.addDefaultKeypress (KeyPress::leftKey, 0);
                    break;
                case KeyPressCommandIDs::nextButtonColour:
                    result.setInfo ("Next colour", "Change the colour of the button to the next in the list", "Button", 0);
                    result.addDefaultKeypress (KeyPress::rightKey, ModifierKeys::shiftModifier);
                    break;
                case KeyPressCommandIDs::previousButtonColour:
                    result.setInfo ("Previous colour", "Change the colour of the button to the previous in the list", "Button", 0);
                    result.addDefaultKeypress (KeyPress::leftKey, ModifierKeys::shiftModifier);
                    break;
                case KeyPressCommandIDs::nextBackgroundColour:
                    result.setInfo ("Next colour", "Change the colour of the background to the next in the list", "Other", 0);
                    result.addDefaultKeypress (KeyPress::rightKey, ModifierKeys::commandModifier);
                    break;
                case KeyPressCommandIDs::previousBackgroundColour:
                    result.setInfo ("Previous colour", "Change the colour of the background to the previous in the list", "Other", 0);
                    result.addDefaultKeypress (KeyPress::leftKey, ModifierKeys::commandModifier);
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
                case KeyPressCommandIDs::buttonMoveUp:
                    buttonY -= 5;
                    resized();
                    break;
                case KeyPressCommandIDs::buttonMoveRight:
                    buttonX += 5;
                    resized();
                    break;
                case KeyPressCommandIDs::buttonMoveDown:
                    buttonY += 5;
                    resized();
                    break;
                case KeyPressCommandIDs::buttonMoveLeft:
                    buttonX -= 5;
                    resized();
                    break;
                case KeyPressCommandIDs::nextButtonColour:
                    ++buttonColourIndex %= colours.size();
                    button.setColour (TextButton::buttonColourId, colours.getUnchecked (buttonColourIndex));
                    break;
                case KeyPressCommandIDs::previousButtonColour:
                    --buttonColourIndex;
                    if (buttonColourIndex < 0)
                        buttonColourIndex = colours.size() - 1;
                    button.setColour (TextButton::buttonColourId, colours.getUnchecked (buttonColourIndex));
                    break;
                case KeyPressCommandIDs::nextBackgroundColour:
                    ++backgroundColourIndex %= colours.size();
                    repaint();
                    break;
                case KeyPressCommandIDs::previousBackgroundColour:
                    --backgroundColourIndex;
                    if (backgroundColourIndex < 0)
                        backgroundColourIndex = colours.size() - 1;
                    repaint();
                    break;
                default:
                    return false;
            }

            return true;
        */
    }
}
