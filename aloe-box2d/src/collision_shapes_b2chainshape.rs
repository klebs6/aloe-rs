crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2ChainShape.h]

/**
  | A chain shape is a free form sequence of line
  | segments.
  |
  | The chain has two-sided collision, so you can
  | use inside and outside collision.  Therefore,
  | you may use any winding order.
  |
  | Since there may be many vertices, they are
  | allocated using b2Alloc.
  |
  | Connectivity information is used to create
  | smooth collisions.
  |
  | WARNING: The chain will not collide properly
  | if there are self-intersections.
  */
pub struct b2ChainShape {

    base: b2Shape,

    /**
      The vertices. Owned by this class.
      */
    vertices:        *mut b2Vec2,

    /**
      The vertex count.
      */
    count:           i32,

    prev_vertex:     b2Vec2,
    next_vertex:     b2Vec2,
    has_prev_vertex: bool,
    has_next_vertex: bool,
}

impl Drop for b2ChainShape {

    /**
       The destructor frees the vertices using
       b2Free.
      */
    fn drop(&mut self) {
        todo!();
        /* 
        b2Free(m_vertices);
        m_vertices = NULL;
        m_count = 0;
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2ChainShape.cpp]
impl Default for b2ChainShape {

    fn default() -> Self {
    
        todo!();
        /*


            m_type = e_chain;
        m_radius = b2_polygonRadius;
        m_vertices = NULL;
        m_count = 0;
        m_hasPrevVertex = 0;
        m_hasNextVertex = 0;
        */
    }
}

impl b2ChainShape {
    
    /**
      | Create a loop. This automatically adjusts
      | connectivity.
      | 
      | -----------
      | @param vertices
      | 
      | an array of vertices, these are copied
      | ----------
      | @param count
      | 
      | the vertex count
      |
      */
    pub fn create_loop(&mut self, 
        vertices: *const b2Vec2,
        count:    i32)  {
        
        todo!();
        /*
            b2Assert(m_vertices == NULL && m_count == 0);
        b2Assert(count >= 3);
        m_count = count + 1;
        m_vertices = (b2Vec2*)b2Alloc(m_count * sizeof(b2Vec2));
        memcpy(m_vertices, vertices, count * sizeof(b2Vec2));
        m_vertices[count] = m_vertices[0];
        m_prevVertex = m_vertices[m_count - 2];
        m_nextVertex = m_vertices[1];
        m_hasPrevVertex = true;
        m_hasNextVertex = true;
        */
    }
    
    /**
      | Create a chain with isolated end vertices.
      | 
      | -----------
      | @param vertices
      | 
      | an array of vertices, these are copied
      | ----------
      | @param count
      | 
      | the vertex count
      |
      */
    pub fn create_chain(&mut self, 
        vertices: *const b2Vec2,
        count:    i32)  {
        
        todo!();
        /*
            b2Assert(m_vertices == NULL && m_count == 0);
        b2Assert(count >= 2);
        m_count = count;
        m_vertices = (b2Vec2*)b2Alloc(count * sizeof(b2Vec2));
        memcpy(m_vertices, vertices, m_count * sizeof(b2Vec2));
        m_hasPrevVertex = false;
        m_hasNextVertex = false;
        */
    }
    
    /**
      | Establish connectivity to a vertex that
      | precedes the first vertex.
      |
      | Don't call this for loops.
      */
    pub fn set_prev_vertex(&mut self, prev_vertex: &b2Vec2)  {
        
        todo!();
        /*
            m_prevVertex = prevVertex;
        m_hasPrevVertex = true;
        */
    }
    
    /**
      | Establish connectivity to a vertex that
      | follows the last vertex.
      |
      | Don't call this for loops.
      */
    pub fn set_next_vertex(&mut self, next_vertex: &b2Vec2)  {
        
        todo!();
        /*
            m_nextVertex = nextVertex;
        m_hasNextVertex = true;
        */
    }
    
    /**
      | Implement b2Shape. Vertices are cloned
      | using b2Alloc.
      |
      */
    pub fn clone(&self, allocator: *mut b2BlockAllocator) -> *mut b2Shape {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2ChainShape));
        b2ChainShape* clone = new (mem) b2ChainShape;
        clone->CreateChain(m_vertices, m_count);
        clone->m_prevVertex = m_prevVertex;
        clone->m_nextVertex = m_nextVertex;
        clone->m_hasPrevVertex = m_hasPrevVertex;
        clone->m_hasNextVertex = m_hasNextVertex;
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
            // edge count = vertex count - 1
        return m_count - 1;
        */
    }
    
    /**
      | Get a child edge.
      |
      */
    pub fn get_child_edge(&self, 
        edge:  *mut b2EdgeShape,
        index: i32)  {
        
        todo!();
        /*
            b2Assert(0 <= index && index < m_count - 1);
        edge->m_type = b2Shape::e_edge;
        edge->m_radius = m_radius;

        edge->m_vertex1 = m_vertices[index + 0];
        edge->m_vertex2 = m_vertices[index + 1];

        if (index > 0)
        {
            edge->m_vertex0 = m_vertices[index - 1];
            edge->m_hasVertex0 = true;
        }
        else
        {
            edge->m_vertex0 = m_prevVertex;
            edge->m_hasVertex0 = m_hasPrevVertex;
        }

        if (index < m_count - 2)
        {
            edge->m_vertex3 = m_vertices[index + 2];
            edge->m_hasVertex3 = true;
        }
        else
        {
            edge->m_vertex3 = m_nextVertex;
            edge->m_hasVertex3 = m_hasNextVertex;
        }
        */
    }
    
    /**
      | This always return false.
      | 
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
      | Implement b2Shape.
      |
      */
    pub fn ray_cast(&self, 
        output:      *mut b2RayCastOutput,
        input:       &b2RayCastInput,
        xf:          &b2Transform,
        child_index: i32) -> bool {
        
        todo!();
        /*
            b2Assert(childIndex < m_count);

        b2EdgeShape edgeShape;

        int32 i1 = childIndex;
        int32 i2 = childIndex + 1;
        if (i2 == m_count)
        {
            i2 = 0;
        }

        edgeShape.m_vertex1 = m_vertices[i1];
        edgeShape.m_vertex2 = m_vertices[i2];

        return edgeShape.RayCast(output, input, xf, 0);
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
            b2Assert(childIndex < m_count);

        int32 i1 = childIndex;
        int32 i2 = childIndex + 1;
        if (i2 == m_count)
        {
            i2 = 0;
        }

        b2Vec2 v1 = b2Mul(xf, m_vertices[i1]);
        b2Vec2 v2 = b2Mul(xf, m_vertices[i2]);

        aabb->lowerBound = b2Min(v1, v2);
        aabb->upperBound = b2Max(v1, v2);
        */
    }
    
    /**
      | Chains have zero mass. @see b2Shape::ComputeMass
      |
      */
    pub fn compute_mass(&self, 
        mass_data: *mut b2MassData,
        density:   f32)  {
        
        todo!();
        /*
            B2_NOT_USED(density);

        massData->mass = 0.0f;
        massData->center.SetZero();
        massData->I = 0.0f;
        */
    }
}
