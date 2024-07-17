crate::ix!();

/**
  | Represents a margin.
  |
  */
pub struct FlexItemMargin
{
    /**
      | Left margin size
      |
      */
    left:   f32,

    /**
      | Right margin size
      |
      */
    right:  f32,

    /**
      | Top margin size
      |
      */
    top:    f32,

    /**
      | Bottom margin size
      |
      */
    bottom: f32,
}

impl Default for FlexItemMargin {
    
    /**
      | Creates a margin of size zero.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : left(),
        : right(),
        : top(),
        : bottom(),

        
        */
    }
}

impl FlexItemMargin {

    pub fn new_from_f32(v: f32) -> Self {
    
        todo!();
        /*
        : left(v),
        : right(v),
        : top(v),
        : bottom(v),

        
        */
    }
    
    pub fn new_from_trbl(
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
    
    /**
      | Creates a margin with this size on all
      | sides.
      |
      */
    pub fn new_from_margin_on_all_sides(size: f32) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Creates a margin with these sizes.
      |
      */
    pub fn new(
        top:    f32,
        right:  f32,
        bottom: f32,
        left:   f32) -> Self {
    
        todo!();
        /*
        
        */
    }
}
