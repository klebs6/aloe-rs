
crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the tooltip.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum TooltipWindowColourIds
{
    /**
      | The colour to fill the background with.
      |
      */
    backgroundColourId      = 0x1001b00,    

    /**
      | The colour to use for the text.
      |
      */
    textColourId            = 0x1001c00,    

    /**
      | The colour to use to draw an outline around
      | the tooltip.
      |
      */
    outlineColourId         = 0x1001c10,     
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide window
  | drawing functionality.
  |
  */
pub trait TooltipWindowLookAndFeelMethods
{
    /**
      | returns the bounds for a tooltip at the
      | given screen coordinate, constrained
      | within the given desktop area.
      |
      */
    fn get_tooltip_bounds(&mut self, 
        tip_text:    &String,
        screen_pos:  Point<i32>,
        parent_area: Rectangle<i32>) -> Rectangle<i32>;


    fn draw_tooltip(&mut self, 
        _0:     &mut Graphics,
        text:   &String,
        width:  i32,
        height: i32);
}
