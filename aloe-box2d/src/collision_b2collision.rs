/*!
  | @file
  |
  | Structures and functions used for computing
  | contact points, distance queries, and TOI
  | queries.
  */

use super::*;

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2Collision.h]

pub const b2_nullFeature: u8 = u8::MAX;

pub enum B2ContactFeatureType
{
    e_vertex = 0,
    e_face = 1
}

/**
  | The features that intersect to form
  | the contact point This must be 4 bytes
  | or less.
  |
  */
#[derive(Copy,Clone)]
pub struct b2ContactFeature {

    /**
      | Feature index on shapeA
      |
      */
    indexa: u8,

    /**
      | Feature index on shapeB
      |
      */
    indexb: u8,

    /**
      | The feature type on shapeA
      |
      */
    typea:  u8,

    /**
      | The feature type on shapeB
      |
      */
    typeb:  u8,
}

/**
  | Contact ids to facilitate warm starting.
  |
  */
pub union b2ContactID
{
    cf:  b2ContactFeature,

    /**
      | Used to quickly compare contact ids.
      |
      */
    key: u32,
}

/**
  | A manifold point is a contact point belonging
  | to a contact manifold. It holds details
  | related to the geometry and dynamics
  | of the contact points.
  | 
  | The local point usage depends on the
  | manifold type:
  | 
  | -e_circles: the local center of circleB
  | 
  | -e_faceA: the local center of cirlceB
  | or the clip point of polygonB
  | 
  | -e_faceB: the clip point of polygonA
  | 
  | This structure is stored across time
  | steps, so we keep it small. Note: the
  | impulses are used for internal caching
  | and may not provide reliable contact
  | forces, especially for high speed collisions.
  |
  */
pub struct b2ManifoldPoint {

    /**
      | usage depends on manifold type
      |
      */
    local_point:     b2Vec2,

    /**
      | the non-penetration impulse
      |
      */
    normal_impulse:  f32,

    /**
      | the friction impulse
      |
      */
    tangent_impulse: f32,

    /**
      | uniquely identifies a contact point
      | between two shapes
      |
      */
    id:              b2ContactID,
}

/**
  | A manifold for two touching convex shapes.
  | Box2D supports multiple types of contact:
  |
  | - clip point versus plane with radius
  |
  | - point versus point with radius (circles) The
  | local point usage depends on the manifold
  | type:
  |
  | -e_circles: the local center of circleA
  |
  | -e_faceA: the center of faceA
  |
  | -e_faceB: the center of faceB Similarly the
  | local normal usage:
  |
  | -e_circles: not used
  |
  | -e_faceA: the normal on polygonA
  |
  | -e_faceB: the normal on polygonB
  |
  | We store contacts in this way so that position
  | correction can account for movement, which is
  | critical for continuous physics.  All contact
  | scenarios must be expressed in one of these
  | types.  This structure is stored across time
  | steps, so we keep it small.
  */
pub struct b2Manifold {

    /**
      | the points of contact
      |
      */
    points:       [b2ManifoldPoint; b2_maxManifoldPoints],

    /**
      | not use for B2ManifoldType::e_points
      |
      */
    local_normal: b2Vec2,

    /**
      | usage depends on manifold type
      |
      */
    local_point:  b2Vec2,

    ty:           B2ManifoldType,

    /**
      | the number of manifold points
      |
      */
    point_count:  i32,
}

pub enum B2ManifoldType
{
    Circles,
    FaceA,
    FaceB
}

/**
  | This is used to compute the current state
  | of a contact manifold.
  |
  */
pub struct b2WorldManifold {

    /**
      | world vector pointing from A to B
      |
      */
    normal: b2Vec2,

    /**
      | world contact point (point of intersection)
      |
      */
    points: [b2Vec2; b2_maxManifoldPoints],
}

/**
  | This is used for determining the state
  | of contact points.
  |
  */
pub enum b2PointState
{
    /**
      | point does not exist
      |
      */
    b2_nullState,       

    /**
      | point was added in the update
      |
      */
    b2_addState,        

    /**
      | point persisted across the update
      |
      */
    b2_persistState,    

    /**
      | point was removed in the update
      |
      */
    b2_removeState      
}

/**
  | Used for computing contact manifolds.
  |
  */
pub struct b2ClipVertex
{
    v:  b2Vec2,
    id: b2ContactID,
}

/**
  | Ray-cast input data. The ray extends
  | from p1 to p1 + maxFraction * (p2 - p1).
  |
  */
