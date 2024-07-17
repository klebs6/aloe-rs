crate::ix!();

#[no_copy]
pub struct SwatchComponent<'a> {
    base:  Component<'a>,
    owner: &'a mut ColourSelector<'a>,
    index: i32,
}

impl<'a> SwatchComponent<'a> {

    pub fn new(
        cs:         &mut ColourSelector<'a>,
        item_index: i32) -> Self {
    
        todo!();
        /*
        : owner(cs),
        : index(itemIndex),
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto col = owner.getSwatchColour (index);

            g.fillCheckerBoard (getLocalBounds().toFloat(), 6.0f, 6.0f,
                                Colour (0xffdddddd).overlaidWith (col),
                                Colour (0xffffffff).overlaidWith (col));
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            PopupMenu m;
            m.addItem (1, TRANS("Use this swatch as the current colour"));
            m.addSeparator();
            m.addItem (2, TRANS("Set this swatch to the current colour"));

            m.showMenuAsync (typename PopupMenu::Options().withTargetComponent (this),
                             ModalCallbackFunction::forComponent (menuStaticCallback, this));
        */
    }
    
    pub fn menu_static_callback(
        result: i32,
        comp:   *mut SwatchComponent)  {
        
        todo!();
        /*
            if (comp != nullptr)
            {
                if (result == 1)  comp->setColourFromSwatch();
                if (result == 2)  comp->setSwatchFromColour();
            }
        */
    }
    
    pub fn set_colour_from_swatch(&mut self)  {
        
        todo!();
        /*
            owner.setCurrentColour (owner.getSwatchColour (index));
        */
    }
    
    pub fn set_swatch_from_colour(&mut self)  {
        
        todo!();
        /*
            if (owner.getSwatchColour (index) != owner.getCurrentColour())
            {
                owner.setSwatchColour (index, owner.getCurrentColour());
                repaint();
            }
        */
    }
}
