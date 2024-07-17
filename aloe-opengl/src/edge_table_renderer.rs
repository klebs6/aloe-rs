crate::ix!();

#[no_copy]
pub struct EdgeTableRenderer<'a, QuadQueueType> {
    quad_queue: &'a mut QuadQueueType,
    colour:     PixelARGB,
    currenty:   i32,
}

impl<'a,QuadQueueType> EdgeTableRenderer<'a,QuadQueueType> {
    
    pub fn new(
        q: &mut QuadQueueType,
        c: PixelARGB) -> Self {
    
        todo!();
        /*
        : quad_queue(q),
        : colour(c),

        
        */
    }
    
    pub fn set_edge_table_ypos(&mut self, y: i32)  {
        
        todo!();
        /*
            currentY = y;
        */
    }
    
    pub fn handle_edge_table_pixel(&mut self, 
        x:           i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            auto c = colour;
                c.multiplyAlpha (alphaLevel);
                quadQueue.add (x, currentY, 1, 1, c);
        */
    }
    
    pub fn handle_edge_table_pixel_full(&mut self, x: i32)  {
        
        todo!();
        /*
            quadQueue.add (x, currentY, 1, 1, colour);
        */
    }
    
    pub fn handle_edge_table_line(&mut self, 
        x:           i32,
        width:       i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            auto c = colour;
                c.multiplyAlpha (alphaLevel);
                quadQueue.add (x, currentY, width, 1, c);
        */
    }
    
    pub fn handle_edge_table_line_full(&mut self, 
        x:     i32,
        width: i32)  {
        
        todo!();
        /*
            quadQueue.add (x, currentY, width, 1, colour);
        */
    }
    
    pub fn handle_edge_table_rectangle(&mut self, 
        x:           i32,
        y:           i32,
        width:       i32,
        height:      i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            auto c = colour;
                c.multiplyAlpha (alphaLevel);
                quadQueue.add (x, y, width, height, c);
        */
    }
    
    pub fn handle_edge_table_rectangle_full(&mut self, 
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            quadQueue.add (x, y, width, height, colour);
        */
    }
}
