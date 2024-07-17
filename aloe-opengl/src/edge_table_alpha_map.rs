crate::ix!();

#[no_copy]
pub struct EdgeTableAlphaMap {
    data:         HeapBlock<u8>,
    area:         Rectangle<i32>,
    current_line: *mut u8,
}

impl EdgeTableAlphaMap {

    pub fn new(et: &EdgeTable) -> Self {
    
        todo!();
        /*


            : area (et.getMaximumBounds().withSize (nextPowerOfTwo (et.getMaximumBounds().getWidth()),
                                                        nextPowerOfTwo (et.getMaximumBounds().getHeight())))

                data.calloc (area.getWidth() * area.getHeight());
                et.iterate (*this);
        */
    }
    
    #[inline] pub fn set_edge_table_ypos(&mut self, y: i32)  {
        
        todo!();
        /*
            currentLine = data + (area.getBottom() - 1 - y) * area.getWidth() - area.getX();
        */
    }
    
    #[inline] pub fn handle_edge_table_pixel(&self, 
        x:           i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            currentLine[x] = (uint8) alphaLevel;
        */
    }
    
    #[inline] pub fn handle_edge_table_pixel_full(&self, x: i32)  {
        
        todo!();
        /*
            currentLine[x] = 255;
        */
    }
    
    #[inline] pub fn handle_edge_table_line(&self, 
        x:           i32,
        width:       i32,
        alpha_level: i32)  {
        
        todo!();
        /*
            memset (currentLine + x, (uint8) alphaLevel, (size_t) width);
        */
    }
    
    #[inline] pub fn handle_edge_table_line_full(&self, 
        x:     i32,
        width: i32)  {
        
        todo!();
        /*
            memset (currentLine + x, 255, (size_t) width);
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
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLine (x, width, alphaLevel);
                }
        */
    }
    
    pub fn handle_edge_table_rectangle_full(&mut self, 
        x:      i32,
        y:      i32,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            while (--height >= 0)
                {
                    setEdgeTableYPos (y++);
                    handleEdgeTableLineFull (x, width);
                }
        */
    }
}
