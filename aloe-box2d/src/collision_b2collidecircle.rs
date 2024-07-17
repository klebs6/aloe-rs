crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2CollideCircle.cpp]

pub fn b2collide_circles(
        manifold: *mut b2Manifold,
        circlea:  *const b2CircleShape,
        xfa:      &b2Transform,
        circleb:  *const b2CircleShape,
        xfb:      &b2Transform)  {
    
    todo!();
    /*
        manifold->pointCount = 0;

        b2Vec2 pA = b2Mul(xfA, circleA->m_p);
        b2Vec2 pB = b2Mul(xfB, circleB->m_p);

        b2Vec2 d = pB - pA;
        float32 distSqr = b2Dot(d, d);
        float32 rA = circleA->m_radius, rB = circleB->m_radius;
        float32 radius = rA + rB;
        if (distSqr > radius * radius)
        {
            return;
        }

        manifold->type = b2Manifold::e_circles;
        manifold->localPoint = circleA->m_p;
        manifold->localNormal.SetZero();
        manifold->pointCount = 1;

        manifold->points[0].localPoint = circleB->m_p;
        manifold->points[0].id.key = 0;
    */
}

pub fn b2collide_polygon_and_circle(
        manifold: *mut b2Manifold,
        polygona: *const b2PolygonShape,
        xfa:      &b2Transform,
        circleb:  *const b2CircleShape,
        xfb:      &b2Transform)  {
    
    todo!();
    /*
        manifold->pointCount = 0;

        // Compute circle position in the frame of the polygon.
        b2Vec2 c = b2Mul(xfB, circleB->m_p);
        b2Vec2 cLocal = b2MulT(xfA, c);

        // Find the min separating edge.
        int32 normalIndex = 0;
        float32 separation = -b2_maxFloat;
        float32 radius = polygonA->m_radius + circleB->m_radius;
        int32 vertexCount = polygonA->m_vertexCount;
        const b2Vec2* vertices = polygonA->m_vertices;
        const b2Vec2* normals = polygonA->m_normals;

        for (int32 i = 0; i < vertexCount; ++i)
        {
            float32 s = b2Dot(normals[i], cLocal - vertices[i]);

            if (s > radius)
            {
                // Early out.
                return;
            }

            if (s > separation)
            {
                separation = s;
                normalIndex = i;
            }
        }

        // Vertices that subtend the incident face.
        int32 vertIndex1 = normalIndex;
        int32 vertIndex2 = vertIndex1 + 1 < vertexCount ? vertIndex1 + 1 : 0;
        b2Vec2 v1 = vertices[vertIndex1];
        b2Vec2 v2 = vertices[vertIndex2];

        // If the center is inside the polygon ...
        if (separation < b2_epsilon)
        {
            manifold->pointCount = 1;
            manifold->type = b2Manifold::e_faceA;
            manifold->localNormal = normals[normalIndex];
            manifold->localPoint = 0.5f * (v1 + v2);
            manifold->points[0].localPoint = circleB->m_p;
            manifold->points[0].id.key = 0;
            return;
        }

        // Compute barycentric coordinates
        float32 u1 = b2Dot(cLocal - v1, v2 - v1);
        float32 u2 = b2Dot(cLocal - v2, v1 - v2);
        if (u1 <= 0.0f)
        {
            if (b2DistanceSquared(cLocal, v1) > radius * radius)
            {
                return;
            }

            manifold->pointCount = 1;
            manifold->type = b2Manifold::e_faceA;
            manifold->localNormal = cLocal - v1;
            manifold->localNormal.Normalize();
            manifold->localPoint = v1;
            manifold->points[0].localPoint = circleB->m_p;
            manifold->points[0].id.key = 0;
        }
        else if (u2 <= 0.0f)
        {
            if (b2DistanceSquared(cLocal, v2) > radius * radius)
            {
                return;
            }

            manifold->pointCount = 1;
            manifold->type = b2Manifold::e_faceA;
            manifold->localNormal = cLocal - v2;
            manifold->localNormal.Normalize();
            manifold->localPoint = v2;
            manifold->points[0].localPoint = circleB->m_p;
            manifold->points[0].id.key = 0;
        }
        else
        {
            b2Vec2 faceCenter = 0.5f * (v1 + v2);

            if (b2Dot (cLocal - faceCenter, normals[vertIndex1]) > radius)
                return;

            manifold->pointCount = 1;
            manifold->type = b2Manifold::e_faceA;
            manifold->localNormal = normals[vertIndex1];
            manifold->localPoint = faceCenter;
            manifold->points[0].localPoint = circleB->m_p;
            manifold->points[0].id.key = 0;
        }
    */
}