pub struct b2RayCastInput
{
    p1:           b2Vec2,
    p2:           b2Vec2,
    max_fraction: f32,
}

/**
  | Ray-cast output data. The ray hits at
  | p1 + fraction * (p2 - p1), where p1 and
  | p2 come from b2RayCastInput.
  |
  */
pub struct b2RayCastOutput
{
    normal:   b2Vec2,
    fraction: f32,
}

/**
  | An axis aligned bounding box.
  |
  */
pub struct b2AABB {

    /**
      | the lower vertex
      |
      */
    lower_bound: b2Vec2,

    /**
      | the upper vertex
      |
      */
    upper_bound: b2Vec2,
}

impl b2AABB {

    /**
      | Get the center of the AABB.
      |
      */
    pub fn get_center(&self) -> b2Vec2 {
        
        todo!();
        /*
            return 0.5f * (lowerBound + upperBound);
        */
    }

    /**
      | Get the extents of the AABB (half-widths).
      |
      */
    pub fn get_extents(&self) -> b2Vec2 {
        
        todo!();
        /*
            return 0.5f * (upperBound - lowerBound);
        */
    }

    /**
      | Get the perimeter length
      |
      */
    pub fn get_perimeter(&self) -> f32 {
        
        todo!();
        /*
            float32 wx = upperBound.x - lowerBound.x;
            float32 wy = upperBound.y - lowerBound.y;
            return 2.0f * (wx + wy);
        */
    }

    /**
      | Combine an AABB into this one.
      |
      */
    pub fn combine(&mut self, aabb: &b2AABB)  {
        
        todo!();
        /*
            lowerBound = b2Min(lowerBound, aabb.lowerBound);
            upperBound = b2Max(upperBound, aabb.upperBound);
        */
    }

    /**
      | Combine two AABBs into this one.
      |
      */
    pub fn combine_two(
        &mut self, 
        aabb1: &b2AABB,
        aabb2: &b2AABB

    ) {
        
        todo!();
        /*
            lowerBound = b2Min(aabb1.lowerBound, aabb2.lowerBound);
            upperBound = b2Max(aabb1.upperBound, aabb2.upperBound);
        */
    }

    /**
      | Does this aabb contain the provided
      | AABB.
      |
      */
    pub fn contains(&self, aabb: &b2AABB) -> bool {
        
        todo!();
        /*
            bool result = true;
            result = result && lowerBound.x <= aabb.lowerBound.x;
            result = result && lowerBound.y <= aabb.lowerBound.y;
            result = result && aabb.upperBound.x <= upperBound.x;
            result = result && aabb.upperBound.y <= upperBound.y;
            return result;
        */
    }
}

/**
   Compute the collision manifold between two
   circles.
  */
pub fn b2collide_circles(
        manifold: *mut b2Manifold,
        circlea:  *const b2CircleShape,
        xfa:      &b2Transform,
        circleb:  *const b2CircleShape,
        xfb:      &b2Transform)  {
    
    todo!();
    /*
    
    */
}

/**
   Compute the collision manifold between
   a polygon and a circle.
  */
pub fn b2collide_polygon_and_circle(
        manifold: *mut b2Manifold,
        polygona: *const b2PolygonShape,
        xfa:      &b2Transform,
        circleb:  *const b2CircleShape,
        xfb:      &b2Transform)  {
    
    todo!();
    /*
    
    */
}

/**
   Compute the collision manifold between two
   polygons.
  */
pub fn b2collide_polygons(
        manifold: *mut b2Manifold,
        polygona: *const b2PolygonShape,
        xfa:      &b2Transform,
        polygonb: *const b2PolygonShape,
        xfb:      &b2Transform)  {
    
    todo!();
    /*
    
    */
}

/**
   Compute the collision manifold between an edge
   and a circle.
  */
pub fn b2collide_edge_and_circle(
        manifold: *mut b2Manifold,
        polygona: *const b2EdgeShape,
        xfa:      &b2Transform,
        circleb:  *const b2CircleShape,
        xfb:      &b2Transform)  {
    
    todo!();
    /*
    
    */
}

/**
   Compute the collision manifold between an edge
   and a circle.
  */
pub fn b2collide_edge_and_polygon(
        manifold: *mut b2Manifold,
        edgea:    *const b2EdgeShape,
        xfa:      &b2Transform,
        circleb:  *const b2PolygonShape,
        xfb:      &b2Transform)  {
    
    todo!();
    /*
    
    */
}

// ---------------- Inline Functions ------------------------------------------

