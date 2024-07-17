crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_BooleanPropertyComponent.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_BooleanPropertyComponent.cpp]

/**
  | A PropertyComponent that contains
  | an on/off toggle button.
  | 
  | This type of property component can
  | be used if you have a boolean value to
  | toggle on/off.
  | 
  | @see PropertyComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct BooleanPropertyComponent<'a> {
    base:     PropertyComponent<'a>,
    button:   ToggleButton<'a>,
    on_text:  String,
    off_text: String,
}

impl<'a> BooleanPropertyComponent<'a> {

    /**
      | Creates a button component.
      | 
      | If you use this constructor, you must
      | override the getState() and setState()
      | methods.
      | 
      | -----------
      | @param propertyName
      | 
      | the property name to be passed to the
      | PropertyComponent
      | ----------
      | @param buttonTextWhenTrue
      | 
      | the text shown in the button when the
      | value is true
      | ----------
      | @param buttonTextWhenFalse
      | 
      | the text shown in the button when the
      | value is false
      |
      */
    pub fn new(
        name:                   &String,
        button_text_when_true:  &String,
        button_text_when_false: &String) -> Self {
    
        todo!();
        /*
        : property_component(name),
        : on_text(buttonTextWhenTrue),
        : off_text(buttonTextWhenFalse),

            addAndMakeVisible (button);
        button.setClickingTogglesState (false);
        button.onClick = [this] { setState (! getState()); };
        */

    }
    
    /**
      | Creates a button component.
      | 
      | -----------
      | @note
      | 
      | if you call this constructor then you
      | must use the Value to interact with the
      | button state, and you can't override
      | the class with your own setState or getState
      | methods.
      | 
      | If you want to use getState and setState,
      | call the other constructor instead.
      | 
      | -----------
      | @param valueToControl
      | 
      | a Value object that this property should
      | refer to.
      | ----------
      | @param propertyName
      | 
      | the property name to be passed to the
      | PropertyComponent
      | ----------
      | @param buttonText
      | 
      | the text shown in the ToggleButton component
      |
      */
    pub fn new_with_value_to_control(
        value_to_control: &Value,
        name:             &String,
        button_text:      &String) -> Self {
    
        todo!();
        /*
        : property_component(name),
        : on_text(buttonText),
        : off_text(buttonText),

            addAndMakeVisible (button);
        button.setClickingTogglesState (false);
        button.setButtonText (buttonText);
        button.getToggleStateValue().referTo (valueToControl);
        button.setClickingTogglesState (true);
        */

    }
    
    pub fn set_state(&mut self, new_state: bool)  {
        
        todo!();
        /*
            button.setToggleState (newState, sendNotification);
        */

    }
    
    pub fn get_state(&self) -> bool {
        
        todo!();
        /*
            return button.getToggleState();
        */

    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            PropertyComponent::paint (g);

        g.setColour (findColour (backgroundColourId));
        g.fillRect (button.getBounds());

        g.setColour (findColour (outlineColourId));
        g.drawRect (button.getBounds());
        */

    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            button.setToggleState (getState(), dontSendNotification);
        button.setButtonText (button.getToggleState() ? onText : offText);
        */

    }
}
