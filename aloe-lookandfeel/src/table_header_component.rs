crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the TableHeaderComponent.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum TableHeaderComponentColourIds
{
    /**
      | The colour for the text in the header.
      |
      */
    textColourId                   = 0x1003800, 

    /**
      | The colour of the table header background.
      | It's up to the LookAndFeel how this is
      | used.
      |
      */
    backgroundColourId             = 0x1003810, 

    /**
      | The colour of the table header's outline.
      |
      */
    outlineColourId                = 0x1003820, 

    /**
      | The colour of the table header background
      | when the mouse is over or down above the
      | the table header. It's up to the LookAndFeel
      | to use a variant of this colour to distinguish
      | between the down and hover state.
      |
      */
    highlightColourId              = 0x1003830, 
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait TableHeaderComponentLookAndFeelMethods {

    fn draw_table_header_background(
        &mut self, 
        _0: &mut Graphics,
        _1: &mut TableHeaderComponent
    );

    fn draw_table_header_column(
        &mut self, 
        _0:            &mut Graphics,
        _1:            &mut TableHeaderComponent,
        column_name:   &String,
        column_id:     i32,
        width:         i32,
        height:        i32,
        is_mouse_over: bool,
        is_mouse_down: bool,
        column_flags:  i32
    );
}
