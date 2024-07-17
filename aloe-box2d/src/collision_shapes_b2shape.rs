crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2Shape.h]

/**
   This holds the mass data computed for a shape.
  */
pub struct b2MassData {

    /**
      The mass of the shape, usually in kilograms.
      */
    mass:   f32,

    /**
      The position of the shape's centroid
      relative to the shape's origin.
      */
    center: b2Vec2,

    /**
      The rotational inertia of the shape about
      the local origin.
      */
    i:      f32,
}

pub trait B2ShapeInterface {

    /**
      | Clone the concrete shape using the provided
      | allocator.
      |
      */
    fn clone(&self, allocator: *mut b2BlockAllocator) -> *mut b2Shape;

    /**
      | Get the number of child primitives.
      |
      */
    fn get_child_count(&self) -> i32;

    /**
      | Test a point for containment in this
      | shape. This only works for convex shapes.
      | 
      | -----------
      | @param xf
      | 
      | the shape world transform.
      | ----------
      | @param p
      | 
      | a point in world coordinates.
      |
      */
    fn test_point(&self, 
        xf: &b2Transform,
        p:  &b2Vec2) -> bool;

    /**
      | Cast a ray against a child shape.
      | 
      | -----------
      | @param output
      | 
      | the ray-cast results.
      | ----------
      | @param input
      | 
      | the ray-cast input parameters.
      | ----------
      | @param transform
      | 
      | the transform to be applied to the shape.
      | ----------
      | @param childIndex
      | 
      | the child shape index
      |
      */
    fn ray_cast(&self, 
        output:      *mut b2RayCastOutput,
        input:       &b2RayCastInput,
        transform:   &b2Transform,
        child_index: i32) -> bool;


    /**
      | Given a transform, compute the associated
      | axis aligned bounding box for a child
      | shape.
      | 
      | -----------
      | @param aabb
      | 
      | returns the axis aligned box.
      | ----------
      | @param xf
      | 
      | the world transform of the shape.
      | ----------
      | @param childIndex
      | 
      | the child shape
      |
      */
    fn computeaabb(&self, 
        aabb:        *mut b2AABB,
        xf:          &b2Transform,
        child_index: i32);

    /**
      | Compute the mass properties of this
      | shape using its dimensions and density.
      | The inertia tensor is computed about
      | the local origin.
      | 
      | -----------
      | @param massData
      | 
      | returns the mass data for this shape.
      | ----------
      | @param density
      | 
      | the density in kilograms per meter squared.
      |
      */
    fn compute_mass(
        &self, 
        mass_data: *mut b2MassData,
        density:   f32
    );

}

pub enum B2ShapeType
{
    e_circle    = 0,
    e_edge      = 1,
    e_polygon   = 2,
    e_chain     = 3,
    e_typeCount = 4
}

/**
  | A shape is used for collision detection. You
  | can create a shape however you like.  Shapes
  | used for simulation in b2World are created
  | automatically when a b2Fixture is
  | created. Shapes may encapsulate a one or more
  | child shapes.
  */
pub struct b2Shape {
    ty:     B2ShapeType,
    radius: f32,
}

impl b2Shape {

    /**
      | Get the type of this shape. You can use
      | this to down cast to the concrete shape.
      |
      | @return the shape type.
      */
    #[inline] pub fn get_type(&self) -> B2ShapeType {
        
        todo!();
        /*
            return m_type;
        */
    }
}
