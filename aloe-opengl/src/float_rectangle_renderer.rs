crate::ix!();

#[no_copy]
pub struct FloatRectangleRenderer<'a, QuadQueueType> {
    quad_queue: &'a mut QuadQueueType,
    colour:     PixelARGB,
}

impl<'a, QuadQueueType> FloatRectangleRenderer<'a, QuadQueueType> {

    pub fn new(
        q: &mut QuadQueueType,
        c: PixelARGB) -> Self {
    
        todo!();
        /*
        : quad_queue(q),
        : colour(c),

        
        */
    }
    
    pub fn invoke(&mut self, 
        x:     i32,
        y:     i32,
        w:     i32,
        h:     i32,
        alpha: i32)  {
        
        todo!();
        /*
            if (w > 0 && h > 0)
                {
                    PixelARGB c (colour);
                    c.multiplyAlpha (alpha);
                    quadQueue.add (x, y, w, h, c);
                }
        */
    }
}
