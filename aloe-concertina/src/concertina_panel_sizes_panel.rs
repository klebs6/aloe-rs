crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_ConcertinaPanel.h]

pub enum ConcertinaPanelSizesExpandMode
{
    stretchAll,
    stretchFirst,
    stretchLast
}

#[derive(Default)]
pub struct ConcertinaPanelSizesPanel {
    size:     i32,
    min_size: i32,
    max_size: i32,
}

impl ConcertinaPanelSizesPanel {

    pub fn new(
        sz: i32,
        mn: i32,
        mx: i32) -> Self {
    
        todo!();
        /*
        : size(sz),
        : min_size(mn),
        : max_size(mx),

        
        */
    }
    
    pub fn set_size(&mut self, new_size: i32) -> i32 {
        
        todo!();
        /*
            jassert (minSize <= maxSize);
                    auto oldSize = size;
                    size = jlimit (minSize, maxSize, newSize);
                    return size - oldSize;
        */
    }
    
    pub fn expand(&mut self, amount: i32) -> i32 {
        
        todo!();
        /*
            amount = jmin (amount, maxSize - size);
                    size += amount;
                    return amount;
        */
    }
    
    pub fn reduce(&mut self, amount: i32) -> i32 {
        
        todo!();
        /*
            amount = jmin (amount, size - minSize);
                    size -= amount;
                    return amount;
        */
    }
    
    pub fn can_expand(&self) -> bool {
        
        todo!();
        /*
            return size < maxSize;
        */
    }
    
    pub fn is_minimised(&self) -> bool {
        
        todo!();
        /*
            return size <= minSize;
        */
    }
}

