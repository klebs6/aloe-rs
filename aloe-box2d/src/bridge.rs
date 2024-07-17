crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/Bridge.h]

pub const BRIDGE_E_COUNT: usize = 30;

pub struct Bridge {
    base:   Test,
    middle: *mut b2Body,
}

impl Default for Bridge {
    
    fn default() -> Self {
        todo!();
        /*


            b2Body* ground = NULL;
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            {
                b2PolygonShape shape;
                shape.SetAsBox(0.5f, 0.125f);

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.density = 20.0f;
                fd.friction = 0.2f;

                b2RevoluteJointDef jd;

                b2Body* prevBody = ground;
                for (int32 i = 0; i < BRIDGE_E_COUNT; ++i)
                {
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(-14.5f + 1.0f * i, 5.0f);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&fd);

                    b2Vec2 anchor(-15.0f + 1.0f * i, 5.0f);
                    jd.Initialize(prevBody, body, anchor);
                    m_world->CreateJoint(&jd);

                    if (i == (BRIDGE_E_COUNT >> 1))
                    {
                        m_middle = body;
                    }
                    prevBody = body;
                }

                b2Vec2 anchor(-15.0f + 1.0f * BRIDGE_E_COUNT, 5.0f);
                jd.Initialize(prevBody, ground, anchor);
                m_world->CreateJoint(&jd);
            }

            for (int32 i = 0; i < 2; ++i)
            {
                b2Vec2 vertices[3];
                vertices[0].Set(-0.5f, 0.0f);
                vertices[1].Set(0.5f, 0.0f);
                vertices[2].Set(0.0f, 1.5f);

                b2PolygonShape shape;
                shape.Set(vertices, 3);

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.density = 1.0f;

                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(-8.0f + 8.0f * i, 12.0f);
                b2Body* body = m_world->CreateBody(&bd);
                body->CreateFixture(&fd);
            }

            for (int32 i = 0; i < 3; ++i)
            {
                b2CircleShape shape;
                shape.m_radius = 0.5f;

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.density = 1.0f;

                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(-6.0f + 6.0f * i, 10.0f);
                b2Body* body = m_world->CreateBody(&bd);
                body->CreateFixture(&fd);
            
        */
    }
}

impl Bridge {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Bridge;
        */
    }
}
