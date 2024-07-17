crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/DistanceTest.h]

pub struct DistanceTest {
    base:       Test,
    positionb:  b2Vec2,
    angleb:     f32,
    transforma: b2Transform,
    transformb: b2Transform,
    polygona:   b2PolygonShape,
    polygonb:   b2PolygonShape,
}

impl Default for DistanceTest {
    
    fn default() -> Self {
        todo!();
        /*
            {
                m_transformA.SetIdentity();
                m_transformA.p.Set(0.0f, -0.2f);
                m_polygonA.SetAsBox(10.0f, 0.2f);
            }

            {
                m_positionB.Set(12.017401f, 0.13678508f);
                m_angleB = -0.0109265f;
                m_transformB.Set(m_positionB, m_angleB);

                m_polygonB.SetAsBox(2.0f, 0.1f);
            
        */
    }
}

impl DistanceTest {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new DistanceTest;
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            b2DistanceInput input;
            input.proxyA.Set(&m_polygonA, 0);
            input.proxyB.Set(&m_polygonB, 0);
            input.transformA = m_transformA;
            input.transformB = m_transformB;
            input.useRadii = true;
            b2SimplexCache cache;
            cache.count = 0;
            b2DistanceOutput output;
            b2Distance(&output, &cache, &input);

            m_debugDraw.DrawString(5, m_textLine, "distance = %g", output.distance);
            m_textLine += 15;

            m_debugDraw.DrawString(5, m_textLine, "iterations = %d", output.iterations);
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

            b2Vec2 x1 = output.pointA;
            b2Vec2 x2 = output.pointB;

            b2Color c1(1.0f, 0.0f, 0.0f);
            m_debugDraw.DrawPoint(x1, 4.0f, c1);

            b2Color c2(1.0f, 1.0f, 0.0f);
            m_debugDraw.DrawPoint(x2, 4.0f, c2);
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
