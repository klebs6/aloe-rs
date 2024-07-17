crate::ix!();

/**
  | Graphical rectangle structure. Used
  | with IPlugView. \ingroup pluginGUI
  |
  */
pub struct ViewRect {
    left:   i32,
    top:    i32,
    right:  i32,
    bottom: i32,
}

impl ViewRect {

    pub fn new(
        l: Option<i32>,
        t: Option<i32>,
        r: Option<i32>,
        b: Option<i32>

    ) -> Self {

        let l: i32 = l.unwrap_or(0);
        let t: i32 = t.unwrap_or(0);
        let r: i32 = r.unwrap_or(0);
        let b: i32 = b.unwrap_or(0);

        todo!();
        /*
        : left(l),
        : top(t),
        : right(r),
        : bottom(b),

        
        */
    }
    
    pub fn get_width(&self) -> i32 {
        
        todo!();
        /*
            return right - left;
        */
    }
    
    pub fn get_height(&self) -> i32 {
        
        todo!();
        /*
            return bottom - top;
        */
    }
}

smtg_type_size_check!{
    ViewRect, 
    16, 16, 16, 16
}
