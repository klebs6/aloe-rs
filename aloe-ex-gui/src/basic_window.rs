crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/WindowsDemo.h]

/**
  | Just a simple window that deletes itself
  | when closed.
  |
  */
#[no_copy]
#[leak_detector]
pub struct BasicWindow<'a> {
    base: DocumentWindow<'a>,
}

impl<'a> BasicWindow<'a> {
    
    pub fn new(
        name:              &String,
        background_colour: Colour,
        buttons_needed:    i32) -> Self {
    
        todo!();
        /*
        : document_window(name, backgroundColour, buttonsNeeded),

        
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            delete this;
        */
    }
}


