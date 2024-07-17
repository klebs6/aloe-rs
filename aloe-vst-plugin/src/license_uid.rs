crate::ix!();

macro_rules! licence_uid {
    ($l1:ident, 
     $l2:ident, 
     $l3:ident, 
     $l4:ident) => {
        /*
        
        { 
            (int8)((l1 & 0xFF000000) >> 24), (int8)((l1 & 0x00FF0000) >> 16), 
            (int8)((l1 & 0x0000FF00) >>  8), (int8)((l1 & 0x000000FF)      ), 
            (int8)((l2 & 0xFF000000) >> 24), (int8)((l2 & 0x00FF0000) >> 16), 
            (int8)((l2 & 0x0000FF00) >>  8), (int8)((l2 & 0x000000FF)      ), 
            (int8)((l3 & 0xFF000000) >> 24), (int8)((l3 & 0x00FF0000) >> 16), 
            (int8)((l3 & 0x0000FF00) >>  8), (int8)((l3 & 0x000000FF)      ), 
            (int8)((l4 & 0xFF000000) >> 24), (int8)((l4 & 0x00FF0000) >> 16), 
            (int8)((l4 & 0x0000FF00) >>  8), (int8)((l4 & 0x000000FF)      )  
        }
        */
    }
}
