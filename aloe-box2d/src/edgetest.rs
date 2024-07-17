crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/EdgeTest.h]

pub struct EdgeTest {
    base: Test,
}

impl Default for EdgeTest {
    
    fn default() -> Self {
        todo!();
        /*
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                b2Vec2 v1(-10.0f, 0.0f), v2(-7.0f, -2.0f), v3(-4.0f, 0.0f);
                b2Vec2 v4(0.0f, 0.0f), v5(4.0f, 0.0f), v6(7.0f, 2.0f), v7(10.0f, 0.0f);

                b2EdgeShape shape;

                shape.Set(v1, v2);
                shape.m_hasVertex3 = true;
                shape.m_vertex3 = v3;
                ground->CreateFixture(&shape, 0.0f);

                shape.Set(v2, v3);
                shape.m_hasVertex0 = true;
                shape.m_hasVertex3 = true;
                shape.m_vertex0 = v1;
                shape.m_vertex3 = v4;
                ground->CreateFixture(&shape, 0.0f);

                shape.Set(v3, v4);
                shape.m_hasVertex0 = true;
                shape.m_hasVertex3 = true;
                shape.m_vertex0 = v2;
                shape.m_vertex3 = v5;
                ground->CreateFixture(&shape, 0.0f);

                shape.Set(v4, v5);
                shape.m_hasVertex0 = true;
                shape.m_hasVertex3 = true;
                shape.m_vertex0 = v3;
                shape.m_vertex3 = v6;
                ground->CreateFixture(&shape, 0.0f);

                shape.Set(v5, v6);
                shape.m_hasVertex0 = true;
                shape.m_hasVertex3 = true;
                shape.m_vertex0 = v4;
                shape.m_vertex3 = v7;
                ground->CreateFixture(&shape, 0.0f);

                shape.Set(v6, v7);
                shape.m_hasVertex0 = true;
                shape.m_vertex0 = v5;
                ground->CreateFixture(&shape, 0.0f);
            }

            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(-0.5f, 0.6f);
                bd.allowSleep = false;
                b2Body* body = m_world->CreateBody(&bd);

                b2CircleShape shape;
                shape.m_radius = 0.5f;

                body->CreateFixture(&shape, 1.0f);
            }

            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(1.0f, 0.6f);
                bd.allowSleep = false;
                b2Body* body = m_world->CreateBody(&bd);

                b2PolygonShape shape;
                shape.SetAsBox(0.5f, 0.5f);

                body->CreateFixture(&shape, 1.0f);
            
        */
    }
}

impl EdgeTest {
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new EdgeTest;
        */
    }
}
