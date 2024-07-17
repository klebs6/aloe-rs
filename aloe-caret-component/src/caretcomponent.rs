crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_CaretComponent.h]

pub trait SetCaretPosition {

    /**
      | Sets the caret's position to place it
      | next to the given character.
      | 
      | The area is the rectangle containing
      | the entire character that the caret
      | is positioned on, so by default a vertical-line
      | caret may choose to just show itself
      | at the left of this area. You can override
      | this method to customise its size.
      | 
      | This method will also force the caret
      | to reset its timer and become visible
      | (if appropriate), so that as it moves,
      | you can see where it is.
      |
      */
    fn set_caret_position(&mut self, character_area: &Rectangle<i32>);
}

#[no_copy]
pub struct CaretComponent<'a> {
    base:  Component<'a>,
    base2: Timer,
    owner: *mut Component<'a>,
}

/**
  | A set of colour IDs to use to change the
  | colour of various aspects of the caret.
  | 
  | These constants can be used either via
  | the Component::setColour(), or LookAndFeel::setColour()
  | methods. @see Component::setColour,
  | Component::findColour, LookAndFeel::setColour,
  | LookAndFeel::findColour
  |
  */
pub enum CaretComponentColourIds
{
    /**
      | The colour with which to draw the caret.
      |
      */
    caretColourId    = 0x1000204, 
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_CaretComponent.cpp]
impl<'a> CaretComponent<'a> {

    /**
      | Creates the caret component.
      | 
      | The keyFocusOwner is an optional component
      | which the caret will check, making itself
      | visible only when the keyFocusOwner
      | has keyboard focus.
      |
      */
    pub fn new(key_focus_owner: *mut Component<'a>) -> Self {
    
        todo!();
        /*
        : owner(keyFocusOwner),

            setPaintingIsUnclipped (true);
        setInterceptsMouseClicks (false, false);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (findColour (caretColourId, true));
        g.fillRect (getLocalBounds());
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            setVisible (shouldBeShown() && ! isVisible());
        */
    }
    
    pub fn set_caret_position(&mut self, character_area: &Rectangle<i32>)  {
        
        todo!();
        /*
            startTimer (380);
        setVisible (shouldBeShown());
        setBounds (characterArea.withWidth (2));
        */
    }
    
    pub fn should_be_shown(&self) -> bool {
        
        todo!();
        /*
            return owner == nullptr || (owner->hasKeyboardFocus (false)
                                     && ! owner->isCurrentlyBlockedByAnotherModalComponent());
        */
    }
}
