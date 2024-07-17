crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/PolyCollision.h]

pub struct PolyCollision {
    base:       Test,
    polygona:   b2PolygonShape,
    polygonb:   b2PolygonShape,
    transforma: b2Transform,
    transformb: b2Transform,
    positionb:  b2Vec2,
    angleb:     f32,
}

impl Default for PolyCollision {
    
    fn default() -> Self {
        todo!();
        /*


            {
                m_polygonA.SetAsBox(0.2f, 0.4f);
                m_transformA.Set(b2Vec2(0.0f, 0.0f), 0.0f);
            }

            {
                m_polygonB.SetAsBox(0.5f, 0.5f);
                m_positionB.Set(19.345284f, 1.5632932f);
                m_angleB = 1.9160721f;
                m_transformB.Set(m_positionB, m_angleB);
            
        */
    }
}

impl PolyCollision {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new PolyCollision;
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            B2_NOT_USED(settings);

            b2Manifold manifold;
            b2CollidePolygons(&manifold, &m_polygonA, m_transformA, &m_polygonB, m_transformB);

            b2WorldManifold worldManifold;
            worldManifold.Initialize(&manifold, m_transformA, m_polygonA.m_radius, m_transformB, m_polygonB.m_radius);

            m_debugDraw.DrawString(5, m_textLine, "point count = %d", manifold.pointCount);
            m_textLine += 15;

            {
                b2Color color(0.9f, 0.9f, 0.9f);
                b2Vec2 v[b2_maxPolygonVertices];
                for (int32 i = 0; i < m_polygonA.m_vertexCount; ++i)
                {
                    v[i] = b2Mul(m_transformA, m_polygonA.m_vertices[i]);
                }
                m_debugDraw.DrawPolygon(v, m_polygonA.m_vertexCount, color);

                for (int32 i = 0; i < m_polygonB.m_vertexCount; ++i)
                {
                    v[i] = b2Mul(m_transformB, m_polygonB.m_vertices[i]);
                }
                m_debugDraw.DrawPolygon(v, m_polygonB.m_vertexCount, color);
            }

            for (int32 i = 0; i < manifold.pointCount; ++i)
            {
                m_debugDraw.DrawPoint(worldManifold.points[i], 4.0f, b2Color(0.9f, 0.3f, 0.3f));
            }
        */
    }
    
    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'a':
                m_positionB.x -= 0.1f;
                break;

            case 'd':
                m_positionB.x += 0.1f;
                break;

            case 's':
                m_positionB.y -= 0.1f;
                break;

            case 'w':
                m_positionB.y += 0.1f;
                break;

            case 'q':
                m_angleB += 0.1f * b2_pi;
                break;

            case 'e':
                m_angleB -= 0.1f * b2_pi;
                break;
            }

            m_transformB.Set(m_positionB, m_angleB);
        */
    }
}
