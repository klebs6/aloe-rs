crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2PolygonShape.h]

pub fn compute_centroid(
        vs:    *const b2Vec2,
        count: i32) -> b2Vec2 {
    
    todo!();
    /*
        b2Assert(count >= 3);

        b2Vec2 c; c.Set(0.0f, 0.0f);
        float32 area = 0.0f;

        // pRef is the reference point for forming triangles.
        // It's location doesn't change the result (except for rounding error).
        b2Vec2 pRef(0.0f, 0.0f);
    #if 0
        // This code would put the reference point inside the polygon.
        for (int32 i = 0; i < count; ++i)
        {
            pRef += vs[i];
        }
        pRef *= 1.0f / count;
    #endif

        const float32 inv3 = 1.0f / 3.0f;

        for (int32 i = 0; i < count; ++i)
        {
            // Triangle vertices.
            b2Vec2 p1 = pRef;
            b2Vec2 p2 = vs[i];
            b2Vec2 p3 = i + 1 < count ? vs[i+1] : vs[0];

            b2Vec2 e1 = p2 - p1;
            b2Vec2 e2 = p3 - p1;

            float32 D = b2Cross(e1, e2);

            float32 triangleArea = 0.5f * D;
            area += triangleArea;

            // Area weighted centroid
            c += triangleArea * inv3 * (p1 + p2 + p3);
        }

        // Centroid
        b2Assert(area > b2_epsilon);
        c *= 1.0f / area;
        return c;
    */
}

/**
  | A convex polygon. It is assumed that the
  | interior of the polygon is to the left of each
  | edge.
  |
  | Polygons have a maximum number of vertices
  | equal to b2_maxPolygonVertices.
  |
  | In most cases you should not need many
  | vertices for a convex polygon.
  */
pub struct b2PolygonShape {
    base:         b2Shape,
    centroid:     b2Vec2,
    vertices:     [b2Vec2; b2_maxPolygonVertices],
    normals:      [b2Vec2; b2_maxPolygonVertices],
    vertex_count: i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/Shapes/b2PolygonShape.cpp]
impl Default for b2PolygonShape {

    fn default() -> Self {
    
        todo!();
        /*


            m_type = e_polygon;
        m_radius = b2_polygonRadius;
        m_vertexCount = 0;
        m_centroid.SetZero();
        */
    }
}
    
impl b2PolygonShape {

    /**
      | Get the vertex count.
      |
      */
    pub fn get_vertex_count(&self) -> i32 {
        
        todo!();
        /*
            return m_vertexCount;
        */
    }

    /**
      | Get a vertex by index.
      |
      */
    #[inline] pub fn get_vertex(&self, index: i32) -> &b2Vec2 {
        
