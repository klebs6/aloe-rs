crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/ComponentDemo.h]

/**
  | This class represents one of the individual
  | lights in our grid.
  |
  */
#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct ToggleLightComponent<'a> {

    base: Component<'a>,

    /**
      | member variables for the Component
      |
      */
    is_on: bool, // default = false
}

impl<'a> ToggleLightComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // Only shows the red ellipse when the button is on.
            if (isOn)
            {
                g.setColour (getLookAndFeel().findColour (Slider::thumbColourId));
                g.fillEllipse (getLocalBounds().toFloat());
            }
        */
    }
    
    pub fn mouse_enter(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            // button toggles state on mouse over.
            isOn = ! isOn;
            repaint();
        */
    }
}

