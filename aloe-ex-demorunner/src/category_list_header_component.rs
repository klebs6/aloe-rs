crate::ix!();

pub struct CategoryListHeaderComponent<'a> {
    base:  Button<'a>,
    owner: &'a mut DemoList<'a>,
}

impl<'a> CategoryListHeaderComponent<'a> {

    pub fn new(o: &mut DemoList) -> Self {
    
        todo!();
        /*
            : Button ({}),
                  owner (o)

                setTitle ("Previous");
                setSize (0, 30);
        */
    }
    
    pub fn paint_button(&mut self, 
        g:  &mut Graphics,
        _1: bool,
        _2: bool)  {
        
        todo!();
        /*
            g.setColour (findColour (Label::textColourId));
                g.drawFittedText ("<", getLocalBounds().reduced (20, 0), Justification::centredLeft, 1);
        */
    }
    
    pub fn clicked(&mut self)  {
        
        todo!();
        /*
            owner.showCategory ({});
        */
    }
}
