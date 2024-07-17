crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_ButtonPropertyComponent.h]

/**
  | A PropertyComponent that contains
  | a button.
  | 
  | This type of property component can
  | be used if you need a button to trigger
  | some kind of action.
  | 
  | @see PropertyComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ButtonPropertyComponent<'a> {
    base:   PropertyComponent<'a>,
    button: TextButton<'a>,
}

pub trait ButtonPropertyComponentInterface: GetButtonText + ButtonClicked { }

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_ButtonPropertyComponent.cpp]
impl<'a> ButtonPropertyComponent<'a> {
    
    /**
      | Creates a button component.
      | 
      | -----------
      | @param propertyName
      | 
      | the property name to be passed to the
      | PropertyComponent
      | ----------
      | @param triggerOnMouseDown
      | 
      | this is passed to the Button::setTriggeredOnMouseDown()
      | method
      |
      */
    pub fn new(
        name:                  &String,
        trigger_on_mouse_down: bool) -> Self {
    
        todo!();
        /*
        : property_component(name),

            addAndMakeVisible (button);
        button.setTriggeredOnMouseDown (triggerOnMouseDown);
        button.onClick = [this] { buttonClicked(); };
        */

    }
    
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            button.setButtonText (getButtonText());
        */

    }
}
