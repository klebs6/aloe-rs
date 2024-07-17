crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the button.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ToggleButtonColourIds
{
    /**
      | The colour to use for the button's text.
      |
      */
    textColourId            = 0x1006501,  

    /**
      | The colour to use for the tick mark.
      |
      */
    tickColourId            = 0x1006502,  

    /**
      | The colour to use for the disabled tick
      | mark and/or outline.
      |
      */
    tickDisabledColourId    = 0x1006503,   
}

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the button.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum TextButtonColourIds
{
    /**
      | The colour used to fill the button shape
      | (when the button is toggled 'off').
      | The look-and-feel class might re-interpret
      | this to add effects, etc.
      |
      */
    buttonColourId                  = 0x1000100,  

    /**
      | The colour used to fill the button shape
      | (when the button is toggled 'on'). The
      | look-and-feel class might re-interpret
      | this to add effects, etc.
      |
      */
    buttonOnColourId                = 0x1000101,  

    /**
      | The colour to use for the button's text
      | when the button's toggle state is "off".
      |
      */
    textColourOffId                 = 0x1000102,  
    /**
      | The colour to use for the button's text.when
      | the button's toggle state is "on".
      |
      */
    textColourOnId                  = 0x1000103,   
}

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the link.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum HyperlinkButtonColourIds
{
    /**
      | The colour to use for the Url text.
      |
      */
    textColourId             = 0x1001f00, 
}

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the link.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | -----------
  | @note
  | 
  | when the ImageOnButtonBackground
  | style is used, the colour IDs that control
  | the button colour are TextButton::buttonColourId
  | and TextButton::buttonOnColourId.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum DrwableButtonColourIds
{
    /**
      | The colour to use for the button's text
      | label.
      |
      */
    textColourId             = 0x1004010,  

    /**
      | The colour to use for the button's text
      | when the button's toggle state is "on".
      |
      */
    textColourOnId           = 0x1004013,  

    /**
      | The colour used to fill the button's
      | background (when the button is toggled
      | 'off'). Note that if you use the ImageOnButtonBackground
      | style, you should use TextButton::buttonColourId
      | to change the button's colour.
      |
      */
    backgroundColourId       = 0x1004011,  

    /**
      | The colour used to fill the button's
      | background (when the button is toggled
      | 'on'). Note that if you use the ImageOnButtonBackground
      | style, you should use TextButton::buttonOnColourId
      | to change the button's colour.
      |
      */
    backgroundOnColourId     = 0x1004012,  
}
