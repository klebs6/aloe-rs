crate::ix!();

#[no_copy]
#[leak_detector]
pub struct PopupMenuHeaderItemComponent<'a> {
    base:    PopupMenuCustomComponent<'a>,
    options: &'a PopupMenuOptions<'a>,
}

impl<'a> PopupMenuHeaderItemComponent<'a> {

    pub fn new(
        name: &String,
        opts: &PopupMenuOptions) -> Self {
    
        todo!();
        /*
        : custom_component(false),
        : options(opts),

            setName (name);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawPopupMenuSectionHeaderWithOptions (g,
                                                                        getLocalBounds(),
                                                                        getName(),
                                                                        options);
        */
    }
    
    pub fn get_ideal_size(&mut self, 
        ideal_width:  &mut i32,
        ideal_height: &mut i32)  {
        
        todo!();
        /*
            getLookAndFeel().getIdealPopupMenuItemSizeWithOptions (getName(),
                                                                       false,
                                                                       -1,
                                                                       idealWidth,
                                                                       idealHeight,
                                                                       options);
                idealHeight += idealHeight / 2;
                idealWidth += idealWidth / 4;
        */
    }
}
