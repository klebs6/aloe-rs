crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2EdgeShape.h]

/**
  | A line segment (edge) shape. These can be
  | connected in chains or loops to other edge
  | shapes. The connectivity information is used
  | to ensure correct contact normals.
  */
pub struct b2EdgeShape {

    base: b2Shape,

    /**
      These are the edge vertices
      */
    vertex1:     b2Vec2,

    /**
      These are the edge vertices
      */
    vertex2:     b2Vec2,

    /**
      Optional adjacent vertices. These are used
      for smooth collision.
      */
    vertex0:     b2Vec2,

    /**
      Optional adjacent vertices. These are used
      for smooth collision.
      */
    vertex3:     b2Vec2,

    has_vertex0: bool,
    has_vertex3: bool,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2EdgeShape.cpp]
impl Default for b2EdgeShape {

    fn default() -> Self {
    
        todo!();
        /*
        m_type = e_edge;
        m_radius = b2_polygonRadius;
        m_vertex0.x = 0.0f;
        m_vertex0.y = 0.0f;
        m_vertex3.x = 0.0f;
        m_vertex3.y = 0.0f;
        m_hasVertex0 = false;
        m_hasVertex3 = false;
        */
    }
}

impl b2EdgeShape {

    /**
      | Set this as an isolated edge.
      |
      */
    pub fn set(&mut self, 
        v1: &b2Vec2,
        v2: &b2Vec2)  {
        
        todo!();
        /*
            m_vertex1 = v1;
        m_vertex2 = v2;
        m_hasVertex0 = false;
        m_hasVertex3 = false;
        */
    }
    
    /**
      | Implement b2Shape.
      |
      */
    pub fn clone(&self, allocator: *mut b2BlockAllocator) -> *mut b2Shape {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2EdgeShape));
        b2EdgeShape* clone = new (mem) b2EdgeShape;
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
      | @see b2Shape::TestPoint
      |
      */
    pub fn test_point(&self, 
        xf: &b2Transform,
        p:  &b2Vec2) -> bool {
        
        todo!();
        /*
            B2_NOT_USED(xf);
        B2_NOT_USED(p);
        return false;
        */
    }

    /**
      | p = p1 + t * d
      | v = v1 + s * e
      | p1 + t * d = v1 + s * e
      | s * e - t * d = p1 - v1
      */
    pub fn ray_cast(&self, 
        output:      *mut b2RayCastOutput,
        input:       &b2RayCastInput,
        xf:          &b2Transform,
        child_index: i32) -> bool {
        
        todo!();
        /*
            B2_NOT_USED(childIndex);

        // Put the ray into the edge's frame of reference.
        b2Vec2 p1 = b2MulT(xf.q, input.p1 - xf.p);
        b2Vec2 p2 = b2MulT(xf.q, input.p2 - xf.p);
        b2Vec2 d = p2 - p1;

        b2Vec2 v1 = m_vertex1;
        b2Vec2 v2 = m_vertex2;
        b2Vec2 e = v2 - v1;
        b2Vec2 normal(e.y, -e.x);
        normal.Normalize();

        // q = p1 + t * d
        // dot(normal, q - v1) = 0
        // dot(normal, p1 - v1) + t * dot(normal, d) = 0
        float32 numerator = b2Dot(normal, v1 - p1);
        float32 denominator = b2Dot(normal, d);

        if (denominator == 0.0f)
        {
            return false;
        }

        float32 t = numerator / denominator;
        if (t < 0.0f || input.maxFraction < t)
        {
            return false;
        }

        b2Vec2 q = p1 + t * d;

        // q = v1 + s * r
        // s = dot(q - v1, r) / dot(r, r)
        b2Vec2 r = v2 - v1;
        float32 rr = b2Dot(r, r);
        if (rr == 0.0f)
        {
            return false;
        }

        float32 s = b2Dot(q - v1, r) / rr;
        if (s < 0.0f || 1.0f < s)
        {
            return false;
        }

        output->fraction = t;
        if (numerator > 0.0f)
        {
            output->normal = -normal;
        }
        else
        {
            output->normal = normal;
        }
        return true;
        */
    }
    
    /**
      | @see b2Shape::ComputeAABB
      |
      */
    pub fn computeaabb(&self, 
        aabb:        *mut b2AABB,
        xf:          &b2Transform,
        child_index: i32)  {
        
        todo!();
        /*
            B2_NOT_USED(childIndex);

        b2Vec2 v1 = b2Mul(xf, m_vertex1);
        b2Vec2 v2 = b2Mul(xf, m_vertex2);

        b2Vec2 lower = b2Min(v1, v2);
        b2Vec2 upper = b2Max(v1, v2);

        b2Vec2 r(m_radius, m_radius);
        aabb->lowerBound = lower - r;
        aabb->upperBound = upper + r;
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
            B2_NOT_USED(density);

        massData->mass = 0.0f;
        massData->center = 0.5f * (m_vertex1 + m_vertex2);
        massData->I = 0.0f;
        */
    }
}
