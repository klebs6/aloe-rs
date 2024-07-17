crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the window.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ResizableWindowColourIds
{
    /**
      | A colour to use to fill the window's background.
      |
      */
    backgroundColourId          = 0x1005700,  
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide window
  | drawing functionality.
  |
  */
pub trait ResizableWindowLookAndFeelMethods
{
    fn draw_corner_resizer(&mut self, 
        _0:                &mut Graphics,
        w:                 i32,
        h:                 i32,
        is_mouse_over:     bool,
        is_mouse_dragging: bool);


    fn draw_resizable_frame(&mut self, 
        _0: &mut Graphics,
        w:  i32,
        h:  i32,
        _3: &BorderSize<i32>);


    fn fill_resizable_window_background(&mut self, 
        _0: &mut Graphics,
        w:  i32,
        h:  i32,
        _3: &BorderSize<i32>,
        _4: &mut ResizableWindow);


    fn draw_resizable_window_border(&mut self, 
        _0:     &mut Graphics,
        w:      i32,
        h:      i32,
        border: &BorderSize<i32>,
        _4:     &mut ResizableWindow);
}
