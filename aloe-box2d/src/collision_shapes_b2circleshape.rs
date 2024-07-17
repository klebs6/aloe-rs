crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2CircleShape.h]

/**
  | A circle shape.
  |
  */
pub struct b2CircleShape {

    base: b2Shape,

    /**
      | Position
      |
      */
    p: b2Vec2,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2CircleShape.cpp]
impl Default for b2CircleShape {

    fn default() -> Self {
    
        todo!();
        /*


            m_type = e_circle;
        m_radius = 0.0f;
        m_p.SetZero();
        */
    }
}

impl b2CircleShape {

    /**
      | Get the vertex count.
      |
      */
    pub fn get_vertex_count(&self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }

    /**
      | Get the supporting vertex index in the
      | given direction.
      |
      */
    #[inline] pub fn get_support(&self, d: &b2Vec2) -> i32 {
        
        todo!();
        /*
            B2_NOT_USED(d);
        return 0;
        */
    }
    
    /**
      | Get the supporting vertex in the given
      | direction.
      |
      */
    #[inline] pub fn get_support_vertex(&self, d: &b2Vec2) -> &b2Vec2 {
        
        todo!();
        /*
            B2_NOT_USED(d);
        return m_p;
        */
    }
    
    /**
      | Get a vertex by index. Used by b2Distance.
      |
      */
    #[inline] pub fn get_vertex(&self, index: i32) -> &b2Vec2 {
        
        todo!();
        /*
            B2_NOT_USED(index);
        b2Assert(index == 0);
        return m_p;
        */
    }
    
    /**
      | Implement b2Shape.
      |
      */
    pub fn clone(&self, allocator: *mut b2BlockAllocator) -> *mut b2Shape {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2CircleShape));
        b2CircleShape* clone = new (mem) b2CircleShape;
        *clone = *this;
        return clone;
        */
    }
    
    /**
      | @see b2Shape::GetChildCount
      |
      */
    pub fn get_child_count(&self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    /**
      | Implement b2Shape.
      |
      */
    pub fn test_point(&self, 
        transform: &b2Transform,
        p:         &b2Vec2) -> bool {
        
        todo!();
        /*
            b2Vec2 center = transform.p + b2Mul(transform.q, m_p);
        b2Vec2 d = p - center;
        return b2Dot(d, d) <= m_radius * m_radius;
        */
    }

    /**
      | Collision Detection in Interactive 3D
      | Environments by Gino van den Bergen
      |
      | From Section 3.1.2
      | x = s + a * r
      | norm(x) = radius
      */
    pub fn ray_cast(&self, 
        output:      *mut b2RayCastOutput,
        input:       &b2RayCastInput,
        transform:   &b2Transform,
        child_index: i32) -> bool {
        
        todo!();
        /*
            B2_NOT_USED(childIndex);

        b2Vec2 position = transform.p + b2Mul(transform.q, m_p);
        b2Vec2 s = input.p1 - position;
        float32 b = b2Dot(s, s) - m_radius * m_radius;

        // Solve quadratic equation.
        b2Vec2 r = input.p2 - input.p1;
        float32 c =  b2Dot(s, r);
        float32 rr = b2Dot(r, r);
        float32 sigma = c * c - rr * b;

        // Check for negative discriminant and short segment.
        if (sigma < 0.0f || rr < b2_epsilon)
        {
            return false;
        }

        // Find the point of intersection of the line with the circle.
        float32 a = -(c + b2Sqrt(sigma));

        // Is the intersection point on the segment?
        if (0.0f <= a && a <= input.maxFraction * rr)
        {
            a /= rr;
            output->fraction = a;
            output->normal = s + a * r;
            output->normal.Normalize();
            return true;
        }

        return false;
        */
    }
    
    /**
      | @see b2Shape::ComputeAABB
      |
      */
    pub fn computeaabb(&self, 
        aabb:        *mut b2AABB,
        transform:   &b2Transform,
        child_index: i32)  {
        
        todo!();
        /*
            B2_NOT_USED(childIndex);

        b2Vec2 p = transform.p + b2Mul(transform.q, m_p);
        aabb->lowerBound.Set(p.x - m_radius, p.y - m_radius);
        aabb->upperBound.Set(p.x + m_radius, p.y + m_radius);
        */
    }
    
    /**
      | @see b2Shape::ComputeMass
      |
      */
    pub fn compute_mass(&self, 
        mass_data: *mut b2MassData,
        density:   f32)  {
        
        todo!();
        /*
            massData->mass = density * b2_pi * m_radius * m_radius;
        massData->center = m_p;

        // inertia about the local origin
        massData->I = massData->mass * (0.5f * m_radius * m_radius + b2Dot(m_p, m_p));
        */
    }
}