        todo!();
        /*
            b2Assert(0 <= index && index < m_vertexCount);
        return m_vertices[index];
        */
    }
    
    /**
      | Implement b2Shape.
      |
      */
    pub fn clone(&self, allocator: *mut b2BlockAllocator) -> *mut b2Shape {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2PolygonShape));
        b2PolygonShape* clone = new (mem) b2PolygonShape;
        *clone = *this;
        return clone;
        */
    }
    
    /**
      | Build vertices to represent an axis-aligned
      | box.
      | 
      | -----------
      | @param hx
      | 
      | the half-width.
      | ----------
      | @param hy
      | 
      | the half-height.
      |
      */
    pub fn set_as_box_axis_aligned(&mut self, hx: f32, hy: f32)  {
        
        todo!();
        /*
            m_vertexCount = 4;
        m_vertices[0].Set(-hx, -hy);
        m_vertices[1].Set( hx, -hy);
        m_vertices[2].Set( hx,  hy);
        m_vertices[3].Set(-hx,  hy);
        m_normals[0].Set(0.0f, -1.0f);
        m_normals[1].Set(1.0f, 0.0f);
        m_normals[2].Set(0.0f, 1.0f);
        m_normals[3].Set(-1.0f, 0.0f);
        m_centroid.SetZero();
        */
    }
    
    /**
      | Build vertices to represent an oriented
      | box.
      | 
      | -----------
      | @param hx
      | 
      | the half-width.
      | ----------
      | @param hy
      | 
      | the half-height.
      | ----------
      | @param center
      | 
      | the center of the box in local coordinates.
      | ----------
      | @param angle
      | 
      | the rotation of the box in local coordinates.
      |
      */
    pub fn set_as_box(&mut self, 
        hx:     f32,
        hy:     f32,
        center: &b2Vec2,
        angle:  f32)  {
        
        todo!();
        /*
            m_vertexCount = 4;
        m_vertices[0].Set(-hx, -hy);
        m_vertices[1].Set( hx, -hy);
        m_vertices[2].Set( hx,  hy);
        m_vertices[3].Set(-hx,  hy);
        m_normals[0].Set(0.0f, -1.0f);
        m_normals[1].Set(1.0f, 0.0f);
        m_normals[2].Set(0.0f, 1.0f);
        m_normals[3].Set(-1.0f, 0.0f);
        m_centroid = center;

        b2Transform xf;
        xf.p = center;
        xf.q.Set(angle);

        // Transform vertices and normals.
        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            m_vertices[i] = b2Mul(xf, m_vertices[i]);
            m_normals[i] = b2Mul(xf.q, m_normals[i]);
        }
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
      | Copy vertices. This assumes the vertices
      | define a convex polygon.  It is assumed
      | that the exterior is the the right of each
      | edge.  The count must be in the range [3,
      | b2_maxPolygonVertices].
      */
    pub fn set(&mut self, 
        vertices: *const b2Vec2,
        count:    i32)  {
        
        todo!();
        /*
            b2Assert(3 <= count && count <= b2_maxPolygonVertices);
        m_vertexCount = count;

        // Copy vertices.
        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            m_vertices[i] = vertices[i];
        }

        // Compute normals. Ensure the edges have non-zero length.
        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            int32 i1 = i;
            int32 i2 = i + 1 < m_vertexCount ? i + 1 : 0;
            b2Vec2 edge = m_vertices[i2] - m_vertices[i1];
            b2Assert(edge.LengthSquared() > b2_epsilon * b2_epsilon);
            m_normals[i] = b2Cross(edge, 1.0f);
            m_normals[i].Normalize();
        }

    #ifdef _DEBUG
        // Ensure the polygon is convex and the interior
        // is to the left of each edge.
        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            int32 i1 = i;
            int32 i2 = i + 1 < m_vertexCount ? i + 1 : 0;
            b2Vec2 edge = m_vertices[i2] - m_vertices[i1];

            for (int32 j = 0; j < m_vertexCount; ++j)
            {
                // Don't check vertices on the current edge.
                if (j == i1 || j == i2)
                {
                    continue;
                }

                b2Vec2 r = m_vertices[j] - m_vertices[i1];

                // If this crashes, your polygon is non-convex, has colinear edges,
                // or the winding order is wrong.
                float32 s = b2Cross(edge, r);
                b2Assert(s > 0.0f && "ERROR: Please ensure your polygon is convex and has a CCW winding order");
            }
        }
    #endif

        // Compute the polygon centroid.
        m_centroid = ComputeCentroid(m_vertices, m_vertexCount);
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
            b2Vec2 pLocal = b2MulT(xf.q, p - xf.p);

        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            float32 dot = b2Dot(m_normals[i], pLocal - m_vertices[i]);
            if (dot > 0.0f)
            {
                return false;
            }
        }

        return true;
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
            B2_NOT_USED(childIndex);

        // Put the ray into the polygon's frame of reference.
        b2Vec2 p1 = b2MulT(xf.q, input.p1 - xf.p);
        b2Vec2 p2 = b2MulT(xf.q, input.p2 - xf.p);
        b2Vec2 d = p2 - p1;

        float32 lower = 0.0f, upper = input.maxFraction;

        int32 index = -1;

        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            // p = p1 + a * d
            // dot(normal, p - v) = 0
            // dot(normal, p1 - v) + a * dot(normal, d) = 0
            float32 numerator = b2Dot(m_normals[i], m_vertices[i] - p1);
            float32 denominator = b2Dot(m_normals[i], d);

            if (denominator == 0.0f)
            {
                if (numerator < 0.0f)
                {
                    return false;
                }
            }
            else
            {
                // Note: we want this predicate without division:
                // lower < numerator / denominator, where denominator < 0
                // Since denominator < 0, we have to flip the inequality:
                // lower < numerator / denominator <==> denominator * lower > numerator.
                if (denominator < 0.0f && numerator < lower * denominator)
                {
                    // Increase lower.
                    // The segment enters this half-space.
                    lower = numerator / denominator;
                    index = i;
                }
                else if (denominator > 0.0f && numerator < upper * denominator)
                {
                    // Decrease upper.
                    // The segment exits this half-space.
                    upper = numerator / denominator;
                }
            }

            // The use of epsilon here causes the assert on lower to trip
            // in some cases. Apparently the use of epsilon was to make edge
            // shapes work, but now those are handled separately.
            //if (upper < lower - b2_epsilon)
            if (upper < lower)
            {
                return false;
            }
        }

        b2Assert(0.0f <= lower && lower <= input.maxFraction);

        if (index >= 0)
        {
            output->fraction = lower;
            output->normal = b2Mul(xf.q, m_normals[index]);
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
        xf:          &b2Transform,
        child_index: i32)  {
        
        todo!();
        /*
            B2_NOT_USED(childIndex);

        b2Vec2 lower = b2Mul(xf, m_vertices[0]);
        b2Vec2 upper = lower;

        for (int32 i = 1; i < m_vertexCount; ++i)
        {
            b2Vec2 v = b2Mul(xf, m_vertices[i]);
            lower = b2Min(lower, v);
            upper = b2Max(upper, v);
        }

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
            // Polygon mass, centroid, and inertia.
        // Let rho be the polygon density in mass per unit area.
        // Then:
        // mass = rho * int(dA)
        // centroid.x = (1/mass) * rho * int(x * dA)
        // centroid.y = (1/mass) * rho * int(y * dA)
        // I = rho * int((x*x + y*y) * dA)
        //
        // We can compute these integrals by summing all the integrals
        // for each triangle of the polygon. To evaluate the integral
        // for a single triangle, we make a change of variables to
        // the (u,v) coordinates of the triangle:
        // x = x0 + e1x * u + e2x * v
        // y = y0 + e1y * u + e2y * v
        // where 0 <= u && 0 <= v && u + v <= 1.
        //
        // We integrate u from [0,1-v] and then v from [0,1].
        // We also need to use the Jacobian of the transformation:
        // D = cross(e1, e2)
        //
        // Simplification: triangle centroid = (1/3) * (p1 + p2 + p3)
        //
        // The rest of the derivation is handled by computer algebra.

        b2Assert(m_vertexCount >= 3);

        b2Vec2 center; center.Set(0.0f, 0.0f);
        float32 area = 0.0f;
        float32 I = 0.0f;

        // s is the reference point for forming triangles.
        // It's location doesn't change the result (except for rounding error).
        b2Vec2 s(0.0f, 0.0f);

        // This code would put the reference point inside the polygon.
        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            s += m_vertices[i];
        }
        s *= 1.0f / m_vertexCount;

        const float32 k_inv3 = 1.0f / 3.0f;

        for (int32 i = 0; i < m_vertexCount; ++i)
        {
            // Triangle vertices.
            b2Vec2 e1 = m_vertices[i] - s;
            b2Vec2 e2 = i + 1 < m_vertexCount ? m_vertices[i+1] - s : m_vertices[0] - s;

            float32 D = b2Cross(e1, e2);

            float32 triangleArea = 0.5f * D;
            area += triangleArea;

            // Area weighted centroid
            center += triangleArea * k_inv3 * (e1 + e2);

            float32 ex1 = e1.x, ey1 = e1.y;
            float32 ex2 = e2.x, ey2 = e2.y;

            float32 intx2 = ex1*ex1 + ex2*ex1 + ex2*ex2;
            float32 inty2 = ey1*ey1 + ey2*ey1 + ey2*ey2;

            I += (0.25f * k_inv3 * D) * (intx2 + inty2);
        }

        // Total mass
        massData->mass = density * area;

        // Center of mass
        b2Assert(area > b2_epsilon);
        center *= 1.0f / area;
        massData->center = center + s;

        // Inertia tensor relative to the local origin (point s).
        massData->I = density * I;

        // Shift to center of mass then to original body origin.
        massData->I += massData->mass * (b2Dot(massData->center, massData->center) - b2Dot(center, center));
        */
    }
}
