crate::ix!();

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the combo
  | box.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum PropertyComponentColourIds
{

    /**
      | The background colour to fill the component
      | with.
      |
      */
    backgroundColourId     = 0x1008300,    

    /**
      | The colour for the property's label
      | text.
      |
      */
    labelTextColourId      = 0x1008301,    
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait PropertyComponentLookAndFeelMethods {

    fn draw_property_panel_section_header(&mut self, 
        _0:      &mut Graphics,
        name:    &String,
        is_open: bool,
        width:   i32,
        height:  i32);

    fn draw_property_component_background(&mut self, 
        _0:     &mut Graphics,
        width:  i32,
        height: i32,
        _3:     &mut PropertyComponent);

    fn draw_property_component_label(&mut self, 
        _0:     &mut Graphics,
        width:  i32,
        height: i32,
        _3:     &mut PropertyComponent);

    fn get_property_component_content_position(&mut self, _0: &mut PropertyComponent) -> Rectangle<i32>;

    fn get_property_panel_section_header_height(&mut self, section_title: &String) -> i32;
}

