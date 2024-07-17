crate::ix!();

/**
  | Represents a margin.
  |
  */
#[derive(Default)]
pub struct GridItemMargin
{
    left:   f32,
    right:  f32,
    top:    f32,
    bottom: f32,
}

impl From<i32> for GridItemMargin {
    
    fn from(v: i32) -> Self {
    
        todo!();
        /*
            : GridItemMargin::GridItemMargin (static_cast<float> (v))
        */
    }
}

impl From<f32> for GridItemMargin {
    
    fn from(v: f32) -> Self {
    
        todo!();
        /*
        : left(v),
        : right(v),
        : top(v),
        : bottom(v),

        
        */
    }
}

impl GridItemMargin {

    /**
      | Creates a margin with these sizes.
      |
      */
    pub fn new(
        t: f32,
        r: f32,
        b: f32,
        l: f32) -> Self {
    
        todo!();
        /*
        : left(l),
        : right(r),
        : top(t),
        : bottom(b),

        
        */
    }
}
