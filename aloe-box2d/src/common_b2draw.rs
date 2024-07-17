crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Draw.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Draw.cpp]

/**
  | Color for debug drawing. Each value
  | has the range [0,1].
  |
  */
pub struct b2Color
{
    r: f32,
    g: f32,
    b: f32,
}

impl b2Color {
    
    pub fn new(
        red:   f32,
        green: f32,
        blue:  f32) -> Self {
    
        todo!();
        /*
        : r(red),
        : g(green),
        : b(blue),

        
        */
    }
    
    pub fn set(&mut self, 
        ri: f32,
        gi: f32,
        bi: f32)  {
        
        todo!();
        /*
            r = ri; g = gi; b = bi;
        */
    }
}

pub trait B2DrawInterface {

    /**
      | Draw a closed polygon provided in CCW
      | order.
      |
      */
    fn draw_polygon(&mut self, 
        vertices:     *const b2Vec2,
        vertex_count: i32,
        color:        &b2Color);

    /**
      | Draw a solid closed polygon provided
      | in CCW order.
      |
      */
    fn draw_solid_polygon(&mut self, 
        vertices:     *const b2Vec2,
        vertex_count: i32,
        color:        &b2Color);

    /**
      | Draw a circle.
      |
      */
    fn draw_circle(&mut self, 
        center: &b2Vec2,
        radius: f32,
        color:  &b2Color);

    /**
      | Draw a solid circle.
      |
      */
    fn draw_solid_circle(&mut self, 
        center: &b2Vec2,
        radius: f32,
        axis:   &b2Vec2,
        color:  &b2Color);

    /**
      | Draw a line segment.
      |
      */
    fn draw_segment(&mut self, 
        p1:    &b2Vec2,
        p2:    &b2Vec2,
        color: &b2Color);

    /**
      | Draw a transform. Choose your own length
      | scale.
      | 
      | -----------
      | @param xf
      | 
      | a transform.
      |
      */
    fn draw_transform(&mut self, xf: &b2Transform);
}

/**
  | Implement and register this class with
  | a b2World to provide debug drawing of
  | physics entities in your game.
  |
  */
pub struct b2Draw {
    draw_flags: u32,
}

/**
  | draw shapes
  |
  */
pub const B2_DRAW_E_SHAPE_BIT:        usize = 0x0001;   

/**
  | draw joint connections
  |
  */
pub const B2_DRAW_E_JOINT_BIT:        usize = 0x0002;   

/**
  | draw axis aligned bounding boxes
  |
  */
pub const B2_DRAW_E_AABB_BIT:         usize = 0x0004;   

/**
  | draw broad-phase pairs
  |
  */
pub const B2_DRAW_E_PAIR_BIT:         usize = 0x0008;   

/**
  | draw center of mass frame
  |
  */
pub const B2_DRAW_E_CENTER_OF_MASS_BIT: usize = 0x0010;   

impl Default for b2Draw {

    fn default() -> Self {
    
        todo!();
        /*


            m_drawFlags = 0;
        */
    }
}

impl b2Draw {

    /**
      | Set the drawing flags.
      |
      */
    pub fn set_flags(&mut self, flags: u32)  {
        
        todo!();
        /*
            m_drawFlags = flags;
        */
    }
    
    /**
      | Get the drawing flags.
      |
      */
    pub fn get_flags(&self) -> u32 {
        
        todo!();
        /*
            return m_drawFlags;
        */
    }
    
    /**
      | Append flags to the current flags.
      |
      */
    pub fn append_flags(&mut self, flags: u32)  {
        
        todo!();
        /*
            m_drawFlags |= flags;
        */
    }
    
    /**
      | Clear flags from the current flags.
      |
      */
    pub fn clear_flags(&mut self, flags: u32)  {
        
        todo!();
        /*
            m_drawFlags &= ~flags;
        */
    }
}
