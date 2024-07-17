crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_GroupComponent.h]

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
pub enum GroupComponentColourIds
{
    /**
      | The colour to use for drawing the line
      | around the edge.
      |
      */
    outlineColourId     = 0x1005400,    

    /**
      | The colour to use to draw the text label.
      |
      */
    textColourId        = 0x1005410,     
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait GroupComponentLookAndFeelMethods {

    fn draw_group_component_outline(&mut self, 
        _0:   &mut Graphics,
        w:    i32,
        h:    i32,
        text: &String,
        _4:   &Justification,
        _5:   &mut GroupComponent);

}

/**
  | A component that draws an outline around
  | itself and has an optional title at the
  | top, for drawing an outline around a
  | group of controls.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
pub struct GroupComponent<'a> {
    base:          Component<'a>,
    text:          String,
    justification: Justification,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_GroupComponent.cpp]
impl<'a> GroupComponent<'a> {

    /**
      | Returns the current text label position.
      | @see setTextLabelPosition
      |
      */
    pub fn get_text_label_position(&self) -> Justification {
        
        todo!();
        /*
            return justification;
        */
    }
    
    /**
      | Creates a GroupComponent.
      | 
      | -----------
      | @param componentName
      | 
      | the name to give the component
      | ----------
      | @param labelText
      | 
      | the text to show at the top of the outline
      |
      */
    pub fn new(
        name:       &String,
        label_text: &String) -> Self {
    
        todo!();
        /*
        : component(name),
        : text(labelText),
        : justification(Justification::left),

            setInterceptsMouseClicks  (false, true);
        */
    }
    
    /**
      | Changes the text that's shown at the
      | top of the component.
      |
      */
    pub fn set_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            if (text != newText)
        {
            text = newText;
            repaint();
        }
        */
    }
    
    /**
      | Returns the currently displayed text
      | label.
      |
      */
    pub fn get_text(&self) -> String {
        
        todo!();
        /*
            return text;
        */
    }
    
    /**
      | Sets the positioning of the text label.
      | (The default is Justification::left)
      | @see getTextLabelPosition
      |
      */
    pub fn set_text_label_position(&mut self, new_justification: Justification)  {
        
        todo!();
        /*
            if (justification != newJustification)
        {
            justification = newJustification;
            repaint();
        }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawGroupComponentOutline (g, getWidth(), getHeight(),
                                                    text, justification, *this);
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
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
            return std::make_unique<AccessibilityHandler> (*this, AccessibilityRole::group);
        */
    }
}
