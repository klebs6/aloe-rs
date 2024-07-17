crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide slider
  | drawing functionality.
  |
  */
pub trait SliderLookAndFeelMethods
: DrawLinearSlider 
+ DrawLinearSliderBackground 
+ DrawLinearSliderThumb 
+ GetSliderThumbRadius 
+ DrawRotarySlider 
+ CreateSliderButton 
+ CreateSliderTextBox 
+ GetSliderEffect 
+ GetSliderPopupFont 
+ GetSliderPopupPlacement 
+ GetSliderLayout 
{ }

pub trait DrawLinearSlider {

    fn draw_linear_slider(
        &mut self, 
        _0:             &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        _8:             SliderStyle,
        _9:             &mut Slider
    );
}

pub trait DrawLinearSliderBackground {

    fn draw_linear_slider_background(
        &mut self, 
        _0:             &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        style:          SliderStyle,
        _9:             &mut Slider
    );
}

pub trait DrawLinearSliderThumb {

    fn draw_linear_slider_thumb(
        &mut self, 
        _0:             &mut Graphics,
        x:              i32,
        y:              i32,
        width:          i32,
        height:         i32,
        slider_pos:     f32,
        min_slider_pos: f32,
        max_slider_pos: f32,
        _8:             SliderStyle,
        _9:             &mut Slider
    );
}

pub trait GetSliderThumbRadius {

    fn get_slider_thumb_radius(&mut self, _0: &mut Slider) -> i32;
}

pub trait DrawRotarySlider {

    fn draw_rotary_slider(
        &mut self, 
        _0:                      &mut Graphics,
        x:                       i32,
        y:                       i32,
        width:                   i32,
        height:                  i32,
        slider_pos_proportional: f32,
        rotary_start_angle:      f32,
        rotary_end_angle:        f32,
        _8:                      &mut Slider
    );
}

pub trait CreateSliderButton {

    fn create_slider_button(
        &mut self, 
        _0:           &mut Slider,
        is_increment: bool
    ) -> *mut Button;
}

pub trait CreateSliderTextBox {

    fn create_slider_text_box(&mut self, _0: &mut Slider) -> *mut Label;
}

pub trait GetSliderEffect {

    fn get_slider_effect(&mut self, _0: &mut Slider) -> *mut dyn ImageEffectFilter;
}

pub trait GetSliderPopupFont {

    fn get_slider_popup_font(&mut self, _0: &mut Slider) -> Font;
}

pub trait GetSliderPopupPlacement {

    fn get_slider_popup_placement(&mut self, _0: &mut Slider) -> i32;
}

pub trait GetSliderLayout {

    fn get_slider_layout(&mut self, _0: &mut Slider) -> SliderLayout;
}

//-------------------------------
/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the slider.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods.
  | 
  | @see Component::setColour, Component::findColour,
  | LookAndFeel::setColour, LookAndFeel::findColour
  |
  */
pub enum SliderColourIds
{
    /**
      | A colour to use to fill the slider's background.
      |
      */
    backgroundColourId          = 0x1001200,  

    /**
      | The colour to draw the thumb with. It's
      | up to the look and feel class how this
      | is used.
      |
      */
    thumbColourId               = 0x1001300,  

    /**
      | The colour to draw the groove that the
      | thumb moves along.
      |
      */
    trackColourId               = 0x1001310,  

    /**
      | For rotary sliders, this colour fills
      | the outer curve.
      |
      */
    rotarySliderFillColourId    = 0x1001311,  

    /**
      | For rotary sliders, this colour is used
      | to draw the outer curve's outline.
      |
      */
    rotarySliderOutlineColourId = 0x1001312,  

    /**
      | The colour for the text in the text-editor
      | box used for editing the value.
      |
      */
    textBoxTextColourId         = 0x1001400,  

    /**
      | The background colour for the text-editor
      | box.
      |
      */
    textBoxBackgroundColourId   = 0x1001500,  

    /**
      | The text highlight colour for the text-editor
      | box.
      |
      */
    textBoxHighlightColourId    = 0x1001600,  

    /**
      | The colour to use for a border around
      | the text-editor box.
      |
      */
    textBoxOutlineColourId      = 0x1001700,   
}
