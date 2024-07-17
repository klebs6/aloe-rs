/*!
  | This folder contains the source from the excellent
  | Box2D physics library.  For any Box2D-related
  | info, visit their website: http://box2d.org
  |
  | To create the aloe module, the only changes
  | required to the original source-code were to
  | adjust the include paths to be relative rather
  | than absolute, and to wrap #ifdefs around a couple
  | of unguarded header files. (Oh, and there were
  | a few compiler warnings that I cleaned up to
  | avoid bothering people with them)
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/utils/aloe_Box2DRenderer.h]

pub trait Box2dRendererInterface {

    /**
      | Converts a b2Color to a aloe Colour.
      |
      */
    fn get_colour(&self, _0: &b2Color) -> Colour;

    /**
      | Returns the thickness to use for drawing
      | outlines.
      |
      */
    fn get_line_thickness(&self) -> f32;

}

/**
  | A simple implementation of the b2Draw
  | class, used to draw a Box2D world.
  | 
  | To use it, simply create an instance
  | of this class in your paint() method,
  | and call its render() method.
  | 
  | @tags{Box2D}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Box2DRenderer<'a> {
    base:     b2Draw,
    graphics: *mut Graphics<'a>,
}

pub fn create_path(
        p:            &mut Path,
        vertices:     *const b2Vec2,
        vertex_count: i32)  {
    
    todo!();
    /*
        p.startNewSubPath (vertices[0].x, vertices[0].y);

        for (int i = 1; i < vertexCount; ++i)
            p.lineTo (vertices[i].x, vertices[i].y);

        p.closeSubPath();
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/utils/aloe_Box2DRenderer.cpp]
impl<'a> Default for Box2DRenderer<'a> {

    /* ---------------- b2Draw methods:  ---------------- */
    
    fn default() -> Self {
    
        todo!();
        /*
        : graphics(nullptr),

            SetFlags (e_shapeBit);
        */
    }
}

impl<'a> Box2DRenderer<'a> {
    
    /**
      | Renders the world.
      | 
      | -----------
      | @param g
      | 
      | the context to render into
      | ----------
      | @param world
      | 
      | the world to render
      | ----------
      | @param box2DWorldLeft
      | 
      | the left coordinate of the area of the
      | world to be drawn
      | ----------
      | @param box2DWorldTop
      | 
      | the top coordinate of the area of the
      | world to be drawn
      | ----------
      | @param box2DWorldRight
      | 
      | the right coordinate of the area of the
      | world to be drawn
      | ----------
      | @param box2DWorldBottom
      | 
      | the bottom coordinate of the area of
      | the world to be drawn
      | ----------
      | @param targetArea
      | 
      | the area within the target context onto
      | which the source world rectangle should
      | be mapped
      |
      */
    pub fn render(&mut self, 
        g:      &mut Graphics,
        world:  &mut b2World,
        left:   f32,
        top:    f32,
        right:  f32,
        bottom: f32,
        target: &Rectangle<f32>)  {
        
        todo!();
        /*
            graphics = &g;

        g.addTransform (AffineTransform::fromTargetPoints (left,  top,    target.getX(),     target.getY(),
                                                           right, top,    target.getRight(), target.getY(),
                                                           left,  bottom, target.getX(),     target.getBottom()));

        world.SetDebugDraw (this);
        world.DrawDebugData();
        */
    }
    
    pub fn get_colour(&self, c: &b2Color) -> Colour {
        
        todo!();
        /*
            return Colour::fromFloatRGBA (c.r, c.g, c.b, 1.0f);
        */
    }
    
    pub fn get_line_thickness(&self) -> f32 {
        
        todo!();
        /*
            return 0.1f;
        */
    }

    pub fn draw_polygon(&mut self, 
        vertices:     *const b2Vec2,
        vertex_count: i32,
        color:        &b2Color)  {
        
        todo!();
        /*
            graphics->setColour (getColour (color));

        Path p;
        createPath (p, vertices, vertexCount);
        graphics->strokePath (p, PathStrokeType (getLineThickness()));
        */
    }

    pub fn draw_solid_polygon(&mut self, 
        vertices:     *const b2Vec2,
        vertex_count: i32,
        color:        &b2Color)  {
        
        todo!();
        /*
            graphics->setColour (getColour (color));

        Path p;
        createPath (p, vertices, vertexCount);
        graphics->fillPath (p);
        */
    }

    pub fn draw_circle(&mut self, 
        center: &b2Vec2,
        radius: f32,
        color:  &b2Color)  {
        
        todo!();
        /*
            graphics->setColour (getColour (color));
        graphics->drawEllipse (center.x - radius, center.y - radius,
                               radius * 2.0f, radius * 2.0f,
                               getLineThickness());
        */
    }
    
    pub fn draw_solid_circle(&mut self, 
        center: &b2Vec2,
        radius: f32,
        axis:   &b2Vec2,
        colour: &b2Color)  {
        
        todo!();
        /*
            graphics->setColour (getColour (colour));
        graphics->fillEllipse (center.x - radius, center.y - radius,
                               radius * 2.0f, radius * 2.0f);
        */
    }
    
    pub fn draw_segment(&mut self, 
        p1:    &b2Vec2,
        p2:    &b2Vec2,
        color: &b2Color)  {
        
        todo!();
        /*
            graphics->setColour (getColour (color));
        graphics->drawLine (p1.x, p1.y, p2.x, p2.y, getLineThickness());
        */
    }
    
    pub fn draw_transform(&mut self, _0: &b2Transform)  {
        
        todo!();
        /*
        
        */
    }
}
