crate::ix!();

/**
  | A size in pixels
  |
  */
pub struct GridPx {
    pixels: f64,
}

impl From<f32> for GridPx {

    fn from(p: f32) -> Self {
    
        todo!();
        /*
        : pixels (static_cast<long double>(p)) 
        /*sta (p >= 0.0f);*/
        */
    }
}
    
impl From<i32> for GridPx {

    fn from(p: i32) -> Self {
    
        todo!();
        /*
        : pixels (static_cast<long double>(p)) 
        /*sta (p >= 0.0f);*/
        */
    }
}
    
impl From<f64> for GridPx {

    fn from(p: f64) -> Self {
    
        todo!();
        /*
            : pixels (p)
        */
    }
}
    
impl From<u64> for GridPx {

    fn from(p: u64) -> Self {
    
        todo!();
        /*
            : pixels (static_cast<long double>(p))
        */
    }
}