impl b2AABB {

    /**
      | Verify that the bounds are sorted.
      |
      */
    #[inline] pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            b2Vec2 d = upperBound - lowerBound;
        bool valid = d.x >= 0.0f && d.y >= 0.0f;
        valid = valid && lowerBound.IsValid() && upperBound.IsValid();
        return valid;
        */
    }
}

#[inline] pub fn b2test_overlap_aabb(
        a: &b2AABB,
        b: &b2AABB) -> bool {
    
    todo!();
    /*
        b2Vec2 d1, d2;
        d1 = b.lowerBound - a.upperBound;
        d2 = a.lowerBound - b.upperBound;

        if (d1.x > 0.0f || d1.y > 0.0f)
            return false;

        if (d2.x > 0.0f || d2.y > 0.0f)
            return false;

        return true;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2Collision.cpp]

impl b2WorldManifold {

    /**
      | Evaluate the manifold with supplied
      | transforms. This assumes modest motion
      | from the original state. This does not
      | change the point count, impulses, etc. The
      | radii must come from the shapes that
      | generated the manifold.
      */
    pub fn initialize(&mut self, 
        manifold: *const b2Manifold,
        xfa:      &b2Transform,
        radiusa:  f32,
        xfb:      &b2Transform,
        radiusb:  f32)  {
        
        todo!();
        /*
            if (manifold->pointCount == 0)
        {
            return;
        }

        switch (manifold->type)
        {
        case b2Manifold::e_circles:
            {
                normal.Set(1.0f, 0.0f);
                b2Vec2 pointA = b2Mul(xfA, manifold->localPoint);
                b2Vec2 pointB = b2Mul(xfB, manifold->points[0].localPoint);
                if (b2DistanceSquared(pointA, pointB) > b2_epsilon * b2_epsilon)
                {
                    normal = pointB - pointA;
                    normal.Normalize();
                }

                b2Vec2 cA = pointA + radiusA * normal;
                b2Vec2 cB = pointB - radiusB * normal;
                points[0] = 0.5f * (cA + cB);
            }
            break;

        case b2Manifold::e_faceA:
            {
                normal = b2Mul(xfA.q, manifold->localNormal);
                b2Vec2 planePoint = b2Mul(xfA, manifold->localPoint);

                for (int32 i = 0; i < manifold->pointCount; ++i)
                {
                    b2Vec2 clipPoint = b2Mul(xfB, manifold->points[i].localPoint);
                    b2Vec2 cA = clipPoint + (radiusA - b2Dot(clipPoint - planePoint, normal)) * normal;
                    b2Vec2 cB = clipPoint - radiusB * normal;
                    points[i] = 0.5f * (cA + cB);
                }
            }
            break;

        case b2Manifold::e_faceB:
            {
                normal = b2Mul(xfB.q, manifold->localNormal);
                b2Vec2 planePoint = b2Mul(xfB, manifold->localPoint);

                for (int32 i = 0; i < manifold->pointCount; ++i)
                {
                    b2Vec2 clipPoint = b2Mul(xfA, manifold->points[i].localPoint);
                    b2Vec2 cB = clipPoint + (radiusB - b2Dot(clipPoint - planePoint, normal)) * normal;
                    b2Vec2 cA = clipPoint - radiusA * normal;
                    points[i] = 0.5f * (cA + cB);
                }

                // Ensure normal points from A to B.
                normal = -normal;
            }
            break;
        }
        */
    }
}

/**
  | Compute the point states given two
  | manifolds. The states pertain to the
  | transition from manifold1 to manifold2. So
  | state1 is either persist or remove while
  | state2 is either add or persist.
  */
pub fn b2get_point_states(
        state1:    [b2PointState; b2_maxManifoldPoints],
        state2:    [b2PointState; b2_maxManifoldPoints],
        manifold1: *const b2Manifold,
        manifold2: *const b2Manifold)  {
    
    todo!();
    /*
        for (int32 i = 0; i < b2_maxManifoldPoints; ++i)
        {
            state1[i] = b2_nullState;
            state2[i] = b2_nullState;
        }

        // Detect persists and removes.
        for (int32 i = 0; i < manifold1->pointCount; ++i)
        {
            b2ContactID id = manifold1->points[i].id;

            state1[i] = b2_removeState;

            for (int32 j = 0; j < manifold2->pointCount; ++j)
            {
                if (manifold2->points[j].id.key == id.key)
                {
                    state1[i] = b2_persistState;
                    break;
                }
            }
        }

        // Detect persists and adds.
        for (int32 i = 0; i < manifold2->pointCount; ++i)
        {
            b2ContactID id = manifold2->points[i].id;

            state2[i] = b2_addState;

            for (int32 j = 0; j < manifold1->pointCount; ++j)
            {
                if (manifold1->points[j].id.key == id.key)
                {
                    state2[i] = b2_persistState;
                    break;
                }
            }
        }
    */
}

