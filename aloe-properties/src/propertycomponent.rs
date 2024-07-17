crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_PropertyComponent.h]

/**
  | A base class for a component that goes
  | in a PropertyPanel and displays one
  | of an item's properties.
  | 
  | Subclasses of this are used to display
  | a property in various forms, e.g. a
  | 
  | ChoicePropertyComponent shows its
  | value as a combo box; a SliderPropertyComponent
  | shows its value as a slider; a TextPropertyComponent
  | as a text box, etc.
  | 
  | A subclass must implement the refresh()
  | method which will be called to tell the
  | component to update itself, and is also
  | responsible for calling this it when
  | the item that it refers to is changed.
  | 
  | @see PropertyPanel, TextPropertyComponent,
  | SliderPropertyComponent,
  | 
  | ChoicePropertyComponent, ButtonPropertyComponent,
  | BooleanPropertyComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct PropertyComponent<'a> {

    base:             Component<'a>,
    base2:            SettableTooltipClient,

    /**
      | Used by the PropertyPanel to determine
      | how high this component needs to be.
      | 
      | A subclass can update this value in its
      | constructor but shouldn't alter it
      | later as changes won't necessarily
      | be picked up.
      |
      */
    preferred_height: i32,
}

pub trait PropertyComponentInterface: Refresh {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/properties/aloe_PropertyComponent.cpp]
impl<'a> PropertyComponent<'a> {

    /**
      | Returns this item's preferred height.
      | 
      | This value is specified either in the
      | constructor or by a subclass changing
      | the preferredHeight member variable.
      |
      */
    pub fn get_preferred_height(&self) -> i32 {
        
        todo!();
        /*
            return preferredHeight;
        */

    }
    
    pub fn set_preferred_height(&mut self, new_height: i32)  {
        
        todo!();
        /*
            preferredHeight = newHeight;
        */

    }

    /**
      | Creates a PropertyComponent.
      | 
      | -----------
      | @param propertyName
      | 
      | the name is stored as this component's
      | name, and is used as the name displayed
      | next to this component in a property
      | panel
      | ----------
      | @param preferredHeight
      | 
      | the height that the component should
      | be given - some items may need to be larger
      | than a normal row height.
      | 
      | This value can also be set if a subclass
      | changes the preferredHeight member
      | variable.
      |
      */
    pub fn new(
        name:   &String,
        height: Option<i32>

    ) -> Self {

        let height: i32 = height.unwrap_or(25);
    
        todo!();
        /*
        : component(name),
        : preferred_height(height),

            jassert (name.isNotEmpty());
        */

    }
    
    /**
      | The default paint method fills the background
      | and draws a label for the item's name.
      | 
      | @see LookAndFeel::drawPropertyComponentBackground(),
      | LookAndFeel::drawPropertyComponentLabel()
      |
      */
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        lf.drawPropertyComponentBackground (g, getWidth(), getHeight(), *this);
        lf.drawPropertyComponentLabel      (g, getWidth(), getHeight(), *this);
        */

    }
    
    /**
      | The default resize method positions
      | any child component to the right of this
      | one, based on the look and feel's default
      | label size.
      |
      */
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (auto c = getChildComponent(0))
            c->setBounds (getLookAndFeel().getPropertyComponentContentPosition (*this));
        */

    }
    
    /**
      | By default, this just repaints the component.
      |
      */
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */

    }
}
