crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/PropertiesDemo.h]

#[no_copy]
#[leak_detector]
pub struct DemoButtonPropertyComponent<'a> {
    base:    ButtonPropertyComponent<'a>,
    counter: i32, // default = 0
}

impl<'a> DemoButtonPropertyComponent<'a> {

    pub fn new(property_name: &String) -> Self {
    
        todo!();
        /*


            : ButtonPropertyComponent (propertyName, true)
            refresh();
        */
    }
    
    pub fn button_clicked(&mut self)  {
        
        todo!();
        /*
            ++counter;
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::InfoIcon, "Action Button Pressed",
                                              "Pressing this type of property component can trigger an action such as showing an alert window!");
            refresh();
        */
    }
    
    pub fn get_button_text(&self) -> String {
        
        todo!();
        /*
            return "Button clicked " + String (counter) + " times";
        */
    }
}