impl b2AABB {
    
    /**
      | From Real-time Collision Detection,
      | p179.
      |
      */
    pub fn ray_cast(&self, 
        output: *mut b2RayCastOutput,
        input:  &b2RayCastInput) -> bool {
        
        todo!();
        /*
            float32 tmin = -b2_maxFloat;
        float32 tmax = b2_maxFloat;

        b2Vec2 p = input.p1;
        b2Vec2 d = input.p2 - input.p1;
        b2Vec2 absD = b2Abs(d);

        b2Vec2 normal;

        for (int32 i = 0; i < 2; ++i)
        {
            if (absD(i) < b2_epsilon)
            {
                // Parallel.
                if (p(i) < lowerBound(i) || upperBound(i) < p(i))
                {
                    return false;
                }
            }
            else
            {
                float32 inv_d = 1.0f / d(i);
                float32 t1 = (lowerBound(i) - p(i)) * inv_d;
                float32 t2 = (upperBound(i) - p(i)) * inv_d;

                // Sign of the normal vector.
                float32 s = -1.0f;

                if (t1 > t2)
                {
                    b2Swap(t1, t2);
                    s = 1.0f;
                }

                // Push the min up
                if (t1 > tmin)
                {
                    normal.SetZero();
                    normal(i) = s;
                    tmin = t1;
                }

                // Pull the max down
                tmax = b2Min(tmax, t2);

                if (tmin > tmax)
                {
                    return false;
                }
            }
        }

        // Does the ray start inside the box?
        // Does the ray intersect beyond the max fraction?
        if (tmin < 0.0f || input.maxFraction < tmin)
        {
            return false;
        }

        // Intersection.
        output->fraction = tmin;
        output->normal = normal;
        return true;
        */
    }
}

/**
  | Sutherland-Hodgman clipping.
  |
  | Clipping for contact manifolds.
  */
pub fn b2clip_segment_to_line(
        out:           [b2ClipVertex; 2],
        in_:           [b2ClipVertex; 2],
        normal:        &b2Vec2,
        offset:        f32,
        vertex_indexa: i32) -> i32 {
    
    todo!();
    /*
        // Start with no output points
        int32 numOut = 0;

        // Calculate the distance of end points to the line
        float32 distance0 = b2Dot(normal, vIn[0].v) - offset;
        float32 distance1 = b2Dot(normal, vIn[1].v) - offset;

        // If the points are behind the plane
        if (distance0 <= 0.0f) vOut[numOut++] = vIn[0];
        if (distance1 <= 0.0f) vOut[numOut++] = vIn[1];

        // If the points are on different sides of the plane
        if (distance0 * distance1 < 0.0f)
        {
            // Find intersection point of edge and plane
            float32 interp = distance0 / (distance0 - distance1);
            vOut[numOut].v = vIn[0].v + interp * (vIn[1].v - vIn[0].v);

            // VertexA is hitting edgeB.
            vOut[numOut].id.cf.indexA = (uint8) vertexIndexA;
            vOut[numOut].id.cf.indexB = vIn[0].id.cf.indexB;
            vOut[numOut].id.cf.typeA = b2ContactFeature::e_vertex;
            vOut[numOut].id.cf.typeB = b2ContactFeature::e_face;
            ++numOut;
        }

        return numOut;
    */
}

/**
   Determine if two generic shapes overlap.
  */
pub fn b2test_overlap(
        shapea: *const b2Shape,
        indexa: i32,
        shapeb: *const b2Shape,
        indexb: i32,
        xfa:    &b2Transform,
        xfb:    &b2Transform) -> bool {
    
    todo!();
    /*
        b2DistanceInput input;
        input.proxyA.Set(shapeA, indexA);
        input.proxyB.Set(shapeB, indexB);
        input.transformA = xfA;
        input.transformB = xfB;
        input.useRadii = true;

        b2SimplexCache cache;
        cache.count = 0;

        b2DistanceOutput output;

        b2Distance(&output, &cache, &input);

        return output.distance < 10.0f * b2_epsilon;
    */
}
