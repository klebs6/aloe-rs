crate::ix!();

#[no_copy]
pub struct TableHeaderComponentDragOverlayComp<'a> {
    base:  Component<'a>,
    image: Image,
}

impl<'a> TableHeaderComponentDragOverlayComp<'a> {

    pub fn new(i: &Image) -> Self {
    
        todo!();
        /*
        : image(i),

            image.duplicateIfShared();
            image.multiplyAllAlphas (0.8f);
            setAlwaysOnTop (true);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.drawImageAt (image, 0, 0);
        */
    }
}
