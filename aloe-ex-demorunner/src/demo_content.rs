crate::ix!();

pub struct DemoContent<'a> {
    base: Component<'a>,
    comp: Box<Component<'a>>,
}

impl<'a> DemoContent<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (comp != nullptr)
                comp->setBounds (getLocalBounds());
        */
    }
    
    pub fn set_component(&mut self, new_component: *mut Component)  {
        
        todo!();
        /*
            comp.reset (newComponent);

            if (comp != nullptr)
            {
                addAndMakeVisible (comp.get());
                resized();
            }
        */
    }
    
    pub fn get_component(&self) -> *mut Component {
        
        todo!();
        /*
            return comp.get();
        */
    }
    
    pub fn show_home_screen(&mut self)  {
        
        todo!();
        /*
            setComponent (createIntroDemo());
        */
    }
}
