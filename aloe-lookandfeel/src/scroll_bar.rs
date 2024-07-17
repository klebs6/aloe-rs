crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the component.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum ScrollBarColourIds
{
    /**
      | The background colour of the scrollbar.
      |
      */
    backgroundColourId          = 0x1000300,    

    /**
      | A base colour to use for the thumb. The
      | look and feel will probably use variations
      | on this colour.
      |
      */
    thumbColourId               = 0x1000400,    

    /**
      | < A base colour to use for the slot area
      | of the bar. The look and feel will probably
      | use variations on this colour.
      |
      */
    trackColourId,               
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide scrollbar-drawing
  | functionality.
  |
  */
pub trait ScrollBarLookAndFeelMethods {

    fn are_scrollbar_buttons_visible(&mut self) -> bool;

    /**
      | Draws one of the buttons on a scrollbar.
      | 
      | -----------
      | @param g
      | 
      | the context to draw into
      | ----------
      | @param scrollbar
      | 
      | the bar itself
      | ----------
      | @param width
      | 
      | the width of the button
      | ----------
      | @param height
      | 
      | the height of the button
      | ----------
      | @param buttonDirection
      | 
      | the direction of the button, where 0
      | = up, 1 = right, 2 = down, 3 = left
      | ----------
      | @param isScrollbarVertical
      | 
      | true if it's a vertical bar, false if
      | horizontal
      | ----------
      | @param isMouseOverButton
      | 
      | whether the mouse is currently over
      | the button (also true if it's held down)
      | ----------
      | @param isButtonDown
      | 
      | whether the mouse button's held down
      |
      */
    fn draw_scrollbar_button(&mut self, 
        g:                     &mut Graphics,
        scrollbar:             &mut ScrollBar,
        width:                 i32,
        height:                i32,
        button_direction:      i32,
        is_scrollbar_vertical: bool,
        is_mouse_over_button:  bool,
        is_button_down:        bool);

    /**
      | Draws the thumb area of a scrollbar.
      | 
      | -----------
      | @param g
      | 
      | the context to draw into
      | ----------
      | @param scrollbar
      | 
      | the bar itself
      | ----------
      | @param x
      | 
      | the x position of the left edge of the
      | thumb area to draw in
      | ----------
      | @param y
      | 
      | the y position of the top edge of the thumb
      | area to draw in
      | ----------
      | @param width
      | 
      | the width of the thumb area to draw in
      | ----------
      | @param height
      | 
      | the height of the thumb area to draw in
      | ----------
      | @param isScrollbarVertical
      | 
      | true if it's a vertical bar, false if
      | horizontal
      | ----------
      | @param thumbStartPosition
      | 
      | for vertical bars, the y coordinate
      | of the top of the thumb, or its x position
      | for horizontal bars
      | ----------
      | @param thumbSize
      | 
      | for vertical bars, the height of the
      | thumb, or its width for horizontal bars.
      | This may be 0 if the thumb shouldn't be
      | drawn.
      | ----------
      | @param isMouseOver
      | 
      | whether the mouse is over the thumb area,
      | also true if the mouse is currently dragging
      | the thumb
      | ----------
      | @param isMouseDown
      | 
      | whether the mouse is currently dragging
      | the scrollbar
      |
      */
    fn draw_scrollbar(&mut self, 
        g:                     &mut Graphics,
        scrollbar:             &mut ScrollBar,
        x:                     i32,
        y:                     i32,
        width:                 i32,
        height:                i32,
        is_scrollbar_vertical: bool,
        thumb_start_position:  i32,
        thumb_size:            i32,
        is_mouse_over:         bool,
        is_mouse_down:         bool);

    /**
      | Returns the component effect to use
      | for a scrollbar
      |
      */
    fn get_scrollbar_effect(&mut self) -> *mut dyn ImageEffectFilter;

    /**
      | Returns the minimum length in pixels
      | to use for a scrollbar thumb.
      |
      */
    fn get_minimum_scrollbar_thumb_size(&mut self, _0: &mut ScrollBar) -> i32;

    /**
      | Returns the default thickness to use
      | for a scrollbar.
      |
      */
    fn get_default_scrollbar_width(&mut self) -> i32;

    /**
      | Returns the length in pixels to use for
      | a scrollbar button.
      |
      */

    fn get_scrollbar_button_size(&mut self, _0: &mut ScrollBar) -> i32;
}
