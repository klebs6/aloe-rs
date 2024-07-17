crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the bar.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ProgressBarColourIds
{
    /**
      | The background colour, behind the bar.
      |
      */
    backgroundColourId              = 0x1001900,    

    /**
      | The colour to use to draw the bar itself.
      | LookAndFeel classes will probably
      | use variations on this colour.
      |
      */
    foregroundColourId              = 0x1001a00,    
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait ProgressBarLookAndFeelMethods {

    /**
      | Draws a progress bar.
      | 
      | If the progress value is less than 0 or
      | greater than 1.0, this should draw a
      | spinning bar that fills the whole space
      | (i.e. to say that the app is still busy
      | but the progress isn't known). It can
      | use the current time as a basis for playing
      | an animation.
      | 
      | (Used by progress bars in AlertWindow).
      |
      */

    fn draw_progress_bar(&mut self, 
        _0:           &mut Graphics,
        _1:           &mut ProgressBar,
        width:        i32,
        height:       i32,
        progress:     f64,
        text_to_show: &String);

    fn is_progress_bar_opaque(&mut self, _0: &mut ProgressBar) -> bool;
}


