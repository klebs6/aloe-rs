crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ToggleButton.h]

/**
  | A button that can be toggled on/off.
  | 
  | All buttons can be toggle buttons, but
  | this lets you create one of the standard
  | ones which has a tick-box and a text label
  | next to it.
  | 
  | @see Button, DrawableButton, TextButton
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ToggleButton<'a> {
    base: Button<'a>,
}

impl<'a> Default for ToggleButton<'a> {
    
    /**
      | Creates a ToggleButton.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : button(String()),

            setClickingTogglesState (true);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ToggleButton.cpp]
impl<'a> ToggleButton<'a> {
    
    /**
      | Creates a ToggleButton.
      | 
      | -----------
      | @param buttonText
      | 
      | the text to put in the button (the component's
      | name is also initially set to this string,
      | but these can be changed later using
      | the setName() and setButtonText()
      | methods)
      |
      */
    pub fn new(button_text: &String) -> Self {
    
        todo!();
        /*
        : button(buttonText),

            setClickingTogglesState (true);
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            getLookAndFeel().drawToggleButton (g, *this, shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown);
        */
    }
    
    /**
      | Resizes the button to fit neatly around
      | its current text. The button's height
      | won't be affected, only its width.
      |
      */
    pub fn change_width_to_fit_text(&mut self)  {
        
        todo!();
        /*
            getLookAndFeel().changeToggleButtonWidthToFitText (*this);
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<ButtonAccessibilityHandler> (*this, AccessibilityRole::toggleButton);
        */
    }
}
